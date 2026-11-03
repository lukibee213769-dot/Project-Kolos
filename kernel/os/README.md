# Kolos OS v0.2.1

Experimental, bootable x86-64 UEFI OS prototype. This is a separate release from
the stable Kolos language/Python toolchain (`1.0.1`). The kernel boots through
the Rust OSDev UEFI bootloader, initializes its heap and framebuffer, and
provides a shell with PS/2 keyboard and polled COM1 serial input.

The published v0.2.1 UEFI image and SHA-256 checksum are available from the
[GitHub Release](https://github.com/lukibee213769-dot/Project-Kolos/releases/tag/os-v0.2.1).

## Requirements

- Rust nightly with `llvm-tools-preview`, `x86_64-unknown-none`, and `x86_64-unknown-uefi` (installed by `rust-toolchain.toml`)
- QEMU with `qemu-system-x86_64`
- OVMF firmware with Secure Boot disabled
- Network access on first build to download Cargo dependencies

## Build and boot

```sh
cargo run --release
```

A successful smoke test boots the kernel, runs `help`, `mem`, `kolos demo`,
`eval`, and `echo` through the serial shell, then exits QEMU successfully.
GitHub Actions runs this test on Ubuntu. The v0.2.1 tag's CI passed, and the
release workflow published `kolos-os-uefi.img` and its SHA-256 checksum.

Run the interactive GUI shell with:

```sh
cargo run --release -- --interactive
```

This is a bootable systems-development target, not a general-purpose operating
system. Storage, userspace, and a persistent filesystem are not implemented.
