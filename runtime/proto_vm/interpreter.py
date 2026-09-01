"""A tiny safe expression evaluator and REPL for the proto VM."""
import ast
import platform
import time


VERSION = "v0.0.1"


class EvalError(Exception):
    pass


ALLOWED_NODES = (
    ast.Expression,
    ast.BinOp,
    ast.UnaryOp,
    ast.BoolOp,
    ast.Compare,
    ast.Constant,
    ast.Add,
    ast.Sub,
    ast.Mult,
    ast.Div,
    ast.Pow,
    ast.Mod,
    ast.USub,
    ast.UAdd,
    ast.And,
    ast.Or,
    ast.Not,
    ast.Eq,
    ast.NotEq,
    ast.Lt,
    ast.LtE,
    ast.Gt,
    ast.GtE,
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
    """Evaluate arithmetic, comparison, and logical expressions safely."""
    try:
        tree = ast.parse(expr, mode="eval")
    except SyntaxError as e:
        raise EvalError("Syntax error") from e

    _check_node(tree)

    code = compile(tree, filename="<expr>", mode="eval")
    return eval(code, {"__builtins__": {}}, {})


def print_help():
    print("""
Kolos proto VM - commands
-------------------------
help       Show this help
version    Show Kolos version
info       Show runtime information
status     Show runtime status
diagnostic Show detailed runtime diagnostics
about      About Kolos
history    Show command history
clear      Clear the console
reset      Reset REPL state
time       Show last execution time
exit       Exit Kolos
quit       Exit Kolos

Expressions:
  +        Addition
  -        Subtraction
  *        Multiplication
  /        Division
  **       Power
  %        Modulo

Comparisons:
  ==       Equal
  !=       Not equal
  <        Less than
  <=       Less than or equal
  >        Greater than
  >=       Greater than or equal

Logic:
  and      Logical AND
  or       Logical OR
  not      Logical NOT
""")


def print_version():
    print(f"Kolos proto VM {VERSION}")


def print_info():
    print(f"""
Kolos proto VM
--------------
Version: {VERSION}
Runtime: Python
Evaluator: AST safe evaluator
Mode: REPL
Arithmetic: enabled
Comparisons: enabled
Logic: enabled
Security: restricted AST
""")


def print_status():
    print(f"""
Kolos Runtime Status
--------------------
Version:    {VERSION}
Runtime:    ONLINE
REPL:       ONLINE
Evaluator:  ONLINE
Security:   ENABLED
""")


def print_diagnostic(execution_count):
    print(f"""
Kolos Runtime Diagnostic
------------------------
Version:     {VERSION}
Python:      {platform.python_version()}
Platform:    {platform.system()}
Runtime:     ONLINE
REPL:        ONLINE
Evaluator:   ONLINE
Security:    ENABLED
Executions:  {execution_count}
""")


def print_about():
    print(f"""
Kolos
-----
Version: {VERSION}
Type: Modular experimental runtime
Component: Proto VM
Purpose: Experimental language/runtime development
""")


def print_history(history):
    if not history:
        print("History is empty.")
        return

    print("Kolos Command History")
    print("---------------------")

    for index, command in enumerate(history, start=1):
        print(f"{index}: {command}")


def clear_screen():
    print("\033[2J\033[H", end="")


def reset_repl(history):
    history.clear()
    print("REPL state reset.")


def repl():
    """Read-eval-print loop."""
    print(f"Kolos proto VM REPL {VERSION}. Type 'help' for commands.")

    history = []
    last_execution_time = None
    execution_count = 0

    while True:
        try:
            s = input("kolos> ")
        except (EOFError, KeyboardInterrupt):
            print()
            break

        s = s.strip()

        if not s:
            continue

        if s in ("exit", "quit"):
            break

        if s == "help":
            history.append(s)
            print_help()
            continue

        if s == "version":
            history.append(s)
            print_version()
            continue

        if s == "info":
            history.append(s)
            print_info()
            continue

        if s == "status":
            history.append(s)
            print_status()
            continue

        if s == "diagnostic":
            history.append(s)
            print_diagnostic(execution_count)
            continue

        if s == "about":
            history.append(s)
            print_about()
            continue

        if s == "history":
            history.append(s)
            print_history(history[:-1])
            continue

        if s == "clear":
            history.append(s)
            clear_screen()
            continue

        if s == "reset":
            reset_repl(history)
            last_execution_time = None
            execution_count = 0
            continue

        if s == "time":
            if last_execution_time is None:
                print("No expression has been executed yet.")
            else:
                print(f"Last execution time: {last_execution_time:.9f} s")
            continue

        history.append(s)

        start = time.perf_counter()

        try:
            res = eval_expr(s)
        except EvalError as e:
            print("Error:", e)
            last_execution_time = None
            continue
        except Exception as e:
            print("Evaluation error:", e)
            last_execution_time = None
            continue

        last_execution_time = time.perf_counter() - start
        execution_count += 1

        print(res)


if __name__ == "__main__":
    repl()