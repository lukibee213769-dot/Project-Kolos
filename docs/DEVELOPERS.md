# Developers Guide (AXIOM)

This guide explains the workspace layout and how to run the prototype components.

The stable AXIOM language/Python toolchain release is `1.0.2`. The bootable OS
is a separate experimental release, `v0.2.2`, in `kernel/os/`; the host Rust
kernel crate has its own Cargo version.

Structure:

- `kernel/` — host Rust kernel/VM prototype and the separate bootable OS target
- `runtime/` — Python runtime prototypes (REPL, bytecode VM)
- `compilers/` — planned compilers
- `pkg/` — package manager components
- `tools/` — developer tools

Running Python tests:

```powershell
.\.venv\Scripts\python.exe -m unittest discover -s tests -p "test_*.py" -v
```

Running the proto VM REPL:

```powershell
.\.venv\Scripts\python.exe -m runtime.proto_vm repl
```

Building Rust kernel (if Rust toolchain installed):

```powershell
cd kernel
cargo build
cargo run
```

Note: The local environment must have the Rust toolchain (`cargo`, `rustc`) installed. CI attempts to build the kernel on Ubuntu runners.

To build and smoke-test AXIOM OS v0.2.2, use the separate
[`kernel/os/README.md`](../kernel/os/README.md) instructions. It requires
nightly Rust, QEMU, and OVMF with Secure Boot disabled.

The planned feature development for AXIOM has concluded; the repository remains
open. See [`PROJECT_STATUS.md`](../PROJECT_STATUS.md).
