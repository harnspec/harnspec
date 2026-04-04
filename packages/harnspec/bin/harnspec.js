#!/usr/bin/env node
import { createRequire } from 'module';
import { pathToFileURL } from 'node:url';

const require = createRequire(import.meta.url);

/**
 * Delegate to the actual HarnSpec CLI in @harnspec/cli.
 * 
 * This wrapper exists to allow users to run `npm install -g harnspec`
 * instead of the scoped `@harnspec/cli`.
 */
try {
  const cliPath = require.resolve('@harnspec/cli/bin/harnspec.js');
  const cliUrl = pathToFileURL(cliPath).href;
  await import(cliUrl);
} catch (err) {
  if (err.code === 'MODULE_NOT_FOUND') {
    console.error('Error: Failed to find @harnspec/cli. Make sure it is installed.');
    console.error('If you installed this package globally, try:');
    console.error('  npm install -g @harnspec/cli');
  } else {
    console.error('Error: Failed to load @harnspec/cli.');
    console.error(err);
  }
  process.exit(1);
}
