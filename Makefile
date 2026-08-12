PYTHON=.venv/Scripts/python.exe

.PHONY: setup test lint run-bootstrap run-sample

setup:
	python -m venv .venv
	$(PYTHON) -m pip install --upgrade pip
	$(PYTHON) -m pip install -r requirements.txt

test:
	$(PYTHON) -m unittest discover -s tests -p "test_*.py" -v

lint:
	$(PYTHON) -m flake8 . || true

run-bootstrap:
	$(PYTHON) bootstrap.py

run-sample:
	$(PYTHON) -c "from runtime.proto_vm.main import run_file; print(run_file('tests/sample.asm'))"
