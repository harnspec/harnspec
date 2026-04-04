#!/usr/bin/env node
/**
 * Restore original package.json files after publishing.
 * This reverts the workspace:* replacements made by prepare-publish.ts
 * 
 * NOTE: This only restores dependency changes, NOT version changes.
 * - For stable releases: Versions should remain at the new release version
 * - For dev testing: Use `git restore` to discard version bumps too
 * 
 * Usage:
 *   npm run restore-packages
 *   pnpm restore-packages
 *   
 * To restore everything including versions (dev testing):
 *   git restore package.json packages/star/package.json packages/star/star/package.json
 *   (replace 'star' with asterisk glob pattern)
 */

import { existsSync, renameSync, unlinkSync, readdirSync, statSync } from 'fs';
import { join, dirname, relative, resolve } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const ROOT = join(__dirname, '..');

function findBackupFiles(dir: string, files: string[] = []): string[] {
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
      findBackupFiles(fullPath, files);
    } else if (entry.name.endsWith('.backup')) {
      files.push(fullPath);
    }
  }

  return files;
}

function restorePackage(backupPath: string): boolean {
  const targetPath = backupPath.replace(/\.backup$/, '');
  const relPath = relative(ROOT, targetPath);

  console.log(`📦 Restoring ${relPath}...`);

  if (existsSync(targetPath)) {
    unlinkSync(targetPath);
  }
  
  renameSync(backupPath, targetPath);
  console.log(`  ✅ Restored from backup`);

  return true;
}

function main() {
  console.log('🔄 Restoring original package.json files...\n');

  const backups = findBackupFiles(join(ROOT, 'packages'));
  
  let restored = 0;
  for (const backup of backups) {
    if (restorePackage(backup)) {
      restored++;
    }
  }

  // Remove copied README.md from CLI package (it's copied from root during prepare-publish)
  const cliReadme = join(ROOT, 'packages/cli/README.md');
  if (existsSync(cliReadme)) {
    unlinkSync(cliReadme);
    console.log('\n📄 Removed packages/cli/README.md (copied from root)');
  }

  if (restored > 0) {
    console.log(`\n✅ Restored ${restored} package(s)`);
  } else {
    console.log('\n⚠️  No backups found. Nothing to restore.');
  }
}

main();

