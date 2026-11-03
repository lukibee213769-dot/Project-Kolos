# Projekt Kolos — najnowsze wydanie Kolos OS v0.2.1

Najnowszym wydaniem systemu operacyjnego w tym repozytorium jest eksperymentalny,
bootowalny **Kolos OS v0.2.1** ([pobierz wydanie](https://github.com/lukibee213769-dot/Project-Kolos/releases/tag/os-v0.2.1)).
Numer `1.0.1` dotyczy osobnego, stabilnego wydania języka Kolos i pakietu Python —
nie jest poprzednią wersją Kolos OS. Te komponenty są wersjonowane niezależnie.

Dystrybucja pakietu Python zawiera wyłącznie opisany niżej toolchain języka.
Osobne workspace Rust kernela/OS oraz menedżera pakietów nie wchodzą w skład
artefaktów pakietu Python ani w jego gwarancję wsparcia.

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

### Kolos OS v0.2.1 — najnowsze wydanie (eksperymentalne)

`kernel/os/` to osobny workspace nightly Rust z bootowalnym jądrem UEFI x86-64
i powłoką z bare-metalowym VM Kolos. Smoke test QEMU sprawdza boot, polecenia
`help`, `mem`, `eval`, `kolos demo`, `echo` oraz poprawne zamknięcie QEMU.
Wydanie v0.2.1 zawiera obraz UEFI i sumę SHA-256:

[Pobierz najnowsze wydanie Kolos OS v0.2.1 z GitHub Releases](https://github.com/lukibee213769-dot/Project-Kolos/releases/tag/os-v0.2.1)

```bash
cd kernel/os
cargo run --release
```

Wymaga nightly Rust, `qemu-system-x86_64` oraz OVMF z wyłączonym Secure Boot.
Tryb interaktywny uruchomisz przez `cargo run --release -- --interactive`. OS
pozostaje prototypem: nie ma trwałego systemu plików, storage ani userspace.
Szczegóły:
[kernel/os/README.md](kernel/os/README.md).

Patrz: [RUNNING.md](RUNNING.md) — pełne instrukcje.

## Contributing

Zainteresowany? Przeczytaj [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — patrz [LICENSE](LICENSE).
