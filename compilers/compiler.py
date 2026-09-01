"""AST to Kolos bytecode compiler with label resolution."""

from typing import Any, List, Tuple

from compilers.parser import (
    NumberNode,
    StringNode,
    BooleanNode,
    UnaryNode,
    BinaryNode,
    IdentifierNode,
    VarDeclNode,
    AssignNode,
    BlockNode,
    IfNode,
    WhileNode,
    FunctionDefNode,
    ReturnNode,
    CallNode,
    PrintNode,
    ProgramNode,
)


class CompilerError(Exception):
    """Raised when AST cannot be compiled."""


BINARY_OPS = {
    "PLUS": "ADD",
    "MINUS": "SUB",
    "STAR": "MUL",
    "SLASH": "DIV",
    "POWER": "POW",
    "MODULO": "MOD",
    "EQ": "EQ",
    "NE": "NE",
    "LT": "LT",
    "LE": "LE",
    "GT": "GT",
    "GE": "GE",
    "AND": "AND",
    "OR": "OR",
}


class Label:
    def __init__(self, name: str = ""):
        self.name = name
        self.target: int | None = None


class Compiler:
    """Compiles Kolos AST into VM bytecode tuples with jump resolution."""

    def __init__(self):
        self.instructions: List[Tuple[str, Any]] = []

    def emit(self, opcode: str, arg: Any = None) -> int:
        self.instructions.append((opcode, arg))
        return len(self.instructions) - 1

    def mark_label(self, label: Label):
        label.target = len(self.instructions)

    def visit(self, node: Any):
        if node is None:
            return

        if isinstance(node, NumberNode):
            self.emit("PUSH", node.value)

        elif isinstance(node, StringNode):
            self.emit("PUSH", node.value)

        elif isinstance(node, BooleanNode):
            self.emit("PUSH", node.value)

        elif isinstance(node, IdentifierNode):
            self.emit("LOAD", node.name)

        elif isinstance(node, UnaryNode):
            self.visit(node.operand)
            operator = node.operator.name
            if operator == "PLUS":
                pass
            elif operator == "MINUS":
                self.emit("NEG", None)
            elif operator == "NOT":
                self.emit("NOT", None)
            else:
                raise CompilerError(f"Unsupported unary operator: {node.operator}")

        elif isinstance(node, BinaryNode):
            self.visit(node.left)
            self.visit(node.right)
            operator = node.operator.name
            if operator not in BINARY_OPS:
                raise CompilerError(f"Unsupported binary operator: {node.operator}")
            self.emit(BINARY_OPS[operator], None)

        elif isinstance(node, VarDeclNode):
            if node.initializer is not None:
                self.visit(node.initializer)
            else:
                self.emit("PUSH", None)
            self.emit("STORE", node.name)

        elif isinstance(node, AssignNode):
            self.visit(node.value)
            self.emit("STORE", node.name)

        elif isinstance(node, PrintNode):
            self.visit(node.expression)
            self.emit("PRINT", None)

        elif isinstance(node, ReturnNode):
            if node.value is not None:
                self.visit(node.value)
            else:
                self.emit("PUSH", None)
            self.emit("RET", None)

        elif isinstance(node, BlockNode):
            for stmt in node.statements:
                self.visit(stmt)

        elif isinstance(node, ProgramNode):
            for stmt in node.statements:
                self.visit(stmt)

        elif isinstance(node, IfNode):
            label_else = Label("else")
            label_end = Label("if_end")

            self.visit(node.condition)
            self.emit("JZ", label_else if node.else_branch else label_end)
            self.visit(node.then_branch)

            if node.else_branch is not None:
                self.emit("JMP", label_end)
                self.mark_label(label_else)
                self.visit(node.else_branch)

            self.mark_label(label_end)

        elif isinstance(node, WhileNode):
            label_start = Label("while_start")
            label_end = Label("while_end")

            self.mark_label(label_start)
            self.visit(node.condition)
            self.emit("JZ", label_end)
            self.visit(node.body)
            self.emit("JMP", label_start)
            self.mark_label(label_end)

        elif isinstance(node, FunctionDefNode):
            label_after = Label(f"after_{node.name}")
            label_entry = Label(f"entry_{node.name}")

            self.emit("DEF_FN", (node.name, node.params, label_entry))
            self.emit("JMP", label_after)

            self.mark_label(label_entry)
            self.visit(node.body)
            self.emit("PUSH", None)
            self.emit("RET", None)

            self.mark_label(label_after)

        elif isinstance(node, CallNode):
            for arg in node.args:
                self.visit(arg)
            self.emit("CALL", (node.callee, len(node.args)))

        else:
            raise CompilerError(f"Unsupported AST node: {type(node).__name__}")

    def compile(self, node: Any) -> List[Tuple[str, Any]]:
        self.visit(node)

        # Resolve labels
        resolved: List[Tuple[str, Any]] = []
        for op, arg in self.instructions:
            if isinstance(arg, Label):
                if arg.target is None:
                    raise CompilerError(f"Unresolved label: {arg.name}")
                resolved.append((op, arg.target))
            elif isinstance(arg, tuple):
                resolved_tuple = tuple(
                    x.target if isinstance(x, Label) else x for x in arg
                )
                resolved.append((op, resolved_tuple))
            else:
                resolved.append((op, arg))

        return resolved


def compile_node(node):
    """Compile one AST node into flat Kolos bytecode."""
    if isinstance(node, NumberNode):
        return ["PUSH", node.value]

    if isinstance(node, StringNode):
        return ["PUSH", node.value]

    if isinstance(node, BooleanNode):
        return ["PUSH", node.value]

    if isinstance(node, IdentifierNode):
        return ["LOAD", node.name]

    if isinstance(node, UnaryNode):
        code = compile_node(node.operand)
        operator = node.operator.name

        if operator == "PLUS":
            return code
        if operator == "MINUS":
            return code + ["NEG"]
        if operator == "NOT":
            return code + ["NOT"]

        raise CompilerError(f"Unsupported unary operator: {node.operator}")

    if isinstance(node, BinaryNode):
        left = compile_node(node.left)
        right = compile_node(node.right)
        operator = node.operator.name

        if operator not in BINARY_OPS:
            raise CompilerError(f"Unsupported binary operator: {node.operator}")

        return left + right + [BINARY_OPS[operator]]

    if isinstance(node, VarDeclNode):
        code = compile_node(node.initializer) if node.initializer else ["PUSH", None]
        return code + ["STORE", node.name]

    if isinstance(node, AssignNode):
        return compile_node(node.value) + ["STORE", node.name]

    if isinstance(node, PrintNode):
        return compile_node(node.expression) + ["PRINT"]

    if isinstance(node, ReturnNode):
        code = compile_node(node.value) if node.value else ["PUSH", None]
        return code + ["RET"]

    if isinstance(node, CallNode):
        code = []
        for arg in node.args:
            code.extend(compile_node(arg))
        return code + ["CALL", (node.callee, len(node.args))]

    if isinstance(node, BlockNode):
        code = []
        for stmt in node.statements:
            code.extend(compile_node(stmt))
        return code

    if isinstance(node, ProgramNode):
        code = []
        for stmt in node.statements:
            code.extend(compile_node(stmt))
        return code

    raise CompilerError(f"Unsupported AST node: {type(node).__name__}")


def compile_ast(node):
    """Compile an AST into flat Kolos bytecode."""
    return compile_node(node)


def compile_program(node) -> List[Tuple[str, Any]]:
    """Compile an AST into VM instructions with resolved jumps."""
    compiler = Compiler()
    instructions = compiler.compile(node)
    instructions.append(("HALT", None))
    return instructions