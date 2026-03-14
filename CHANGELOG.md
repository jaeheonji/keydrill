# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [1.0.0] - 2026-03-14

### Features

- Validate external layout files, error on missing keys or unknown builtin ([03fd9e7](https://github.com/jaeheonji/keydrill/commit/03fd9e7e353707f8f07b61040b23689a9f3df3e2))
- Add builtin Colemak-DH layout and consolidate levels ([8a5a27e](https://github.com/jaeheonji/keydrill/commit/8a5a27e95a6d1361424018c3f5b1f1ed9e4b2eee))
- Add configurable color cycling effect for keyboard animation ([f2124a3](https://github.com/jaeheonji/keydrill/commit/f2124a395a770e9f9da3b123dcc878e3633bc1c4))
- Remove next key highlight from keyboard UI ([a7049cf](https://github.com/jaeheonji/keydrill/commit/a7049cfa82fd02618c9d5d727f13843f4e4aff02))
- Rework typing screen layout with enter-to-submit ([5c0b856](https://github.com/jaeheonji/keydrill/commit/5c0b8563e3917138ab0b60a2e4ef07c4f78fc20e))
- Restructure theme with nested word/keyboard sections and style help text ([c047677](https://github.com/jaeheonji/keydrill/commit/c04767794902a66a23b86ed6003c49705828094e))
- Rework word pool, theme, and remove keyboard effects from UI ([a87b284](https://github.com/jaeheonji/keydrill/commit/a87b2847f9a361020bb6ad052d5bf770a363aa5b))
- Add config system with customizable theme and external layout loading ([65ece5c](https://github.com/jaeheonji/keydrill/commit/65ece5ccfd11ef9ba9bd809d4fdea37d7dbb6888))
- Add debug logging with --debug CLI flag ([9608c1d](https://github.com/jaeheonji/keydrill/commit/9608c1d9a4ae94db86ea7f74a0c961cb74ec3ea5))

### Bug Fixes

- *(ci)* Use macos-latest with cross-compilation for x86_64 target ([53b296c](https://github.com/jaeheonji/keydrill/commit/53b296cbb90119374e838a78ac32d313ece3c163))

### Refactor

- Introduce key! macro to reduce PHYSICAL_KEYS verbosity ([245632d](https://github.com/jaeheonji/keydrill/commit/245632d3e12f22dd9d7851114a162da084d54cbc))
- Fix clippy warnings, Unicode bug, and reduce code duplication ([9d93679](https://github.com/jaeheonji/keydrill/commit/9d93679a6346fa268587f2bf577fa1172d7cd293))

### Documentation

- Add README, configuration guide, and custom layouts guide ([3b376cb](https://github.com/jaeheonji/keydrill/commit/3b376cb78ea6a0f8dea8b1402c1f71a44615572f))
- Add example custom Colemak layout file ([30723b1](https://github.com/jaeheonji/keydrill/commit/30723b17e19916eb49a75c07c6c929182c852259))

### Styling

- Apply rustfmt formatting across UI and layout modules ([93196b7](https://github.com/jaeheonji/keydrill/commit/93196b723fef310f304db5e931ad653986caa7dd))

### Miscellaneous

- Exclude changelog update commits from changelog [skip ci] ([2ce203a](https://github.com/jaeheonji/keydrill/commit/2ce203abc86faca4b3a614433e1a41a7ac38a0b2))
- Skip release bump commits from changelog ([73b3464](https://github.com/jaeheonji/keydrill/commit/73b3464ca47b7bcee3dc87999f1f5b020b3390ca))
- Add package authors and description ([b5f89cd](https://github.com/jaeheonji/keydrill/commit/b5f89cd05ec9c1c6f9f5caac9bf892338a81ada5))

### Ci

- Add GitHub Actions CI/CD and git-cliff configuration ([5943482](https://github.com/jaeheonji/keydrill/commit/59434829a9f2b95704d8c9c6aa4a38d10dd8c3f1))


