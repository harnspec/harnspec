#!/usr/bin/env node
import { createRequire } from 'module';
const require = createRequire(import.meta.url);

/**
 * Delegate to the actual HarnSpec CLI in @harnspec/cli.
 * 
 * This wrapper exists to allow users to run `npm install -g harnspec`
 * instead of the scoped `@harnspec/cli`.
 */
try {
  const cliPath = require.resolve('@harnspec/cli/bin/harnspec.js');
  await import(cliPath);
} catch (err) {
  console.error('Error: Failed to find @harnspec/cli. Make sure it is installed.');
  console.error('If you installed this package globally, try:');
  console.error('  npm install -g @harnspec/cli');
  console.debug(err);
  process.exit(1);
}
