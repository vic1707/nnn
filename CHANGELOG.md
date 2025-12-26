# Changelog

All notable changes to this project are documented in this file.

## [v1.2.2] - 2025-12-26

- **Feat:** Support for custom test-harness ([7439a6a0](https://github.com/vic1707/nnn/commit/7439a6a0))

## [v1.2.1] - 2025-12-26

- **Chore:** fix zizmor ([ce40569c](https://github.com/vic1707/nnn/commit/ce40569c))
- **Chore:** update deps ([ae64c878](https://github.com/vic1707/nnn/commit/ae64c878))
- **Chore:** Feature gate macros (enabled by default) ([fe3f6641](https://github.com/vic1707/nnn/commit/fe3f6641))

## [v1.2.0] - 2025-10-23

- **Chore:** Upgrade to rust 2024 edition ([8bf801aa](https://github.com/vic1707/nnn/commit/8bf801aa))
- **Chore:** Fix zizmor's pin hash issues ([0cd472e7](https://github.com/vic1707/nnn/commit/0cd472e7))

## [v1.1.0] - 2025-03-10

- **Feature:** add `AsRef` derive ([34e07733](https://github.com/vic1707/nnn/commit/34e07733)).
- **Feature:** add `Deref` derive ([3e9c7fa1](https://github.com/vic1707/nnn/commit/3e9c7fa1)).
- **Feature:** allow some derives to take generic inputs as targets ([bd29f3f1](https://github.com/vic1707/nnn/commit/bd29f3f1))
- **Improvement:**: Add `CHANGELOG.md` ([3fc6aff7](https://github.com/vic1707/nnn/commit/3fc6aff7)).

## [v1.0.2] - 2025-03-09

### Fixed

- **Fix:** Add compile tests for `nnn_derives` and fix `From` ([cb628ad](https://github.com/vic1707/nnn/commit/cb628ad))
- **Chore:** Code formatting update ([5b6d08c](https://github.com/vic1707/nnn/commit/5b6d08c))

## [v1.0.1] - 2025-03-09

- **Fix:** Correct missing use statements and syntax in generated code ([5365393](https://github.com/vic1707/nnn/commit/5365393))
- **Docs:** Improve and add missing documentation ([21af9cb](https://github.com/vic1707/nnn/commit/21af9cb))
- **Chore:** Update dependencies ([0f2d744](https://github.com/vic1707/nnn/commit/0f2d744))

## [v1.0.0] - 2025-03-09

- **Feature:** Add `IntoIterator` derive for nnn ([4564665](https://github.com/vic1707/nnn/commit/4564665))
- **Feature:** Merge in newtype trait changes ([ef3453d](https://github.com/vic1707/nnn/commit/ef3453d))
- **Feature:** Add test for `FromStr` derive ([6877dd9](https://github.com/vic1707/nnn/commit/6877dd9))
- **Chore:** Update dependencies ([6ee2c6e](https://github.com/vic1707/nnn/commit/6ee2c6e))
- **Chore:** Prepare for 2024 edition ([593cff8](https://github.com/vic1707/nnn/commit/593cff8))

## [v0.1.2] - 2025-01-11

- **Chore:** Macro is now `#![no_std]` ([c49fa04](https://github.com/vic1707/nnn/commit/c49fa04))
- **Chore:** Update dependencies ([7abbdb9](https://github.com/vic1707/nnn/commit/7abbdb9))

## [v0.1.1] - 2025-01-01

- **Fix:** Remove `BorrowMut` derive as it can break constraints ([19c9543](https://github.com/vic1707/nnn/commit/19c9543))

## [v0.1.0] - 2025-01-01

- **Fix:** Resolve validators (`positive` & `negative`) issues for `f32` & `f64` ([9c90d9e](https://github.com/vic1707/nnn/commit/9c90d9e))
- **Chore:** Address potential clippy error ([f4d2b0f](https://github.com/vic1707/nnn/commit/f4d2b0f))

## [v0.0.2] - 2024-12-30

- **Fix:** `docs.rs` badge display ([147b2c3](https://github.com/vic1707/nnn/commit/147b2c3))
- **Chore:** `Default` implementation needlessly used `try_new` ([4694041](https://github.com/vic1707/nnn/commit/4694041))

## [v0.0.1] - 2024-12-29

Initial Release
