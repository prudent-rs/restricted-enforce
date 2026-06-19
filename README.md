# private

This will be a set of macros to enable other crates to export macros (either declarative/by
example/using `macro_rules`, or procedural) with private-like variables or constants.

## Blockers and related issues

Please give thumbs up (and contribute, if you can) to

- [SergioBenitez/proc-macro2-diagnostics#13](https://github.com/SergioBenitez/proc-macro2-diagnostics/issues/13)
  defect: Error message and details missing, when macro fails to generate main() on STABLE
<!-- @TODO
## Normally Unicode-compatible

`private` _does_ allow non-ASCII identifiers, as per [Rust RFC
2457](https://rust-lang.github.io/rfcs/2457-non-ascii-idents.html). You can have non-ASCII characters in either/both

- `path` right of `@` (optional; like `$crate` or `$crate::module::submodule` if used from a consumer/3rd party macro),
  and
- the `name` (given by you or the developer of the consumer/3rd party macro).

However, there is a very rare Unicode-related incompatibility. It happens only when

- migrating/copying existing code,
- with existing `name` containing (some, but _not_ all) character(s) in an alphabet that differentiates between capital/uppercase letters and small/lowercase letters,
- having both a variable (`let` or `let mut`) and `
- migrating/renaming `name` to 

This is _not_ `private`-specific. The problem would surface even if similar code were written manually, and then migrated/renamed.
-->
## NOT watt-compatible

NOT compatible with [dtolnay/watt](https://github.com/dtolnay/watt) (because of side effects of
build.rs).
