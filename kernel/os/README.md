# Kolos OS boot target

This is a separate x86-64 UEFI kernel target. It is intentionally separate from the stable Python language release. It boots through the Rust OSDev UEFI bootloader, reports the bootloader memory map over COM1, and exits QEMU with a machine-readable status for smoke testing.

## Requirements

- Rust nightly with `llvm-tools-preview`, `x86_64-unknown-none`, and `x86_64-unknown-uefi` (installed by `rust-toolchain.toml`)
- QEMU with `qemu-system-x86_64`
- Network access on first build to download Cargo dependencies and OVMF firmware

## Build and boot

```sh
cargo run -- uefi
```

A successful boot prints `KOLOS_BOOT_OK` on the serial console and exits QEMU successfully. GitHub Actions runs this command on Ubuntu. The kernel is currently a bootable systems-development target, not a general-purpose operating system: storage, input devices, userspace, and a persistent filesystem are not implemented.
