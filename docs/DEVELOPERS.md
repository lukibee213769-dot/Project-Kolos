# Developers Guide (Kolos)

This guide explains the workspace layout and how to run the prototype components.

Structure:

- `kernel/` — Rust kernel prototype (Cargo project)
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
