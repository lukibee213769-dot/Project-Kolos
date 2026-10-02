# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2026-10-01

### Added
- **Object-Oriented Programming (OOP) Support** 🎉
  - Class definitions with `class` keyword
  - Constructors for initialization
  - Instance methods with `this` reference
  - Property access and assignment (dot notation)
  - Method calls on instances
  - Full object instantiation with `new` keyword

### Fixed
- Logical `and` and `or` now short-circuit instead of evaluating skipped expressions.
- Installed packages include nested runtime modules and the sample assembly used by `kolos run-sample`.
- Corrected package metadata and GitHub repository links; builds now produce source and wheel distributions.
- CI now blocks on Python correctness lint, runs the declared Python-version matrix, builds distributions, and smoke-tests the installed CLI.

## [1.0.1] - 2026-10-01

### Fixed
- Excluded the unfinished package-manager workspace from the stable Python distribution; kernel code was already excluded from the wheel and source archive.
- Clarified the supported release scope and aligned package and interpreter versions.

## [1.0.1 updated] - 2026-10-02

### Fixed
- Bootable Kolos OS: declared the UEFI kernel as a Cargo **artifact dependency**, so the `build.rs` script finally receives `CARGO_BIN_FILE_KERNEL_kernel` instead of panicking. This was the root cause of every `bootable-os-qemu` CI failure.
- Bootable Kolos OS: pinned the kernel artifact to the `x86_64-unknown-uefi` target and added the `rust-src` component (required by the bootloader build) to `rust-toolchain.toml` and to the CI toolchain step.
- Bootable Kolos OS: replaced the blocking `Command::output()` with a piped, 120-second watchdog so a hung firmware boot fails fast instead of burning the whole job budget.
- Bootable Kolos OS: dropped the stray `uefi` argument from `cargo run -Zbindeps -- uefi`, which Cargo forwarded to the runner binary.
- CI: upgraded `actions/checkout` and `actions/setup-python` to Node 24-compatible releases to clear the deprecation notice.
- Repository: stopped tracking the generated `kernel/target/` build directory that had been committed by accident.

## [0.1.0-alpha] - 2026-09-01

### Added
- Initial project scaffold and prototype runtime (REPL, interpreter)
- Bytecode VM, assembler, loader with comprehensive tests
- Rust kernel prototype with stubs for VM, scheduler, and memory management
- Python CLI (`kolos`) with subcommands: `repl`, `bootstrap`, `run-sample`
- Lexer, parser, and AST evaluator for Kolos language
- Package manager prototype
- Tools: linter and formatter stubs
- CI/CD pipeline with Python tests and optional Rust kernel build
- Contributing guidelines and issue/PR templates
- Docker support (Dockerfile + compose files)
- Comprehensive documentation in `docs/`

### Fixed
- Modernized build system (pyproject.toml with setuptools backend)
- Improved CI workflow for better error handling and platform separation

