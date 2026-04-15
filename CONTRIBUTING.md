# Contributing to GigawattPkg

## GitHub Flow

1. Create a branch from `main` named `feat/<short-name>`, `fix/<short-name>`, or `chore/<short-name>`.
2. Make focused commits and keep each PR small enough to review quickly.
3. Open a pull request against `main`.
4. Ensure CI passes (fmt, clippy, tests, and release build).
5. Squash-merge into `main` once approved.

## Commit and PR style

- Prefer clear, imperative commit messages, for example: `feat: add dnf backend detection`.
- Link related issues in PR descriptions.
- Keep behavior changes covered by tests where practical.

## Release process

Releases are tag-driven through GitHub Actions.

1. From up-to-date `main`, create and push a semver tag:
   - `git tag v0.1.1`
   - `git push origin v0.1.1`
2. The `Release` workflow builds `gigawattpkg` and `gwpkg`, packages them, and creates a GitHub Release.
3. Generated release notes are attached automatically, along with a SHA-256 checksum file.

## Local quality gate

Run this before opening a PR:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
```
