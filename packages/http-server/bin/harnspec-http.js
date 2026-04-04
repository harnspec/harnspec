#!/usr/bin/env node
/**
 * HarnSpec HTTP Server Binary Wrapper
 * 
 * This script detects the current platform and architecture,
 * then spawns the appropriate Rust HTTP server binary.
 * 
 * The wrapper looks for binaries in the following locations:
 * 1. Platform-specific npm package (@harnspec/http-darwin-x64, etc.)
 * 2. Local binaries directory (for development)
 * 3. Rust target directory (for local development)
 */

import { spawn } from 'child_process';
import { createRequire } from 'module';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { accessSync } from 'fs';

const require = createRequire(import.meta.url);
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Debug mode - enable with HARNSPEC_DEBUG=1
const DEBUG = process.env.HARNSPEC_DEBUG === '1';
const DEV_MODE = process.env.HARNSPEC_DEV_MODE === '1';
const debug = (...args) => DEBUG && console.error('[harnspec-http debug]', ...args);

// Platform detection mapping
const PLATFORM_MAP = {
  darwin: { x64: 'darwin-x64', arm64: 'darwin-arm64' },
  linux: { x64: 'linux-x64', arm64: 'linux-arm64' },
  win32: { x64: 'windows-x64' }
};

function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;

  debug('Platform detection:', { platform, arch });

  const platformKey = PLATFORM_MAP[platform]?.[arch];
  if (!platformKey) {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    console.error('Supported: macOS (x64/arm64), Linux (x64/arm64), Windows (x64)');
    process.exit(1);
  }

  const isWindows = platform === 'win32';
  const binaryName = isWindows ? 'harnspec-http.exe' : 'harnspec-http';
  const packageName = `@harnspec/http-${platformKey}`;

  debug('Binary info:', { platformKey, binaryName, packageName });

  // In dev mode, prefer rust target directory first for faster iteration
  if (DEV_MODE) {
    // Try rust/target/debug directory first (faster builds)
    try {
      const rustDebugPath = join(__dirname, '..', '..', '..', 'rust', 'target', 'debug', binaryName);
      debug('Trying rust debug binary:', rustDebugPath);
      accessSync(rustDebugPath);
      debug('Found rust debug binary:', rustDebugPath);
      return rustDebugPath;
    } catch (e) {
      debug('Rust debug binary not found:', e.message);
    }

    // Try rust/target/release directory
    try {
      const rustReleasePath = join(__dirname, '..', '..', '..', 'rust', 'target', 'release', binaryName);
      debug('Trying rust release binary:', rustReleasePath);
      accessSync(rustReleasePath);
      debug('Found rust release binary:', rustReleasePath);
      return rustReleasePath;
    } catch (e) {
      debug('Rust release binary not found:', e.message);
    }
  }

  // Try to resolve platform package
  try {
    const resolvedPath = require.resolve(`${packageName}/${binaryName}`);
    debug('Found platform package binary:', resolvedPath);
    return resolvedPath;
  } catch (e) {
    debug('Platform package not found:', packageName, '-', e.message);
  }

  // Try local binaries directory (for development/testing)
  try {
    const localPath = join(__dirname, '..', 'binaries', platformKey, binaryName);
    debug('Trying local binary:', localPath);
    accessSync(localPath);
    debug('Found local binary:', localPath);
    return localPath;
  } catch (e) {
    debug('Local binary not found:', e.message);
  }

  // Try rust/target/release directory (fallback for local development)
  try {
    const rustTargetPath = join(__dirname, '..', '..', '..', 'rust', 'target', 'release', binaryName);
    debug('Trying rust target binary:', rustTargetPath);
    accessSync(rustTargetPath);
    debug('Found rust target binary:', rustTargetPath);
    return rustTargetPath;
  } catch (e) {
    debug('Rust target binary not found:', e.message);
  }

  console.error(`Binary not found for ${platform}-${arch}`);
  console.error(`Expected package: ${packageName}`);
  console.error('');
  console.error('To install:');
  console.error('  npm install @harnspec/http-server');
  console.error('');
  process.exit(1);
}

// Execute binary
const binaryPath = getBinaryPath();
const args = process.argv.slice(2);

debug('Spawning binary:', binaryPath);
debug('Arguments:', args);

const child = spawn(binaryPath, args, {
  stdio: 'inherit',
  windowsHide: true,
});

child.on('exit', (code) => {
  debug('Binary exited with code:', code);
  process.exit(code ?? 1);
});

child.on('error', (err) => {
  console.error('Failed to start harnspec-http:', err.message);
  debug('Spawn error:', err);
  process.exit(1);
});
