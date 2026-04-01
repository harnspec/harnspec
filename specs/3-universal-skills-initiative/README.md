---
status: in-progress
created: 2026-02-03
priority: high
tags:
- agent-skills
- umbrella
- cross-platform
- skills
created_at: 2026-02-03T07:54:30.806474499Z
updated_at: 2026-02-03T07:54:30.806474499Z
---

# Universal Agent Skills Initiative

## Overview

Umbrella spec for the Agent Skills ecosystem initiative. This groups all work related to making harnspec skills universally compatible, easier to install, and properly maintained.

### Strategic Goals

1. **Universal Compatibility**: Skills work across all mainstream AI coding tools
2. **Public Repository**: Host skills in dedicated `harnspec/skills` repo
3. **Smooth Installation**: `harnspec init` correctly references installed skill paths
4. **Community Contribution**: Enable external contributions to skills
5. **Brand Alignment**: Follow the transition to the `harnspec` brand

### Child Specs

| 222 - Cross-Tool Compatibility | Detection, installation, platform support | planned |
| 282 - AGENTS.md Path References | Template substitution for skill paths | planned |
| [Spec 2 - Skills Reorg](../2-reback-skills/README.md) | Distribute @harnspec/skills registry | in-progress |
| [Spec 4 - Integrated Skills](../4-integrated-skills-management/README.md) | Integrated skills management via monorepo | planned |

## Design

### Architecture

```
codervisor/skills (new repo)
├── skills/
│   └── harnspec/
│       ├── SKILL.md
│       └── references/
├── .github/
│   └── workflows/
│       └── validate.yml    # skills-ref validate
└── README.md

harnspec/harnspec (this repo)
├── packages/cli/
│   └── templates/          # skills installed from harnspec/skills
└── .github/skills/
    └── harnspec/           # dev copy, synced from harnspec/skills
```

### Benefits

- **Separation of Concerns**: Skills maintained independently from CLI
- **Versioning**: Skills can be versioned separately
- **Community**: Lower barrier to contribute skills
- **Distribution**: CLI fetches skills from published repo

## Plan

- [ ] Complete 222: Cross-tool compatibility strategy
- [ ] Complete 282: AGENTS.md skill path references  
- [/] Complete [Spec 2](../2-reback-skills/README.md): Rename and skills repository migration
- [ ] Update documentation for new skills workflow
- [ ] Announce universal skills support

## Test

- [ ] Skills work in Claude, Cursor, Copilot, and CLI-based tools
- [ ] `harnspec init` generates correct AGENTS.md paths
- [ ] harnspec-skills repo contains validated skills
- [ ] Skills can be installed from public repo

## Notes

This is an **umbrella spec** - it tracks overall progress but delegates implementation to child specs.