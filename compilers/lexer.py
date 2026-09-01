"""Lexer for the Kolos language."""

from dataclasses import dataclass
from enum import Enum, auto


class TokenType(Enum):
    NUMBER = auto()
    STRING = auto()
    IDENTIFIER = auto()

    PLUS = auto()
    MINUS = auto()
    STAR = auto()
    SLASH = auto()
    POWER = auto()
    MODULO = auto()

    LPAREN = auto()
    RPAREN = auto()
    LBRACE = auto()
    RBRACE = auto()

    SEMICOLON = auto()
    COMMA = auto()
    ASSIGN = auto()

    EQ = auto()
    NE = auto()
    LT = auto()
    LE = auto()
    GT = auto()
    GE = auto()

    AND = auto()
    OR = auto()
    NOT = auto()

    LET = auto()
    IF = auto()
    ELSE = auto()
    WHILE = auto()
    FN = auto()
    RETURN = auto()
    PRINT = auto()
    CLASS = auto()
    THIS = auto()

    DOT = auto()
    EOF = auto()


@dataclass(frozen=True)
class Token:
    type: TokenType
    value: str
    position: int


class LexerError(Exception):
    """Raised when invalid source code is encountered."""


class Lexer:
    """Convert Kolos source code into tokens."""

    def __init__(self, source: str):
        self.source = source
        self.position = 0

    def tokenize(self):
        tokens = []

        while self.position < len(self.source):
            char = self.source[self.position]

            if char.isspace():
                self.position += 1
                continue

            # Handle single-line comments (# or //)
            if char == "#" or self.source[self.position:self.position + 2] == "//":
                while self.position < len(self.source) and self.source[self.position] != "\n":
                    self.position += 1
                continue

            start = self.position

            # String literals ("..." or '...')
            if char in ('"', "'"):
                tokens.append(self._string(start, char))
                continue

            if char.isdigit():
                tokens.append(self._number(start))
                continue

            if char.isalpha() or char == "_":
                tokens.append(self._identifier(start))
                continue

            two_char = self.source[self.position:self.position + 2]

            if two_char == "**":
                tokens.append(Token(TokenType.POWER, two_char, start))
                self.position += 2
                continue

            if two_char == "==":
                tokens.append(Token(TokenType.EQ, two_char, start))
                self.position += 2
                continue

            if two_char == "!=":
                tokens.append(Token(TokenType.NE, two_char, start))
                self.position += 2
                continue

            if two_char == "<=":
                tokens.append(Token(TokenType.LE, two_char, start))
                self.position += 2
                continue

            if two_char == ">=":
                tokens.append(Token(TokenType.GE, two_char, start))
                self.position += 2
                continue

            single_tokens = {
                "+": TokenType.PLUS,
                "-": TokenType.MINUS,
                "*": TokenType.STAR,
                "/": TokenType.SLASH,
                "%": TokenType.MODULO,
                "(": TokenType.LPAREN,
                ")": TokenType.RPAREN,
                "{": TokenType.LBRACE,
                "}": TokenType.RBRACE,
                ";": TokenType.SEMICOLON,
                ",": TokenType.COMMA,
                "=": TokenType.ASSIGN,
                "<": TokenType.LT,
                ">": TokenType.GT,
                ".": TokenType.DOT,
            }

            token_type = single_tokens.get(char)

            if token_type is not None:
                tokens.append(Token(token_type, char, start))
                self.position += 1
                continue

            raise LexerError(
                f"Unexpected character {char!r} at position {start}"
            )

        tokens.append(Token(TokenType.EOF, "", self.position))
        return tokens

    def _string(self, start: int, quote: str):
        self.position += 1  # Skip opening quote
        str_chars = []
        while self.position < len(self.source) and self.source[self.position] != quote:
            if self.source[self.position] == "\\" and self.position + 1 < len(self.source):
                self.position += 1
                escape_char = self.source[self.position]
                if escape_char == "n":
                    str_chars.append("\n")
                elif escape_char == "t":
                    str_chars.append("\t")
                elif escape_char == "\\":
                    str_chars.append("\\")
                elif escape_char == quote:
                    str_chars.append(quote)
                else:
                    str_chars.append(escape_char)
            else:
                str_chars.append(self.source[self.position])
            self.position += 1

        if self.position >= len(self.source):
            raise LexerError(f"Unterminated string literal at position {start}")

        self.position += 1  # Skip closing quote
        return Token(TokenType.STRING, "".join(str_chars), start)

    def _number(self, start):
        while (
            self.position < len(self.source)
            and self.source[self.position].isdigit()
        ):
            self.position += 1

        # Check for floating point
        if (
            self.position < len(self.source)
            and self.source[self.position] == "."
            and self.position + 1 < len(self.source)
            and self.source[self.position + 1].isdigit()
        ):
            self.position += 1
            while (
                self.position < len(self.source)
                and self.source[self.position].isdigit()
            ):
                self.position += 1

        return Token(
            TokenType.NUMBER,
            self.source[start:self.position],
            start,
        )

    def _identifier(self, start):
        while (
            self.position < len(self.source)
            and (
                self.source[self.position].isalnum()
                or self.source[self.position] == "_"
            )
        ):
            self.position += 1

        value = self.source[start:self.position]

        keywords = {
            "and": TokenType.AND,
            "or": TokenType.OR,
            "not": TokenType.NOT,
            "let": TokenType.LET,
            "if": TokenType.IF,
            "else": TokenType.ELSE,
            "while": TokenType.WHILE,
            "fn": TokenType.FN,
            "return": TokenType.RETURN,
            "print": TokenType.PRINT,
            "class": TokenType.CLASS,
            "this": TokenType.THIS,
        }

        token_type = keywords.get(value, TokenType.IDENTIFIER)

        return Token(token_type, value, start)
