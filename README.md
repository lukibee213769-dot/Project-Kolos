# Projekt Kolos

Wersja robocza: "Kolos" — bardzo duży, modularny projekt: system operacyjny + język programowania + ekosystem narzędzi.

Cel tego repozytorium: dostarczyć strukturę projektu, prototypowy runtime i narzędzia do dalszej implementacji.

Struktura katalogów:

- kernel/
- runtime/
- compilers/
- pkg/
- tools/
- infra/
- docs/

Szybki start:

```bash
python bootstrap.py
```

Uruchomienie CLI bez instalacji:

```powershell
# Uruchom bez instalowania pakietu:
.\.venv\Scripts\python.exe kolos_cli.py repl
.\.venv\Scripts\python.exe kolos_cli.py bootstrap
.\.venv\Scripts\python.exe kolos_cli.py run-sample
```

Uwaga: `pip install -e .` może używać `pyproject.toml` i w niektórych konfiguracjach próbować zbudować podprojekty (np. `kernel/`) — jeśli chcesz instalować jako pakiet, upewnij się, że środowisko buildowe jest poprawnie skonfigurowane.

Instalacja i uruchomienie (zalecane):

```powershell
python -m venv .venv
.\.venv\Scripts\Activate.ps1
python -m pip install --upgrade pip
python -m pip install -r requirements.txt
# opcjonalnie: zainstaluj editable package
python -m pip install -e c:\Users\Lukasz\OneDrive\guns

# użyj CLI:
kolos repl
kolos bootstrap
kolos run-sample
```

Aby zbudować kernel (opcjonalnie): patrz `RUNNING.md` — zawiera instrukcje instalacji `rustup` i kroki `cargo build` / `cargo run`.
