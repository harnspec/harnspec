# HarnSpec Commands Reference

Quick reference for all `harnspec` CLI commands used in the SDD process.

## Spec Management

```bash
# General
harnspec board                # View current project status
harnspec list                 # List all specs
harnspec search "query"       # Search within specs

# Creation & Inspection
harnspec create "spec-name"   # Create a new spec
harnspec view <spec-id>       # View detailed spec content
harnspec tokens <spec-id>     # Check token count for context economy

# Modification
harnspec update <spec-id> --status in-progress  # Update status
harnspec update <spec-id> --priority high       # Update priority
harnspec update <spec-id> --tag core            # Add/update tags
```

## Proposal Mode

```bash
# Interactive proposal — from vague idea to structured specs
harnspec proposal                        # Launch interactive mode
harnspec proposal "your idea"            # Start with an initial idea
harnspec proposal --file proposal.md     # From a written document
harnspec proposal --non-interactive --file doc.md  # For AI agents
harnspec proposal --priority high --tags "feature"  # With metadata
```

## Relationships

```bash
# Hierarchy
harnspec rel add <child> --parent <parent>    # Add child to parent
harnspec children <parent-id>                 # List all children of a parent

# Technical Dependencies
harnspec rel add <spec> --depends-on <other>  # Add a technical blocker
harnspec rel rm <spec> --depends-on <other>   # Remove dependency
harnspec deps <spec-id>                       # View dependency graph
```

## Validation & Stats

```bash
harnspec validate             # Verify quality of all specs
harnspec stats                # Get overall project health metrics
harnspec stats --detailed     # More detailed stats breakdown
```

## Utilities

```bash
harnspec check                # Check project configuration
harnspec open <spec-id>       # Open spec file in default editor
harnspec backfill <ids...>    # Backfill missing metadata for specific specs
```

## Tooling & Environment

```bash
harnspec init                 # Initialize HarnSpec in current directory
harnspec init --yes           # Auto-approve all defaults (including Skills)
harnspec ui                   # Start the HarnSpec UI server
harnspec ui --port 3000       # Start UI on specific port
```

## Output Control

Most commands support output formatting:
```bash
harnspec <command> -o text     # Default human-readable text
harnspec <command> -o json     # Machine-readable JSON
```

## Pro Tips

- **Use IDs**: You can reference specs by their numerical ID (e.g., `001`) or partial filename.
- **Combine update flags**: `harnspec update <spec> --status in-progress --priority critical`.
- **Search early**: Always search for existing work before running `create` to avoid duplication.
- **Validate often**: Use `harnspec validate` to ensure your documentation remains high-quality as it grows.
