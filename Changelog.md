# Changelog
All notable changes to this project will be documented in this file

[unreleased]: https://github.com/eugenesvk/helperes/compare/0.1.2...HEAD
## [Unreleased]
<!-- - __Added__ -->
  <!-- + :sparkles:  -->
  <!-- new features -->
<!-- - __Changed__ -->
  <!-- +   -->
  <!-- changes in existing functionality -->
<!-- - __Fixed__ -->
  <!-- + :beetle:  -->
  <!-- bug fixes -->
<!-- - __Deprecated__ -->
  <!-- + :poop:  -->
  <!-- soon-to-be removed features -->
<!-- - __Removed__ -->
  <!-- + :wastebasket:  -->
  <!-- now removed features -->
<!-- - __Security__ -->
  <!-- + :lock:  -->
  <!-- vulnerabilities -->

- __Added__
  + `stros` feature flag with helper OsStr functions: concatentating with a single allocation


[0.1.2]: https://github.com/eugenesvk/helperes/releases/tag/0.1.2
## [0.1.2]
- __Added__
  + `pe` to a non-panicking variant of error printer

[0.1.1]: https://github.com/eugenesvk/helperes/releases/tag/0.1.1
## [0.1.1]
- __Changed__
  + names of pub imports to avoid conflicts

[0.1.0]: https://github.com/eugenesvk/helperes/releases/tag/0.1.0
## [0.1.0]

- __Added__
  + ✨ type helper functions `type_of` and `print_type_of`
  + ✨ print helper macros/aliases `p` no-panic (propagates errors),  `pp` regular, `pe` error, `pt` type, `pb` during build
  `_mod` proc macro to rewrite path of module imports in a folder to `modname/[modname].rs` (brackets are sorted to the top in a file manager, making the main module more visible)
