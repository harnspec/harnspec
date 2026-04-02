#!/usr/bin/env tsx
/**
 * Verify published npm packages have all required platform binaries
 * 
 * This script checks that all platform-specific packages are published
 * to npm and can be installed. It prevents incomplete releases where
 * only some platforms are available.
 * 
 * Usage:
 *   pnpm verify-npm-publish [version]
 *   pnpm verify-npm-publish 0.2.10
 *   pnpm verify-npm-publish latest  # Check latest published version
 */

import { exec } from 'node:child_process';
import { promisify } from 'node:util';

const execAsync = promisify(exec);

const PLATFORMS = ['darwin-x64', 'darwin-arm64', 'linux-x64', 'windows-x64'];
const PACKAGES = ['cli', 'http'];

interface PackageInfo {
  name: string;
  version: string;
  exists: boolean;
  error?: string;
}

async function checkPackageExists(packageName: string, version: string): Promise<PackageInfo> {
  try {
    const versionArg = version === 'latest' ? '' : `@${version}`;
    const { stdout } = await execAsync(`npm view ${packageName}${versionArg} version 2>&1`);
    const publishedVersion = stdout.trim();

    return {
      name: packageName,
      version: publishedVersion,
      exists: true
    };
  } catch (error) {
    return {
      name: packageName,
      version,
      exists: false,
      error: error instanceof Error ? error.message : String(error)
    };
  }
}

async function verifyPlatformPackages(version: string): Promise<boolean> {
  console.log(`🔍 Verifying platform packages for version: ${version}\n`);

  const checks: Promise<PackageInfo>[] = [];

  for (const pkg of PACKAGES) {
    for (const platform of PLATFORMS) {
      const packageName = `@harnspec/${pkg}-${platform}`;
      checks.push(checkPackageExists(packageName, version));
    }
  }

  const results = await Promise.all(checks);

  // Group by package type
  const cliResults = results.filter(r => r.name.includes('cli'));
  const httpResults = results.filter(r => r.name.includes('http'));

  let allGood = true;

  console.log('📦 CLI Platform Packages:');
  for (const result of cliResults) {
    if (result.exists) {
      console.log(`  ✅ ${result.name}@${result.version}`);
    } else {
      console.log(`  ❌ ${result.name} - NOT FOUND`);
      if (result.error) {
        console.log(`     Error: ${result.error}`);
      }
      allGood = false;
    }
  }

  console.log('\n📦 HTTP Platform Packages:');
  for (const result of httpResults) {
    if (result.exists) {
      console.log(`  ✅ ${result.name}@${result.version}`);
    } else {
      console.log(`  ❌ ${result.name} - NOT FOUND`);
      if (result.error) {
        console.log(`     Error: ${result.error}`);
      }
      allGood = false;
    }
  }

  return allGood;
}

async function verifyMainPackages(version: string): Promise<boolean> {
  console.log(`\n🔍 Verifying main packages for version: ${version}\n`);

  const mainPackages = ['harnspec', '@harnspec/http'];
  const checks = mainPackages.map(pkg => checkPackageExists(pkg, version));
  const results = await Promise.all(checks);

  let allGood = true;

  console.log('📦 Main Packages:');
  for (const result of results) {
    if (result.exists) {
      console.log(`  ✅ ${result.name}@${result.version}`);
    } else {
      console.log(`  ❌ ${result.name} - NOT FOUND`);
      if (result.error) {
        console.log(`     Error: ${result.error}`);
      }
      allGood = false;
    }
  }

  return allGood;
}

async function checkOptionalDependencies(packageName: string, version: string): Promise<void> {
  console.log(`\n🔍 Checking optionalDependencies in ${packageName}@${version}...\n`);

  try {
    const versionArg = version === 'latest' ? '' : `@${version}`;
    const { stdout } = await execAsync(`npm view ${packageName}${versionArg} optionalDependencies --json`);

    if (!stdout.trim()) {
      console.log(`⚠️  ${packageName} has no optionalDependencies`);
      console.log(`   This means users won't automatically get platform binaries!`);
      return;
    }

    const optDeps = JSON.parse(stdout);

    console.log(`📋 Optional Dependencies in ${packageName}:`);
    for (const [dep, depVersion] of Object.entries(optDeps)) {
      console.log(`  - ${dep}@${depVersion}`);
    }

    // Check if all platforms are included
    const expectedPrefixes = packageName === 'harnspec' ?
      PLATFORMS.map(p => `harnspec-${p}`) :
      PLATFORMS.map(p => `@harnspec/http-${p}`);

    const missing = expectedPrefixes.filter(prefix =>
      !Object.keys(optDeps).some(dep => dep.includes(prefix))
    );

    if (missing.length > 0) {
      console.log(`\n⚠️  Missing platform dependencies:`);
      for (const m of missing) {
        console.log(`  - ${m}`);
      }
    } else {
      console.log(`\n✅ All platform dependencies are included`);
    }
  } catch (error) {
    console.error(`❌ Failed to check optionalDependencies:`, error);
  }
}

async function main() {
  const args = process.argv.slice(2);
  const version = args[0] || 'latest';

  console.log('═'.repeat(60));
  console.log('  HarnSpec npm Package Verification');
  console.log('═'.repeat(60));
  console.log('');

  try {
    const platformsOk = await verifyPlatformPackages(version);
    const mainOk = await verifyMainPackages(version);

    // Check optionalDependencies configuration
    await checkOptionalDependencies('harnspec', version);
    await checkOptionalDependencies('@harnspec/http', version);

    console.log('\n' + '═'.repeat(60));

    if (platformsOk && mainOk) {
      console.log('✅ SUCCESS: All packages are published correctly!');
      console.log('');
      console.log('Users can install with:');
      if (version === 'latest') {
        console.log('  npm install -g harnspec');
        console.log('  npm install -g @harnspec/http');
      } else {
        console.log(`  npm install -g harnspec@${version}`);
        console.log(`  npm install -g @harnspec/http@${version}`);
      }
      console.log('═'.repeat(60));
      process.exit(0);
    } else {
      console.log('❌ FAILURE: Some packages are missing!');
      console.log('');
      console.log('This will cause installation failures for users on those platforms.');
      console.log('Please ensure all platform packages are published before the main package.');
      console.log('═'.repeat(60));
      process.exit(1);
    }
  } catch (error) {
    console.error('\n❌ Error during verification:', error);
    process.exit(1);
  }
}

main().catch(error => {
  console.error('Fatal error:', error);
  process.exit(1);
});
