# Projekt Kolos

**Kolos 1.0.0** is the stable release of the Kolos programming language, compiler, Python interpreters, and CLI.

The Rust kernel and package manager are experimental prototypes, not a production operating system or dependency manager. This release's support guarantee applies to the Python language toolchain described below.

[![CI](https://github.com/lukibee213769-dot/Project-Kolos/actions/workflows/ci.yml/badge.svg)](https://github.com/lukibee213769-dot/Project-Kolos/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Struktura projektu

- **kernel/** — Rust kernel prototype (VM, scheduler, memory mgmt)
- **runtime/** — Python bytecode VM, interpreter, REPL
- **compilers/** — Lexer, parser, AST evaluator, bytecode assembler
- **tools/** — Linter, formatter, diagnostics
- **pkg/** — local package manifest prototype; dependency installation/resolution is not implemented
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

Patrz: [RUNNING.md](RUNNING.md) — pełne instrukcje.

## Contributing

Zainteresowany? Przeczytaj [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — patrz [LICENSE](LICENSE).

