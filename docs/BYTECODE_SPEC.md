# Bytecode Specification (Kolos)

This document specifies the simple bytecode format used by the proto runtime.

Format
- A program is a list of instructions. Each instruction is a tuple `(OP, ARG)` where `ARG` may be `None`.
- The assembler `assemble_text` supports a small assembly language with labels.

Instructions
- `PUSH <value>`: push a literal value (number) onto the stack.
- `POP`: drop the top value from the stack.
- `DUP`: duplicate the top value on the stack.
- `ADD`, `SUB`, `MUL`, `DIV`, `POW`, `MOD`: binary arithmetic operators; they pop two values and push the result.
- `PRINT`: pop and print the top value.
- `JMP <target>`: unconditional jump to instruction index `target`.
- `JZ <target>`: pop top; if zero jump to `target`.
- `HALT`: terminate program.

Assembly language
- Labels: `label:` placed at start of a line; can be target of `JMP`/`JZ`.
- Comments: `#` starts a comment to end of line.

Notes
- This is intentionally minimal for the prototype. Future extensions may include:
  - Local variables and LOAD/STORE instructions
  - Function call / frames
  - Typed values and runtime type system
  - Binary encoding for compact bytecode
