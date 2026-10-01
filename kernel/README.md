# Kolos Kernel

`src/` retains the host-executable Rust VM and scheduler model used by the existing kernel tests. The bootable x86-64 UEFI kernel and QEMU smoke test live in [`os/`](os/README.md), in a separate nightly Rust workspace.

The UEFI target currently boots through the Rust OSDev bootloader, reports the bootloader-provided memory map on COM1, and exits QEMU with a test status. It is an early hobby-OS foundation, not yet a general-purpose operating system; input devices, storage, userspace, and persistent filesystems are not implemented.
