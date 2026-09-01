"""Kolos source-to-bytecode execution pipeline."""

from typing import Any, List, Optional, Tuple

from compilers.parser import parse
from compilers.compiler import compile_program
from runtime.bytecode.vm import VM


def compile_source(source: str) -> List[Tuple[str, Any]]:
    """Compile Kolos source code into VM bytecode instructions."""
    ast = parse(source)
    return compile_program(ast)


def execute_source(source: str, vm: Optional[VM] = None) -> Any:
    """Compile and execute Kolos source code on the VM."""
    bytecode = compile_source(source)
    if vm is None:
        vm = VM(bytecode)
    else:
        vm.bytecode = bytecode
        vm.ip = 0
    return vm.run()


def execute_file(file_path: str, vm: Optional[VM] = None) -> Any:
    """Read a Kolos source file, compile and execute it."""
    with open(file_path, "r", encoding="utf-8") as f:
        source = f.read()
    return execute_source(source, vm)