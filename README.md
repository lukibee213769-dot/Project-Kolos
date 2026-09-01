# Projekt Kolos

**Kolos** — амбициозny, modularny projekt: system operacyjny + język programowania + ekosystem narzędzi. 
Wersja robocza (0.1.0-alpha).

[![CI](https://github.com/lukibee213769-dot/a/actions/workflows/ci.yml/badge.svg)](https://github.com/lukibee213769-dot/a/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Struktura projektu

- **kernel/** — Rust kernel prototype (VM, scheduler, memory mgmt)
- **runtime/** — Python bytecode VM, interpreter, REPL
- **compilers/** — Lexer, parser, AST evaluator, bytecode assembler
- **tools/** — Linter, formatter, diagnostics
- **pkg/** — Package manager prototype
- **docs/** — Architecture and design docs
- **examples/** — Sample .kolos program files
- **tests/** — Unit tests (Python + Rust)

## Szybki start (Python Runtime)

```powershell
# Clone i setup venv
git clone https://github.com/lukibee213769-dot/a.git
cd a
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

