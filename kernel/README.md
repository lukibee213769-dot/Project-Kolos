# Kolos Kernel

This directory contains two distinct Rust projects with independent versions:

- `src/` is the host-executable Rust VM and scheduler model (`kolos-kernel` crate version `0.1.0`).
- [`os/`](os/README.md) is the separately versioned bootable Kolos OS **v0.2.1**, built in its own nightly Rust workspace.

The OS boots as an x86-64 UEFI image through the Rust OSDev bootloader. The kernel initializes its descriptor tables, PIC, heap, and framebuffer, then provides a shell with PS/2 keyboard and polled COM1 serial input. GitHub Actions smoke-tests the image in QEMU.

Kolos OS v0.2.1 is an experimental systems-development target, not a general-purpose operating system. It does not provide storage, userspace, or a persistent filesystem. Its version does not change the stable Python distribution version, which remains `1.0.1`.
