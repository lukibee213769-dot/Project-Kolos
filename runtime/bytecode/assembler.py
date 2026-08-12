"""Assembler that compiles simple arithmetic expressions into bytecode."""
import ast
from typing import List, Tuple


def _compile_node(node, out: List[Tuple[str, object]]):
    if isinstance(node, ast.BinOp):
        _compile_node(node.left, out)
        _compile_node(node.right, out)
        if isinstance(node.op, ast.Add):
            out.append(('ADD', None))
        elif isinstance(node.op, ast.Sub):
            out.append(('SUB', None))
        elif isinstance(node.op, ast.Mult):
            out.append(('MUL', None))
        elif isinstance(node.op, ast.Div):
            out.append(('DIV', None))
        elif isinstance(node.op, ast.Pow):
            out.append(('POW', None))
        elif isinstance(node.op, ast.Mod):
            out.append(('MOD', None))
        else:
            raise ValueError('Unsupported binary op')
    elif isinstance(node, ast.UnaryOp):
        if isinstance(node.op, ast.USub):
            _compile_node(node.operand, out)
            out.append(('PUSH', -1))
            out.append(('MUL', None))
        else:
            raise ValueError('Unsupported unary op')
    elif isinstance(node, ast.Constant):
        out.append(('PUSH', node.value))
    else:
        raise ValueError(f'Unsupported node type: {type(node)}')


def assemble(expr: str) -> List[Tuple[str, object]]:
    """Assemble a numeric expression into bytecode list."""
    tree = ast.parse(expr, mode='eval')
    out = []
    _compile_node(tree.body, out)
    out.append(('HALT', None))
    return out


def assemble_text(asm: str) -> List[Tuple[str, object]]:
    """Assemble a small assembly-like language with labels and simple instrs.

    Supported instructions: PUSH <num>, ADD, SUB, MUL, DIV, POW, MOD,
    POP, DUP, PRINT, JMP <label>, JZ <label>, HALT

    Labels are `name:` at start of line.
    """
    lines = [ln.split('#', 1)[0].strip() for ln in asm.splitlines()]
    # First pass: collect labels
    labels = {}
    instrs = []
    for line in lines:
        if not line:
            continue
        if line.endswith(':'):
            labels[line[:-1]] = len(instrs)
        else:
            instrs.append(line)
    # Second pass: assemble
    out: List[Tuple[str, object]] = []
    for line in instrs:
        parts = line.split()
        op = parts[0].upper()
        if op == 'PUSH':
            val = ast.literal_eval(' '.join(parts[1:]))
            out.append(('PUSH', val))
        elif op in (
            'ADD',
            'SUB',
            'MUL',
            'DIV',
            'POW',
            'MOD',
            'POP',
            'DUP',
            'PRINT',
            'HALT',
        ):
            out.append((op, None))
        elif op in ('JMP', 'JZ'):
            label = parts[1]
            if label not in labels:
                raise ValueError(f'Unknown label: {label}')
            out.append((op, labels[label]))
        else:
            raise ValueError(f'Unsupported op: {op}')
    return out
