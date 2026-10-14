# Notice

This crate is a standalone copy of `ext/fs` from
[denoland/deno](https://github.com/denoland/deno) at tag `v2.9.7`
(`deno_fs` 0.169.0), MIT licensed. See `LICENSE.md`.

Changes from upstream:

- Workspace dependency versions are pinned explicitly, because the crate is
  built outside the Deno monorepo.
- `rand` is pinned to `=0.8.6` (upstream: `=0.8.5`). The newer release fixes
  an unsoundness with custom loggers that use `rand::rng()`. This is the only
  functional change.
