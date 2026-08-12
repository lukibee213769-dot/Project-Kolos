"""Simple stack-based bytecode virtual machine."""
from typing import List, Tuple


class VMError(Exception):
    pass


class VM:
    def __init__(self, bytecode: List[Tuple[str, object]]):
        self.bytecode = bytecode
        self.ip = 0
        self.stack = []

    def step(self):
        if self.ip >= len(self.bytecode):
            raise VMError("IP out of range")
        instr, arg = self.bytecode[self.ip]
        self.ip += 1
        if instr == 'PUSH':
            self.stack.append(arg)
        elif instr == 'DUP':
            self.stack.append(self.stack[-1])
        elif instr == 'ADD':
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a + b)
        elif instr == 'SUB':
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a - b)
        elif instr == 'MUL':
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a * b)
        elif instr == 'DIV':
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a / b)
        elif instr == 'POW':
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a ** b)
        elif instr == 'MOD':
            b = self.stack.pop()
            a = self.stack.pop()
            self.stack.append(a % b)
        elif instr == 'POP':
            self.stack.pop()
        elif instr == 'PRINT':
            val = self.stack.pop()
            print(val)
        elif instr == 'JMP':
            # arg is target ip
            self.ip = arg
        elif instr == 'JZ':
            # pop top, if zero jump
            v = self.stack.pop()
            if v == 0:
                self.ip = arg
        elif instr == 'HALT':
            return 'HALT'
        else:
            raise VMError(f"Unknown instr {instr}")

    def run(self):
        while True:
            res = self.step()
            if res == 'HALT':
                break
        if not self.stack:
            return None
        return self.stack[-1]
