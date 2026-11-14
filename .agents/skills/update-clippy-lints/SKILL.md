---
name: update-clippy-lints
description: Update this repository's Cargo.toml Clippy, Cargo and rustc lint configuration after reviewing their changes since the last checked Rust release.
disable-model-invocation: true
---

# Update Clippy, Cargo and rustc lints

This skill is only for this repository's lint policy in `Cargo.toml`: warn all of `clippy::pedantic` through the group, keep explicit `allow` exceptions for pedantic lints we do not want, warn many selected `clippy::restriction` lints individually, warn all of `cargo::pedantic` through the group, and warn or deny selected allowed-by-default rustc lints individually.

## State

Use `.agents/skills/update-clippy-lints/last-checked-clippy-version.txt` as the proposed repository-local state file. It contains exactly one released Rust version, for example `1.96`. That version covers the Clippy, Cargo and rustc lint reviews.

## Repository template note

This repository is a cargo-generate template. `Cargo.toml` may contain invalid placeholders such as `{{project-name}}`, so if Cargo fails before linting, validate in a temporary copy instead of the repository root.

In that temporary copy, replace:

- `{{project-name}}` with `placeholder`
- `{{project_description}}` with `placeholder`

Run Cargo commands in the temporary copy only.

## Workflow

1. Read `Cargo.toml` and the state file. Extract:
   - the `clippy::pedantic` group setting and explicit pedantic exceptions;
   - the individual lints after the `# below lints are from clippy::restriction` comment;
   - the `[lints.cargo]` group settings and individual lints;
   - the `[lints.rust]` individual lints.
2. Fetch the raw Clippy changelog Markdown directly from `https://raw.githubusercontent.com/rust-lang/rust-clippy/master/CHANGELOG.md`.
3. Review changelog entries after the stored version through the latest released `## Rust X.Y` heading. Treat `Unreleased / Beta / In Rust Nightly` separately and do not advance the state file to it.
4. For each reviewed version, inspect at least the `New Lints` and `Moves and Deprecations` sections, plus any entry mentioning a lint currently configured in `Cargo.toml`.
5. Fetch the Cargo lints reference for the stored version and for the latest reviewed release, `https://doc.rust-lang.org/<version>/cargo/reference/lints.html` with the full version, for example `1.100.0`, and compare the lints listed in each group to find added, moved, and removed Cargo lints. To tell a rename apart from a removal and an addition, read the `## Cargo X.Y` sections of `https://raw.githubusercontent.com/rust-lang/cargo/master/doc/book/src/CHANGELOG.md` for the same versions; that changelog also lists unreleased versions, which this review skips.
6. Install, if needed, the toolchains of the stored version and of the latest reviewed release, and compare the `Lint checks provided by rustc` table printed by `rustc +<version> -W help` for each, to find added, removed, and re-levelled rustc lints. That table spells lint names with hyphens where `Cargo.toml` uses underscores.
7. For configured Clippy, Cargo or rustc lints that were renamed, deprecated, removed, or merged into another lint, update `Cargo.toml` to use the current lint name or remove the obsolete entry when there is no replacement.
8. Remove the individual entry of a configured Clippy or Cargo lint that moved out of `restriction` into a group already warned or denied by default, or by this repo's group settings, and of a configured rustc lint whose default level is now at least its configured level. For the groups, in this repo that means at least `correctness`, `suspicious`, `style`, `complexity`, `perf`, and `pedantic`.
9. For every lint newly added to Clippy's or Cargo's `restriction` group, or newly allowed-by-default in rustc, do not add it immediately. Skip unstable rustc lints: compiling an empty `fn main() {}` with `rustc +<version> -W <lint_name>` reports them as an `unknown lint` that `is unstable`. Report each remaining lint to the user with:
   - lint name;
   - version where it appeared;
   - changelog PR link, when the changelog has one;
   - description link, `https://rust-lang.github.io/rust-clippy/master/index.html#<lint_name>` for Clippy, `https://doc.rust-lang.org/cargo/reference/lints.html#<lint_name>` for Cargo, or `https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#<lint-name>`, with hyphens, for rustc;
   - a short note if the lint looks potentially noisy or policy-changing.
   Ask whether to add each lint.
10. Apply only user-approved new lints to `Cargo.toml`.
11. Keep each individual lint list sorted alphabetically by lint name, as the Clippy restriction list already is, and preserve the existing comment style.
12. Run Clippy with the latest reviewed released toolchain, not necessarily the local default. If needed, install it with `rustup toolchain install <version> --component clippy`, then run `cargo +<version> clippy --all-targets --all-features`. Use the exact released toolchain version, for example `1.96` or `1.96.1`. Treat any `unknown lint`, `has been renamed to`, or `has been removed` warning as a failure: lints configured in `Cargo.toml` only produce warnings, whatever `-D` flags are passed. Fix lint-configuration errors caused by the update.
13. Run `cargo +<rust-version> check --all-targets --all-features` with the toolchain of the `rust-version` set in `Cargo.toml`. If it warns about the lint configuration, such as an `unknown lint` or an `unused manifest key` under `lints`, raise `rust-version` to the oldest release where it no longer does: this template's `rust-version` is a starting point for generated projects, not a compatibility promise.
14. When the review, user decisions, and validation are complete, update the state file to the latest released `Rust X.Y` version fully reviewed for Clippy, Cargo and rustc.

## Reporting

Summarize:

- Cargo.toml changes made for renamed, deprecated, removed, or moved lints.
- New lints the user accepted and added.
- New lints the user rejected or deferred.
- Any `rust-version` change, with the warning that required it.
- The previous and new last-checked versions.
