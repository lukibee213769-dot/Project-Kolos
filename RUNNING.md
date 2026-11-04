# How to run Project Kolos (summary)

## Version scope

- The stable Kolos language and Python toolchain release is `1.0.1`.
- The final planned experimental bootable OS release is `v0.2.2`, maintained separately under `kernel/os/`.
- The host Rust kernel crate has its own Cargo version and is not the OS release version.

Prerequisites:
- Python 3.10+ and a venv (recommended)
- (Optional) Rust toolchain (`rustup`, `cargo`) to build the kernel prototype

Quick setup (PowerShell):

```powershell
python -m venv .venv
.\.venv\Scripts\Activate.ps1
python -m pip install --upgrade pip
python -m pip install -r requirements.txt
```

Run bootstrap (prints modules):

```powershell
.\.venv\Scripts\python.exe bootstrap.py
```

Run the proto VM REPL:

```powershell
.\.venv\Scripts\python.exe -m runtime.proto_vm repl
```

Run unit tests:

```powershell
.\.venv\Scripts\python.exe -m unittest discover -s tests -p "test_*.py" -v
```

Run sample assembly file:

```powershell
.\.venv\Scripts\python.exe -c "from runtime.proto_vm.main import run_file; print(run_file('tests/sample.asm'))"
```

Build Rust kernel (optional):

```powershell
cd kernel
cargo build
cargo run
```

Install Rust locally (Windows):

```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy RemoteSigned
Invoke-WebRequest -Uri https://win.rustup.rs -UseBasicParsing -OutFile rustup-init.exe
.\rustup-init.exe -y
```

After installing, open a new terminal (or restart the shell) and run:

```powershell
cd kernel
cargo build
cargo test
cargo run --release
```

Note: CI also builds the kernel on Ubuntu runners; run the commands above locally only if you have `cargo` installed.

Bootable Kolos OS v0.2.2 smoke test (nightly Rust, QEMU, and OVMF with Secure Boot disabled):

```powershell
cd kernel\os
cargo run --release
```

This builds and boots the UEFI image, exercises `help`, `mem`, `eval`,
`kolos demo`, and `echo` through the serial shell, then exits QEMU. The
published v0.2.2 image and SHA-256 checksum are on the
[GitHub Release page](https://github.com/lukibee213769-dot/Project-Kolos/releases/tag/os-v0.2.2).
To start the interactive GUI instead:

```powershell
cargo run --release -- --interactive
```
