# AXIOM project status

**Status:** Planned development concluded on 2026-11-04. The GitHub repository
remains public and open; this status does not archive it or prevent future
contributions.

## Final release versions

- **AXIOM OS v0.2.2** — final planned release of the experimental bootable
  x86-64 UEFI prototype. Release assets are published on the
  [GitHub Releases page](https://github.com/lukibee213769-dot/AXIOM/releases/tag/os-v0.2.2).
- **AXIOM language/Python toolchain 1.0.2** — separate stable distribution.
  The OS version does not change this package's version.
- The host Rust kernel crate and the OS runner have their own Cargo package
  versions; they are not the project-wide release numbers.

## What is verified

The final OS tag is required to pass the Python CI matrix, Rust kernel checks,
OS runner tests, UEFI image build, and QEMU smoke test before the release
workflow publishes the image and SHA-256 checksum. The QEMU smoke test checks
boot, shell commands, expression evaluation, the bytecode VM demo, and clean
emulator exit.

## Known scope limits

AXIOM OS is a bootable prototype, not a general-purpose operating system. It
does not implement storage, userspace, or a persistent filesystem. Serial
shell input is polled on COM1. No claim is made that the project is
bug-free on all hardware or environments; the verified boot test is the
documented QEMU/Ubuntu CI path.

The repository is intentionally left open. No further feature work is planned
as part of this final release.
