# Agent Guide

## Goal

- Maintain `gpui-lucide` safely: generated icon enum, GPUI icon component, and playground app.

## Required Workflow

1. Use Context7 MCP for library/API docs, setup/config, and codegen guidance.
2. Keep changes minimal and scoped to the requested task.
3. For each feature/fix, add or update tests.
4. Validate before handoff:
   - `cargo check`
   - `cargo test`

## Code Rules

- Prefer `rg` for search.
- If changing generated icon behavior (`build.rs`, `icons/`), ensure tests cover:
  - `IconName::count()`
  - `IconName::all()`
  - `IconName::name()` / `IconName::path()` mapping
