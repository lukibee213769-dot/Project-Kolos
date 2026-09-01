"""Code formatter for Kolos language files."""

import sys
from pathlib import Path


def format_code(source: str) -> str:
    lines = source.splitlines()
    formatted_lines = []
    indent_level = 0
    indent_str = "    "

    for line in lines:
        stripped = line.strip()
        if not stripped:
            formatted_lines.append("")
            continue

        # Reduce indent for closing braces
        if stripped.startswith("}"):
            indent_level = max(0, indent_level - 1)

        formatted_lines.append(f"{indent_str * indent_level}{stripped}")

        # Increase indent for opening braces that don't close on the same line
        opens = stripped.count("{")
        closes = stripped.count("}")
        net = opens - closes
        if net > 0:
            if not stripped.startswith("}"):
                indent_level += net

    return "\n".join(formatted_lines).strip() + "\n"


def format_file(path: str):
    p = Path(path)
    if not p.exists():
        raise FileNotFoundError(f"File not found: {path}")
    with open(p, "r", encoding="utf-8") as f:
        content = f.read()
    formatted = format_code(content)
    with open(p, "w", encoding="utf-8") as f:
        f.write(formatted)
    print(f"Formatted {path}")


if __name__ == "__main__":
    if len(sys.argv) > 1:
        format_file(sys.argv[1])
    else:
        print("Usage: python formatter.py <file.kolos>")
