#!/usr/bin/env node
/**
 * Prepare packages for npm publish by replacing workspace:* dependencies with actual versions.
 * Run this script before publishing to ensure no workspace protocol leaks into npm.
 * 
 * Usage:
 *   npm run prepare-publish
 *   pnpm prepare-publish
 * 
 * This script:
 * 1. Finds all workspace:* dependencies in packages
 * 2. Resolves actual versions from local package.json files
 * 3. Creates temporary package.json files with resolved versions
 * 4. Copies root README.md to CLI package for npm display
 * 5. After publish, restore original package.json files
 */

import { readFileSync, writeFileSync, existsSync, copyFileSync, readdirSync, statSync } from 'fs';
import { join, dirname, relative, resolve } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const ROOT = join(__dirname, '..');

interface PackageJson {
  name: string;
  version: string;
  private?: boolean;
  dependencies?: Record<string, string>;
  devDependencies?: Record<string, string>;
  peerDependencies?: Record<string, string>;
  optionalDependencies?: Record<string, string>;
}

function findPackageJsonFiles(dir: string, files: string[] = []): string[] {
  const entries = readdirSync(dir, { withFileTypes: true });

  for (const entry of entries) {
    const fullPath = join(dir, entry.name);

    if (
      entry.name === 'node_modules' ||
      entry.name === 'dist' ||
      entry.name === 'target' ||
      entry.name === '.turbo' ||
      entry.name === '.git'
    ) {
      continue;
    }

    if (entry.isDirectory()) {
      findPackageJsonFiles(fullPath, files);
    } else if (entry.name === 'package.json') {
      files.push(fullPath);
    }
  }

  return files;
}

function readPackageJson(pkgPath: string): PackageJson {
  return JSON.parse(readFileSync(pkgPath, 'utf-8'));
}

function writePackageJson(pkgPath: string, pkg: PackageJson): void {
  writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');
}

const pkgMap: Record<string, string> = {};

function initPackageMap() {
  const packagesDir = join(ROOT, 'packages');
  const files = findPackageJsonFiles(packagesDir);
  
  for (const file of files) {
    const pkg = readPackageJson(file);
    if (pkg.name) {
      pkgMap[pkg.name] = relative(ROOT, file);
    }
  }
}

function resolveWorkspaceVersion(depName: string): string | null {
  const pkgPath = pkgMap[depName];
  if (!pkgPath) {
    console.warn(`⚠️  Unknown workspace package: ${depName}`);
    return null;
  }

  const fullPath = join(ROOT, pkgPath);
  if (!existsSync(fullPath)) {
    console.warn(`⚠️  Package not found: ${fullPath}`);
    return null;
  }

  const pkg = readPackageJson(fullPath);
  return pkg.version;
}

function replaceWorkspaceDeps(deps: Record<string, string> | undefined, depType: string): boolean {
  if (!deps) return false;

  let changed = false;
  for (const [name, version] of Object.entries(deps)) {
    if (version.startsWith('workspace:')) {
      const resolvedVersion = resolveWorkspaceVersion(name);
      if (resolvedVersion) {
        deps[name] = resolvedVersion;
        console.log(`  ✓ ${depType}.${name}: workspace:* → ${resolvedVersion}`);
        changed = true;
      }
    }
  }
  return changed;
}

function processPackage(pkgPath: string): boolean {
  const pkg = readPackageJson(pkgPath);
  
  // Skip private packages unless they are the root ones we want to process
  // Actually, we should process all non-private ones
  if (pkg.private) {
    console.log(`\n⏭️  Skipping private package ${pkg.name}`);
    return false;
  }

  console.log(`\n📦 Processing ${pkg.name}...`);

  let changed = false;
  changed = replaceWorkspaceDeps(pkg.dependencies, 'dependencies') || changed;
  changed = replaceWorkspaceDeps(pkg.devDependencies, 'devDependencies') || changed;
  changed = replaceWorkspaceDeps(pkg.peerDependencies, 'peerDependencies') || changed;
  changed = replaceWorkspaceDeps(pkg.optionalDependencies, 'optionalDependencies') || changed;

  if (changed) {
    // Create backup
    const backupPath = pkgPath + '.backup';
    writeFileSync(backupPath, readFileSync(pkgPath, 'utf-8'));
    console.log(`  💾 Backup saved to ${relative(ROOT, pkgPath)}.backup`);

    // Write updated package.json
    writePackageJson(pkgPath, pkg);
    console.log(`  ✅ Updated ${relative(ROOT, pkgPath)}`);
    return true;
  } else {
    console.log(`  ⏭️  No workspace:* dependencies found`);
    return false;
  }
}

function main() {
  console.log('🚀 Preparing packages for npm publish...\n');
  console.log('This will replace workspace:* with actual versions.\n');

  initPackageMap();

  const packagesPath = join(ROOT, 'packages');
  const allPackageFiles = findPackageJsonFiles(packagesPath);
  
  const modified: string[] = [];
  for (const pkgFile of allPackageFiles) {
    if (processPackage(pkgFile)) {
      modified.push(relative(ROOT, pkgFile));
    }
  }

  // Copy root README.md to CLI package for npm display
  const rootReadme = join(ROOT, 'README.md');
  const cliReadme = join(ROOT, 'packages/cli/README.md');
  if (existsSync(rootReadme)) {
    copyFileSync(rootReadme, cliReadme);
    console.log('\n📄 Copied root README.md to packages/cli/README.md');
    modified.push('packages/cli/README.md');
  }

  if (modified.length > 0) {
    console.log('\n✅ Preparation complete!');
    console.log('\nModified packages:');
    modified.forEach(pkg => console.log(`  - ${pkg}`));
    console.log('\n⚠️  IMPORTANT: After publishing, restore original files:');
    console.log('   pnpm restore-packages');
    console.log('   OR manually: mv package.json.backup package.json');
  } else {
    console.log('\n✅ No workspace:* dependencies found. Ready to publish!');
  }
}

main();

