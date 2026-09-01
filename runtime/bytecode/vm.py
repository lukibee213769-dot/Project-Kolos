"""Stack-based bytecode virtual machine with call frames and variables."""

from dataclasses import dataclass
from typing import Any, Dict, List, Tuple


class VMError(Exception):
    """Raised when the bytecode VM encounters an error."""
    pass


@dataclass
class CallFrame:
    return_ip: int
    locals: Dict[str, Any]


class VM:
    def __init__(self, bytecode: List[Tuple[str, Any]]):
        self.bytecode = bytecode
        self.ip = 0
        self.stack: List[Any] = []
        self.globals: Dict[str, Any] = {}
        self.functions: Dict[str, Tuple[List[str], int]] = {}
        self.call_stack: List[CallFrame] = []
        self.stdout_log: List[str] = []

    def step(self):
        if self.ip >= len(self.bytecode):
            raise VMError("IP out of range")

        instr, arg = self.bytecode[self.ip]
        self.ip += 1

        if instr == "PUSH":
            self.stack.append(arg)

        elif instr == "DUP":
            if not self.stack:
                raise VMError("DUP on empty stack")
            self.stack.append(self.stack[-1])

        elif instr == "STORE":
            if not self.stack:
                raise VMError("STORE on empty stack")
            value = self.stack.pop()
            if self.call_stack:
                self.call_stack[-1].locals[arg] = value
            else:
                self.globals[arg] = value

        elif instr == "LOAD":
            found = False
            value = None
            if self.call_stack and arg in self.call_stack[-1].locals:
                value = self.call_stack[-1].locals[arg]
                found = True
            elif arg in self.globals:
                value = self.globals[arg]
                found = True

            if not found:
                raise VMError(f"Undefined variable: {arg}")
            self.stack.append(value)

        elif instr == "DEF_FN":
            name, params, target_ip = arg
            self.functions[name] = (params, target_ip)

        elif instr == "CALL":
            name, argc = arg
            if name not in self.functions:
                raise VMError(f"Undefined function: {name}")
            params, target_ip = self.functions[name]
            if argc != len(params):
                raise VMError(
                    f"Function '{name}' expected {len(params)} args, got {argc}"
                )

            args_val = [self.stack.pop() for _ in range(argc)][::-1]
            frame_locals = dict(zip(params, args_val))
            self.call_stack.append(
                CallFrame(return_ip=self.ip, locals=frame_locals)
            )
            self.ip = target_ip

        elif instr == "RET":
            ret_val = self.stack.pop() if self.stack else None
            if not self.call_stack:
                self.stack.append(ret_val)
                return "HALT"
            frame = self.call_stack.pop()
            self.ip = frame.return_ip
            self.stack.append(ret_val)

        elif instr == "ADD":
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a + b)

        elif instr == "SUB":
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a - b)

        elif instr == "MUL":
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a * b)

        elif instr == "DIV":
            b = self.stack.pop()
            a = self.stack.pop()

            if b == 0:
                raise VMError("Division by zero")

            self.stack.append(a / b)

        elif instr == "POW":
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a ** b)

        elif instr == "MOD":
            b = self.stack.pop()
            a = self.stack.pop()

            if b == 0:
                raise VMError("Modulo by zero")

            self.stack.append(a % b)

        elif instr == "NEG":
            value = self.stack.pop()
            self.stack.append(-value)

        elif instr == "NOT":
            value = self.stack.pop()
            self.stack.append(not value)

        elif instr == "EQ":
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a == b)

        elif instr == "NE":
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a != b)

        elif instr == "LT":
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a < b)

        elif instr == "LE":
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a <= b)

        elif instr == "GT":
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a > b)

        elif instr == "GE":
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a >= b)

        elif instr == "AND":
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a and b)

        elif instr == "OR":
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a or b)

        elif instr == "POP":
            if not self.stack:
                raise VMError("POP on empty stack")
            self.stack.pop()

        elif instr == "PRINT":
            if not self.stack:
                raise VMError("PRINT on empty stack")
            value = self.stack.pop()
            self.stdout_log.append(str(value))
            print(value)

        elif instr == "JMP":
            self.ip = arg

        elif instr == "JZ":
            if not self.stack:
                raise VMError("JZ on empty stack")

            value = self.stack.pop()

            if not value:
                self.ip = arg

        elif instr == "JNZ":
            if not self.stack:
                raise VMError("JNZ on empty stack")

            value = self.stack.pop()

            if value:
                self.ip = arg

        elif instr == "HALT":
            return "HALT"

        else:
            raise VMError(f"Unknown instr {instr}")

    def run(self):
        while True:
            result = self.step()

            if result == "HALT":
                break

        if not self.stack:
            return None

        return self.stack[-1]