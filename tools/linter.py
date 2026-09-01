"""Linter and syntax checker for Kolos source code."""

import sys
from pathlib import Path
from compilers.lexer import Lexer, LexerError
from compilers.parser import Parser, ParserError


def lint_source(source: str) -> list[str]:
    issues = []
    try:
        Lexer(source).tokenize()
    except LexerError as e:
        issues.append(f"Lexer Error: {e}")
        return issues

    try:
        Parser(source).parse()
    except ParserError as e:
        issues.append(f"Parser Error: {e}")

    return issues


def lint_file(path: str) -> bool:
    p = Path(path)
    if not p.exists():
        print(f"Error: file not found {path}")
        return False

    with open(p, "r", encoding="utf-8") as f:
        source = f.read()

    issues = lint_source(source)
    if not issues:
        print(f"[OK] {path} passed lint checks without issues.")
        return True
    else:
        print(f"[FAIL] {path} has {len(issues)} issue(s):")
        for issue in issues:
            print(f"  - {issue}")
        return False


if __name__ == "__main__":
    if len(sys.argv) > 1:
        success = lint_file(sys.argv[1])
        sys.exit(0 if success else 1)
    else:
        print("Usage: python linter.py <file.kolos>")
