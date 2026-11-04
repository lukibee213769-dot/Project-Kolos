# Changelog

All notable changes to this project will be documented in this file.

## [Kolos OS 0.2.2] - 2026-11-04

### Final release
- Mark the bootable OS prototype's planned development as concluded; the repository remains open.
- Align the kernel package, shell banner, and current documentation on Kolos OS v0.2.2.
- Make the Rust kernel CI job a required release gate instead of allowing its failures.
- Rebuild and publish the UEFI image and SHA-256 checksum from the passing v0.2.2 tag CI.
- Preserve the scope limits: this is not a general-purpose OS and has no storage, userspace, or persistent filesystem.

## [Kolos OS 0.2.1] - 2026-11-03

### Fixed
- Select OVMF firmware with Secure Boot disabled so the unsigned UEFI image can boot.
- Poll COM1 for shell input instead of relying on the faulty IRQ4 receive path.
- Check shell commands, expression evaluation, the bytecode VM result, and clean QEMU exit in CI.
- Correct the `kolos demo` loop exit target and preserve its final sum as the VM result.
- Run the OS runner's firmware-selection unit tests in CI.
- Synchronize the OS Cargo lockfile with the current kernel manifest.
- Publish the bootable UEFI image and SHA-256 checksum after successful CI for the `os-v0.2.1` tag.

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

The bootable Kolos OS job (`bootable-os-qemu`) now passes end to end: the UEFI
image builds and the kernel actually boots in QEMU.

### Fixed
- Bootable Kolos OS: declared the UEFI kernel as a Cargo artifact dependency so the build order is explicit, then removed it again once it became clear that `CARGO_BIN_FILE_KERNEL_kernel` is not populated for a cross-target artifact.
- Bootable Kolos OS: replaced the blocking `Command::output()` with a polled supervisor — the serial console is drained on a helper thread and the main thread polls `try_wait()`, killing the VM after 120 s. An earlier `Arc<Mutex<Child>>` watchdog deadlocked against `wait_with_output()` and hung the job until the 10-minute step timeout.
- Bootable Kolos OS: `build.rs` now invokes `cargo build -p kernel --target x86_64-unknown-none` itself and reads the artifact from the target directory. Cargo only sets `CARGO_BIN_FILE_*` when the artifact target matches the dependent package's target, which never held for a host runner driving a bare-metal kernel.
- Bootable Kolos OS: the kernel is built for `x86_64-unknown-none`, not `x86_64-unknown-uefi`. `bootloader_api::entry_point!` exports `_start`; the EFI application link flavor expects `efi_main` and failed with "undefined symbol: efi_main".
- Bootable Kolos OS: the nested kernel build now writes to `target/kernel-build/`. Sharing the outer target directory deadlocked the build, because the parent Cargo holds an exclusive lock on it for the entire build.
- Bootable Kolos OS: added the `rust-src` component (required by the bootloader build) to `rust-toolchain.toml` and to the CI toolchain step.
- Bootable Kolos OS: dropped the stray `uefi` argument from `cargo run -Zbindeps -- uefi`, which Cargo forwarded to the runner binary.
- CI: split the QEMU job into separate build (25 min budget) and boot (5 min budget) steps, and raised the apt timeouts. The single combined step kept hitting the 10-minute step limit.
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
