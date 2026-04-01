#!/usr/bin/env node

/**
 * @harnspec/skills CLI
 * 
 * Injects SDD skills into the current project.
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Root of the @harnspec/skills package
const packageRoot = path.join(__dirname, '..');
const skillsSource = path.join(packageRoot, '.agents', 'skills', 'harnspec');

const targetProjectDir = process.cwd();
const targetSkillsDir = path.join(targetProjectDir, '.agents', 'skills', 'harnspec');

function copyFolderSync(from, to) {
    if (!fs.existsSync(to)) {
        fs.mkdirSync(to, { recursive: true });
    }
    fs.readdirSync(from).forEach(element => {
        const fromPath = path.join(from, element);
        const toPath = path.join(to, element);
        if (fs.lstatSync(fromPath).isFile()) {
            fs.copyFileSync(fromPath, toPath);
        } else {
            copyFolderSync(fromPath, toPath);
        }
    });
}

function run() {
    // Check if the command was called with --help
    if (process.argv.includes('--help') || process.argv.includes('-h')) {
        console.log('Usage: npx @harnspec/skills [target_dir]');
        console.log('\nOptions:');
        console.log('  --help, -h    Show this help message');
        console.log('\nDefault target_dir is ".agents/skills/harnspec" relative to current working directory.');
        process.exit(0);
    }

    try {
        if (!fs.existsSync(skillsSource)) {
            console.error('Error: Official skills source not found in package.');
            process.exit(1);
        }

        console.log(`Injecting SDD skills to ${targetSkillsDir}...`);
        
        // Ensure parent directory exists
        const parentDir = path.dirname(targetSkillsDir);
        if (!fs.existsSync(parentDir)) {
            fs.mkdirSync(parentDir, { recursive: true });
        }

        // Copy skills (recursive, always replace)
        copyFolderSync(skillsSource, targetSkillsDir);

        console.log('✓ Officially injected HarnSpec SDD skills.');
    } catch (error) {
        console.error('Failed to inject skills:', error.message);
        process.exit(1);
    }
}

run();
