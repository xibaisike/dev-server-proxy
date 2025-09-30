
<!--
Sync Impact Report
Version change: (none) → 1.0.0
List of modified principles: All (template → concrete)
Added sections: All
Removed sections: None
Templates requiring updates: ✅ plan-template.md, ✅ spec-template.md, ✅ tasks-template.md
Follow-up TODOs: TODO(RATIFICATION_DATE): Please specify the original ratification date.
-->

# dev-server-proxy Constitution

## Core Principles


### I. Monorepo Structure & Modularity
All code MUST be organized as a monorepo with at least two sub-packages (`common`, `app`).
Each package MUST be independently testable and reusable. Directory structure MAY include: `utils`, `services`, `models`, `routes`, `controllers`, `constants`, `labels`, `locales`.
Rationale: Ensures maintainability, scalability, and clear separation of concerns.

### II. Configuration over Magic Strings
All magic strings and environment-specific values MUST be managed via configuration files, not hardcoded in code.
Rationale: Reduces errors, improves maintainability, and enables environment flexibility.

### III. Test-Driven Development (TDD)
All features MUST be developed using TDD: write tests first, ensure they fail, then implement code to pass tests. Jest MUST be used for unit testing. Red-Green-Refactor cycle is mandatory.
Rationale: Guarantees test coverage and code quality.

### IV. Smoke and Integration Testing
All code changes MUST pass smoke tests and integration tests before merging. Automated tests MUST validate contract and inter-package compatibility.
Rationale: Prevents regressions and ensures system stability.

### V. Preferred Tooling
TypeScript (ESM), Node.js, pnpm, and tsx MUST be used. esbuild is the preferred build tool. Jest is the preferred test runner. Deviation requires explicit justification.
Rationale: Ensures consistency, modern standards, and efficient workflows.


## Technology & Architecture Constraints

- All code MUST use TypeScript with ESM modules.
- Node.js is the required runtime.
- pnpm is the required package manager.
- esbuild is the preferred build tool; alternatives require justification.
- Directory structure MUST reflect modularity and separation of concerns.
- All configuration, secrets, and environment variables MUST be externalized.

Rationale: Enforces a modern, maintainable, and scalable architecture.


## Development Workflow & Quality Gates

- All code MUST be developed using TDD.
- All features and bugfixes MUST include corresponding Jest tests.
- All code changes MUST pass smoke and integration tests before merge.
- Code reviews MUST verify adherence to this constitution.
- All magic strings MUST be managed via configuration.

Rationale: Ensures code quality, maintainability, and compliance with project standards.

## Governance

This constitution supersedes all other engineering practices for this project. Amendments require documentation, team approval, and a migration plan for any breaking changes. All PRs and reviews MUST verify compliance with these principles. Versioning follows semantic versioning: MAJOR for breaking/removal, MINOR for new/expanded principles, PATCH for clarifications. Compliance reviews are required at each release.

TODO(RATIFICATION_DATE): Please specify the original ratification date.

**Version**: 1.0.0 | **Ratified**: TODO(RATIFICATION_DATE) | **Last Amended**: 2025-09-30
<!-- Version: 1.0.0 | Ratified: TODO(RATIFICATION_DATE) | Last Amended: 2025-09-30 -->