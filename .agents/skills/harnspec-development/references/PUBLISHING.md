# Publishing Releases

**Publish CLI and UI packages to npm with synchronized versions:**

## Architecture Overview

With the Rust migration complete (spec 181):

- **CLI** (`harnspec`) - JavaScript wrapper that invokes Rust binary
- **MCP** (`@harnspec/mcp`) - JavaScript wrapper that invokes Rust MCP binary  
- **UI** (`@harnspec/ui`) - Next.js app with inlined utilities
- **Platform packages** - Rust binaries for each platform (published separately)

The Rust binaries are distributed via optional dependencies (e.g., `harnspec-darwin-arm64`).

## Publishing Dev Versions

For testing and preview releases, publish dev versions via CI that don't affect the stable `latest` tag:

### Dev Release via CI

Manually trigger the GitHub Actions workflow to publish dev versions for all platforms:

```bash
# Option 1: Manual trigger via GitHub UI
# Go to Actions → Publish to npm → Run workflow (set dev=true)

# Option 2: Manual trigger via CLI
gh workflow run publish.yml --field dev=true

# Dry run (build/validate only)
gh workflow run publish.yml --field dev=true --field dry_run=true
```

The `.github/workflows/publish.yml` workflow (with `dev=true`) will automatically:

- **Auto-bump version** to a prerelease (e.g., `0.2.4-dev.123456789`)
- Build Rust binaries for current platform
- Sync all package versions (including Rust platform packages)
- **Publish platform packages first** (e.g., `harnspec-darwin-arm64`, `@harnspec/mcp-darwin-arm64`)
- Publish main packages (CLI, MCP, UI) with the `dev` tag
- Keep the `latest` tag unchanged for stable users

**Note**: Versions are auto-generated from the current base version + the workflow run id, so you don't need to manually update package.json files for dev releases.

### Testing Dev Versions

Users install dev versions with:

```bash
npm install -g harnspec@dev
npm install @harnspec/mcp@dev
npm install @harnspec/ui@dev
```

## Release Checklist

⚠️ **CRITICAL**: All steps must be completed in order. Do NOT skip steps.

1. **Version bump**: Update version in all package.json files (root, cli, ui, mcp) for consistency
2. **Update CHANGELOG.md**: Add release notes with date and version
3. **Type check**: Run `pnpm typecheck` to catch type errors (REQUIRED before release)
4. **Test**: Run `pnpm test:run` to ensure tests pass
5. **Build**: Run `pnpm build` to build all packages
6. **Validate**: Run `node bin/harnspec.mjs validate --warnings-only` and `cd docs-site && npm run build` to ensure everything works
7. **Commit & Tag**:

   ```bash
   git add -A && git commit -m "feat: release version X.Y.Z with [brief description]"
   git tag -a vX.Y.Z -m "Release vX.Y.Z: [title]"
   git push origin vX.Y.Z
   ```

8. **Prepare for publish**: Run `pnpm prepare-publish` to replace `workspace:*` with actual versions
   - ⚠️ **CRITICAL**: This step prevents `workspace:*` from leaking into npm packages
   - Creates backups of original package.json files
   - Replaces all `workspace:*` dependencies with actual versions
9. **Publish to npm**: For each package (cli, mcp, ui-components, http-server, ui) **in order**:

   ```bash
   cd packages/cli && npm publish --access public
   cd ../mcp && npm publish --access public
   cd ../ui-components && npm publish --access public
   cd ../http-server && npm publish --access public
   cd ../ui && npm publish --access public
   ```

   - ⚠️ **IMPORTANT**: Publish in dependency order:
     - `ui-components` has no workspace dependencies
     - `http-server` has no workspace dependencies  
     - `ui` depends on both `ui-components` and `http-server`
   - If a package version already exists (403 error), that's OK - skip it
   - Tag packages as latest if needed: `npm dist-tag add @harnspec/ui@X.Y.Z latest`
10. **Restore packages**: Run `pnpm restore-packages` to restore original package.json files with `workspace:*`
11. **Create GitHub Release** (REQUIRED - DO NOT SKIP):

   ```bash
   # Create release notes file with formatted content
   cat > /tmp/release-notes.md << 'EOF'
   ## Release vX.Y.Z - YYYY-MM-DD
   
   ### 🎉 Major Changes
   [List major features/changes]
   
   ### 🐛 Bug Fixes
   [List bug fixes]
   
   ### ✨ Enhancements
   [List enhancements]
   
   ### 📦 Published Packages
   - `harnspec@X.Y.Z`
   - `@harnspec/mcp@X.Y.Z`
   - `@harnspec/ui-components@X.Y.Z`
   - `@harnspec/http-server@X.Y.Z`
   - `@harnspec/ui@X.Y.Z`
   
   ### 🔗 Links
   - [npm: harnspec](https://www.npmjs.com/package/harnspec)
   - [Documentation](https://harnspec.github.io)
   - [Full Changelog](https://github.com/harnspec/harnspec/blob/main/CHANGELOG.md)
   EOF
   
   # Create the release
   gh release create vX.Y.Z --title "Release vX.Y.Z: [Title]" --notes-file /tmp/release-notes.md
   ```

- ⚠️ **This step is MANDATORY** - GitHub releases are the official release announcement
- Users discover new versions through GitHub releases
- Release notes provide context that CHANGELOG.md alone doesn't

1. **Verify**:

- `npm view harnspec version` to confirm CLI publication
- `npm view @harnspec/mcp version` to confirm MCP publication
- `npm view @harnspec/ui-components version` to confirm UI Components publication
- `npm view @harnspec/http-server version` to confirm HTTP Server publication
- `npm view @harnspec/ui version` to confirm UI publication
- `npm view harnspec dependencies` to ensure no `workspace:*` dependencies leaked
- `npm view @harnspec/ui-components dependencies` to ensure no `workspace:*` dependencies leaked
- `npm view @harnspec/http-server dependencies` to ensure no `workspace:*` dependencies leaked
- `npm view @harnspec/ui dependencies` to ensure no `workspace:*` dependencies leaked
- Test installation: `npm install -g harnspec@latest` in a clean environment
- Test UI installation: `npm install -g @harnspec/ui@latest` in a clean environment
- **Check GitHub release page**: <https://github.com/harnspec/harnspec/releases>
- Verify release appears with correct title and notes

## Critical - Workspace Dependencies

- The CLI package is now a thin wrapper for Rust binaries - no need for bundling
- If you see `workspace:*` in published dependencies, the package is broken and must be republished

## Package Publication Notes

**Important**:

- `@harnspec/http-server` IS published to npm as a public scoped package (required dependency for `@harnspec/ui`, provides the API backend)
- `@harnspec/ui` IS published to npm as a public scoped package
- Packages `harnspec` (CLI), `@harnspec/mcp` (MCP), `@harnspec/ui-components`, `@harnspec/http-server`, and `@harnspec/ui` are published automatically via GitHub Actions when a release is created
- Platform-specific binary packages (e.g., `harnspec-darwin-arm64`) are published separately via the rust-binaries workflow
