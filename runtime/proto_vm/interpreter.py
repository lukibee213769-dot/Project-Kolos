"""A tiny safe expression evaluator and REPL for the proto VM."""
import ast


class EvalError(Exception):
    pass


ALLOWED_NODES = (
    ast.Expression,
    ast.BinOp,
    ast.UnaryOp,
    ast.Constant,
    ast.Add,
    ast.Sub,
    ast.Mult,
    ast.Div,
    ast.Pow,
    ast.Mod,
    ast.USub,
    ast.UAdd,
    ast.Load,
    ast.Expr,
)


def _check_node(node):
    if not isinstance(node, ALLOWED_NODES):
        name = node.__class__.__name__
        raise EvalError(f"Disallowed node: {name}")
    for child in ast.iter_child_nodes(node):
        _check_node(child)


def eval_expr(expr: str):
    """Evaluate numeric expressions safely (supports basic ops)."""
    try:
        tree = ast.parse(expr, mode="eval")
    except SyntaxError as e:
        raise EvalError("Syntax error") from e
    _check_node(tree)
    # Compile and evaluate in empty namespaces
    code = compile(tree, filename="<expr>", mode="eval")
    return eval(code, {"__builtins__": {}}, {})


def repl():
    """Simple read-eval-print loop."""
    print("Kolos proto VM REPL. Type 'exit' or 'quit' to leave.")
    while True:
        try:
            s = input("kolos> ")
        except (EOFError, KeyboardInterrupt):
            print()
            break
        if not s:
            continue
        if s.strip() in ("exit", "quit"):
            break
        try:
            res = eval_expr(s)
        except EvalError as e:
            print("Error:", e)
            continue
        except Exception as e:
            print("Evaluation error:", e)
            continue
        print(res)


if __name__ == "__main__":
    repl()
