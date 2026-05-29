---
name: update-clippy-lints
description: Update this repository's Cargo.toml Clippy lint configuration after reviewing rust-clippy changelog entries.
disable-model-invocation: true
---

# Update Clippy lints

This skill is only for this repository's lint policy in `Cargo.toml`: warn all of `clippy::pedantic` through the group, keep explicit `allow` exceptions for pedantic lints we do not want, and warn many selected `clippy::restriction` lints individually.

## State

Use `.agents/skills/update-clippy-lints/last-checked-clippy-version.txt` as the proposed repository-local state file. It contains exactly one released Clippy/Rust version, for example `1.96`.

## Repository template note

This repository is a cargo-generate template. `Cargo.toml` may contain invalid placeholders such as `{{project-name}}`, so if Cargo fails before linting, validate in a temporary copy instead of the repository root.

In that temporary copy, replace:

- `{{project-name}}` with `placeholder`
- `{{project_description}}` with `placeholder`

Run Clippy in the temporary copy only.

## Workflow

1. Read `Cargo.toml` and the state file. Extract:
   - the `clippy::pedantic` group setting and explicit pedantic exceptions;
   - the individual lints after the `# below lints are from clippy::restriction` comment.
2. Fetch the raw changelog Markdown directly from `https://raw.githubusercontent.com/rust-lang/rust-clippy/master/CHANGELOG.md`.
3. Review changelog entries after the stored version through the latest released `## Rust X.Y` heading. Treat `Unreleased / Beta / In Rust Nightly` separately and do not advance the state file to it.
4. For each reviewed version, inspect at least the `New Lints` and `Moves and Deprecations` sections, plus any entry mentioning a lint currently configured in `Cargo.toml`.
5. For configured lints that were renamed, deprecated, removed, or merged into another lint, update `Cargo.toml` to use the current lint name or remove the obsolete entry when there is no replacement.
6. If a configured individual lint moved out of `restriction` into a group already warned or denied by default, or by this repo's group settings, remove its individual entry. In this repo that means at least `correctness`, `suspicious`, `style`, `complexity`, `perf`, and `pedantic`.
7. For every lint newly added to `restriction`, do not add it immediately. Report it to the user with:
   - lint name;
   - version where it appeared;
   - changelog PR link;
   - description link `https://rust-lang.github.io/rust-clippy/master/index.html#<lint_name>`;
   - a short note if the lint looks potentially noisy or policy-changing.
   Ask whether to add each lint.
8. Apply only user-approved new `restriction` lints to `Cargo.toml`.
9. Keep the restriction lint list sorted as it already is, alphabetically by lint name, and preserve the existing comment style.
10. Run Clippy with the latest reviewed released toolchain, not necessarily the local default. If needed, install it with `rustup toolchain install <version> --component clippy`, then run `cargo +<version> clippy --all-targets --all-features -- -D unknown-lints` to catch unknown, renamed, or removed lints. Use the exact released toolchain version, for example `1.96` or `1.96.1`. Fix lint-configuration errors caused by the update.
11. When the review, user decisions, and validation are complete, update the state file to the latest released `Rust X.Y` heading fully reviewed.

## Reporting

Summarize:

- Cargo.toml changes made for renamed, deprecated, removed, or moved lints.
- New `restriction` lints the user accepted and added.
- New `restriction` lints the user rejected or deferred.
- The previous and new last-checked Clippy versions.
