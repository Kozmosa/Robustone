# Issue 57 Rustdoc Warning Cleanup Design

## Goal

Clean up all rustdoc warnings currently produced by `cargo doc --workspace --all-features --no-deps`, not only the examples listed in issue #57. The change is documentation-only: no public API, decoding behavior, formatting behavior, or runtime logic changes.

## Scope

Fix the current workspace rustdoc warnings:

- `robustone-core/src/architecture.rs`: public `From` implementation docs link to private `Architecture::parse`.
- `robustone-core/src/utils/endian.rs`: `Vec<u8>` is parsed as an HTML tag instead of code.
- `robustone-isa/src/lib.rs`: bracketed bit positions such as `[31]` and `[30:25]` are parsed as intra-doc links.

Do not add new lint policy, CI configuration, or API visibility changes as part of this issue.

## Approach

Use the smallest documentation edits that preserve each comment's meaning while preventing rustdoc from interpreting plain notation as links or HTML:

- Replace intra-doc links to private items with plain code text or equivalent prose.
- Wrap generic Rust types in code spans.
- Wrap ISA bit-field bracket notation in code spans.

## Verification

Run `cargo doc --workspace --all-features --no-deps` and require zero rustdoc warnings. Run `cargo test --workspace --all-features` if needed to confirm no behavior was affected.
