# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.2] - 2017-11-06

### Added

- The missing STATX system call.

## [v0.2.1] - 2017-05-06

### Added

- Some ARM specific syscals.

## [v0.2.0] - 2017-02-27

### Added

- More syscalls. Syscalls are now generated from the latest Linux version (4.10
  at the time of writing this) source code.

### Changed

- [breaking-change]. Some syscalls have been renamed to match the Linux source
  code.
  - ARM
    - SYNC_FILE_RANGE2 -> ARM_SYNC_FILE_RANGE
  - SPARC64
    - GETRESGID -> GETRESGID32
    - GETRESUID -> GETRESUID32
    - SETRESGID -> SETRESGID32
    - SETRESUID -> SETRESUID32

### Fixed

- x86 syscall6. It was hitting rust-lang/rust#39098

## [v0.1.5] - 2017-02-03

### Fixed

- The syscall number for GETRANDOM on x86

## [v0.1.4] - 2017-02-02

### Added

- GETRANDOM and NEWFSTATAT syscalls

## [v0.1.3] - 2017-01-21

### Fixed

- Error when using MIPS' syscall5/syscall6 with optimizations enabled

## [v0.1.2] - 2017-01-21

### Added

- syscall5 and syscall6 for mips

## [v0.1.1] - 2017-01-14

### Fixed

- syscall2 for linux-aarch64. There was a type in the register constraints.

## v0.1.0 - 2017-01-13

Initial release. Forked from [syscall] v0.2.1.

[syscall]: https://crates.io/crates/syscall

- Support for ARM, Aarch64, MIPS, MIPS64, PowerPC, PowerPC64, SPARC64, x86 and
  x86_64 Linux

- Support for x86_64 macOS

- Support for x86_64 FreeBSD

[Unreleased]: https://github.com/japaric/syscall.rs/compare/v0.2.2...HEAD
[v0.2.2]: https://github.com/japaric/syscall.rs/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/japaric/syscall.rs/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/japaric/syscall.rs/compare/v0.1.5...v0.2.0
[v0.1.5]: https://github.com/japaric/syscall.rs/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/japaric/syscall.rs/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/japaric/syscall.rs/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/japaric/syscall.rs/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/syscall.rs/compare/v0.1.0...v0.1.1
