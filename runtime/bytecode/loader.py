from .assembler import assemble_text
from .vm import VM


def run_assembly_text(asm_text: str):
    bc = assemble_text(asm_text)
    vm = VM(bc)
    return vm.run()


def run_file(path: str):
    with open(path, 'r', encoding='utf-8') as f:
        text = f.read()
    return run_assembly_text(text)
