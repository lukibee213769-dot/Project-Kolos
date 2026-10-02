# Projekt Kolos

**Kolos 1.0.1 updated** is the stable release of the Kolos programming language, compiler, Python interpreters, and CLI.

The supported distribution contains only the language toolchain described below. The separate Rust kernel and package-manager workspaces are not included in the release artifacts or covered by its support guarantee.

[![CI](https://github.com/lukibee213769-dot/Project-Kolos/actions/workflows/ci.yml/badge.svg)](https://github.com/lukibee213769-dot/Project-Kolos/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Struktura projektu

- **kernel/** — separate Rust systems research workspace; not part of the release distribution
- **runtime/** — Python bytecode VM, interpreter, REPL
- **compilers/** — Lexer, parser, AST evaluator, bytecode assembler
- **tools/** — Linter, formatter, diagnostics
- **pkg/** — package-management work in progress; not part of the release distribution
- **docs/** — Architecture and design docs
- **examples/** — Sample .kolos program files
- **tests/** — Unit tests (Python + Rust)

## Szybki start (Python Runtime)

```powershell
# Clone i setup venv
git clone https://github.com/lukibee213769-dot/Project-Kolos.git
cd Project-Kolos
python -m venv .venv
.\.venv\Scripts\Activate.ps1
python -m pip install --upgrade pip
pip install -r requirements.txt
pip install -e .

# Uruchom CLI:
kolos repl          # Bytecode interpreter REPL
kolos bootstrap     # Setup demo
kolos run-sample    # Uruchom sample.asm
```

## Alternatywnie: bez instalacji

```powershell
.\.venv\Scripts\python.exe kolos_cli.py repl
.\.venv\Scripts\python.exe kolos_cli.py run-sample
```

## Language Features

### Object-Oriented Programming (OOP)

```kolos
class Dog {
    fn constructor(name) {
        this.name = name;
    }

    fn bark() {
        print this.name;
        return "Woof!";
    }
}

let dog = new Dog("Buddy");
dog.bark()
```

### Functions & Control Flow

```kolos
fn fibonacci(n) {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

print fibonacci(10)
```

## Setup Rust Kernel (opcjonalnie)

```bash
rustup update
cd kernel
cargo build --release
cargo test
```

### Bootowalne Kolos OS (eksperymentalne)

`kernel/os/` to osobny workspace nightly Rust z bootowalnym jądrem UEFI x86-64
i testem smoke w QEMU:

```bash
cd kernel/os
rustup component add rust-src llvm-tools-preview
cargo run -Z bindeps
```

Wymaga `qemu-system-x86_64` na PATH. Jądro wypisuje mapę pamięci na COM1,
drukuje znacznik `KOLOS_BOOT_OK` i kończy QEMU przez `isa-debug-exit`; runner
zabije VM po 120 s, jeśli boot się zawiesi.

Patrz: [RUNNING.md](RUNNING.md) — pełne instrukcje.

## Contributing

Zainteresowany? Przeczytaj [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — patrz [LICENSE](LICENSE).

