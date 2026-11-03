# Kolos OS v0.2.1 boot target

This is a separate x86-64 UEFI kernel target. It is intentionally separate from the stable Python language release. It boots through the Rust OSDev UEFI bootloader, reports the bootloader memory map over COM1, and exits QEMU with a machine-readable status for smoke testing.

## Requirements

- Rust nightly with `llvm-tools-preview`, `x86_64-unknown-none`, and `x86_64-unknown-uefi` (installed by `rust-toolchain.toml`)
- QEMU with `qemu-system-x86_64`
- OVMF firmware with Secure Boot disabled
- Network access on first build to download Cargo dependencies

## Build and boot

```sh
cargo run --release
```

A successful smoke test boots the kernel, runs `eval`, `kolos demo`, and `echo`
through the serial shell, then exits QEMU successfully. GitHub Actions runs this
test on Ubuntu. After the tag's CI run passes, the release workflow publishes
the bootable `kolos-os-uefi.img` image and its SHA-256 checksum.

Run the interactive GUI shell with:

```sh
cargo run --release -- --interactive
```

The kernel is currently a bootable systems-development target, not a
general-purpose operating system: storage, userspace, and a persistent
filesystem are not implemented.
