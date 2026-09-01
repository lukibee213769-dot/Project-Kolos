# Bytecode Specification (Kolos)

This document specifies the bytecode instructions and format used by the Kolos Virtual Machine.

## Format
- A program is a sequential list of instruction tuples `(OPCODE, ARG)` where `ARG` is `None` for operations without an immediate argument.
- The execution model is stack-based with support for global variables, local scopes, and call frames.

## Instruction Set Architecture (ISA)

### 1. Stack & Literals
- `PUSH <value>`: Push a constant value (`int`, `float`, `bool`, `str`, `None`) onto the evaluation stack.
- `POP`: Pop and discard the top value from the stack.
- `DUP`: Duplicate the top value on the stack.

### 2. Arithmetic & Unary Operators
- `ADD`: Pop `b`, pop `a`, push `a + b`.
- `SUB`: Pop `b`, pop `a`, push `a - b`.
- `MUL`: Pop `b`, pop `a`, push `a * b`.
- `DIV`: Pop `b`, pop `a`, push `a / b` (raises `VMError` on division by zero).
- `POW`: Pop `b`, pop `a`, push `a ** b`.
- `MOD`: Pop `b`, pop `a`, push `a % b` (raises `VMError` on modulo by zero).
- `NEG`: Pop `val`, push `-val`.
- `NOT`: Pop `val`, push `not val`.

### 3. Comparisons & Logic
- `EQ`: Pop `b`, pop `a`, push `a == b`.
- `NE`: Pop `b`, pop `a`, push `a != b`.
- `LT`: Pop `b`, pop `a`, push `a < b`.
- `LE`: Pop `b`, pop `a`, push `a <= b`.
- `GT`: Pop `b`, pop `a`, push `a > b`.
- `GE`: Pop `b`, pop `a`, push `a >= b`.
- `AND`: Pop `b`, pop `a`, push `a and b`.
- `OR`: Pop `b`, pop `a`, push `a or b`.

### 4. Variables & Environment
- `STORE <name>`: Pop top value and store it into the current local scope (if in function) or global scope.
- `LOAD <name>`: Look up `name` in local scope, then global scope. Push value onto the stack.

### 5. Control Flow
- `JMP <target_ip>`: Unconditionally set instruction pointer `ip = target_ip`.
- `JZ <target_ip>`: Pop top value; if falsy (`0`, `False`, `None`, empty), set `ip = target_ip`.
- `JNZ <target_ip>`: Pop top value; if truthy, set `ip = target_ip`.

### 6. Functions & Call Frames
- `DEF_FN <(name, params, entry_ip)>`: Register a function with parameter list and entry instruction pointer.
- `CALL <(name, argc)>`: Pop `argc` arguments, push a new `CallFrame(return_ip, locals)` to `call_stack`, and jump to function entry point.
- `RET`: Pop return value, pop top call frame, restore caller `ip`, and push return value onto caller's stack.

### 7. System & I/O
- `PRINT`: Pop top value and print to stdout / log output.
- `HALT`: Terminate program execution.
