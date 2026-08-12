# How to run Project Kolos (summary)

Prerequisites:
- Python 3.11+ and a venv (recommended)
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
