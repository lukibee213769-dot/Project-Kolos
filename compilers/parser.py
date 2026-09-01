"""Recursive-descent parser for the Kolos language."""

from dataclasses import dataclass
from typing import List, Optional

from compilers.lexer import Lexer, Token, TokenType


class ParserError(Exception):
    """Raised when source code cannot be parsed."""


@dataclass
class NumberNode:
    value: int | float


@dataclass
class StringNode:
    value: str


@dataclass
class BooleanNode:
    value: bool


@dataclass
class IdentifierNode:
    name: str


@dataclass
class UnaryNode:
    operator: TokenType
    operand: object


@dataclass
class BinaryNode:
    left: object
    operator: TokenType
    right: object


@dataclass
class VarDeclNode:
    name: str
    initializer: Optional[object]


@dataclass
class AssignNode:
    name: str
    value: object


@dataclass
class BlockNode:
    statements: List[object]


@dataclass
class IfNode:
    condition: object
    then_branch: object
    else_branch: Optional[object] = None


@dataclass
class WhileNode:
    condition: object
    body: object


@dataclass
class FunctionDefNode:
    name: str
    params: List[str]
    body: object


@dataclass
class ReturnNode:
    value: Optional[object]


@dataclass
class CallNode:
    callee: str
    args: List[object]


@dataclass
class PrintNode:
    expression: object


@dataclass
class ProgramNode:
    statements: List[object]


class Parser:
    def __init__(self, source: str):
        self.tokens = Lexer(source).tokenize()
        self.position = 0

    def current(self) -> Token:
        return self.tokens[self.position]

    def advance(self) -> Token:
        token = self.current()
        self.position += 1
        return token

    def match(self, *types: TokenType) -> bool:
        if self.current().type in types:
            self.advance()
            return True
        return False

    def expect(self, token_type: TokenType) -> Token:
        token = self.current()

        if token.type != token_type:
            raise ParserError(
                f"Expected {token_type.name}, "
                f"got {token.type.name} at position {token.position}"
            )

        return self.advance()

    def parse(self):
        statements = []
        while self.current().type != TokenType.EOF:
            stmt = self.parse_statement()
            statements.append(stmt)

        if len(statements) == 1 and not isinstance(
            statements[0],
            (
                VarDeclNode,
                AssignNode,
                BlockNode,
                IfNode,
                WhileNode,
                FunctionDefNode,
                ReturnNode,
                PrintNode,
            ),
        ):
            return statements[0]

        if len(statements) == 1 and isinstance(statements[0], ProgramNode):
            return statements[0]

        return ProgramNode(statements)

    def parse_statement(self):
        stmt = None
        needs_semicolon = True

        if self.current().type == TokenType.LET:
            stmt = self.parse_let()
        elif self.current().type == TokenType.IF:
            stmt = self.parse_if()
            needs_semicolon = False
        elif self.current().type == TokenType.WHILE:
            stmt = self.parse_while()
            needs_semicolon = False
        elif self.current().type == TokenType.FN:
            stmt = self.parse_fn()
            needs_semicolon = False
        elif self.current().type == TokenType.RETURN:
            stmt = self.parse_return()
        elif self.current().type == TokenType.PRINT:
            stmt = self.parse_print()
        elif self.current().type == TokenType.LBRACE:
            stmt = self.parse_block()
            needs_semicolon = False
        elif (
            self.current().type == TokenType.IDENTIFIER
            and self.position + 1 < len(self.tokens)
            and self.tokens[self.position + 1].type == TokenType.ASSIGN
        ):
            name = self.advance().value
            self.expect(TokenType.ASSIGN)
            value = self.parse_or()
            stmt = AssignNode(name, value)
        else:
            stmt = self.parse_or()

        if needs_semicolon:
            if self.match(TokenType.SEMICOLON):
                while self.match(TokenType.SEMICOLON):
                    pass
            elif self.current().type not in (TokenType.EOF, TokenType.RBRACE):
                raise ParserError(
                    f"Expected ';' after statement, got {self.current().type.name} "
                    f"at position {self.current().position}"
                )
        else:
            while self.match(TokenType.SEMICOLON):
                pass

        return stmt

    def parse_let(self):
        self.expect(TokenType.LET)
        name = self.expect(TokenType.IDENTIFIER).value
        initializer = None
        if self.match(TokenType.ASSIGN):
            initializer = self.parse_or()
        return VarDeclNode(name, initializer)

    def parse_if(self):
        self.expect(TokenType.IF)
        condition = self.parse_or()
        then_branch = self.parse_block()
        else_branch = None
        if self.match(TokenType.ELSE):
            if self.current().type == TokenType.IF:
                else_branch = self.parse_if()
            else:
                else_branch = self.parse_block()
        return IfNode(condition, then_branch, else_branch)

    def parse_while(self):
        self.expect(TokenType.WHILE)
        condition = self.parse_or()
        body = self.parse_block()
        return WhileNode(condition, body)

    def parse_fn(self):
        self.expect(TokenType.FN)
        name = self.expect(TokenType.IDENTIFIER).value
        self.expect(TokenType.LPAREN)
        params = []
        if self.current().type != TokenType.RPAREN:
            params.append(self.expect(TokenType.IDENTIFIER).value)
            while self.match(TokenType.COMMA):
                params.append(self.expect(TokenType.IDENTIFIER).value)
        self.expect(TokenType.RPAREN)
        body = self.parse_block()
        return FunctionDefNode(name, params, body)

    def parse_return(self):
        self.expect(TokenType.RETURN)
        if self.current().type in (TokenType.SEMICOLON, TokenType.RBRACE, TokenType.EOF):
            value = None
        else:
            value = self.parse_or()
        return ReturnNode(value)

    def parse_print(self):
        self.expect(TokenType.PRINT)
        expr = self.parse_or()
        return PrintNode(expr)

    def parse_block(self):
        self.expect(TokenType.LBRACE)
        statements = []
        while (
            self.current().type != TokenType.RBRACE
            and self.current().type != TokenType.EOF
        ):
            statements.append(self.parse_statement())
        self.expect(TokenType.RBRACE)
        return BlockNode(statements)

    def parse_or(self):
        node = self.parse_and()

        while self.match(TokenType.OR):
            operator = self.tokens[self.position - 1]
            right = self.parse_and()
            node = BinaryNode(node, operator.type, right)

        return node

    def parse_and(self):
        node = self.parse_comparison()

        while self.match(TokenType.AND):
            operator = self.tokens[self.position - 1]
            right = self.parse_comparison()
            node = BinaryNode(node, operator.type, right)

        return node

    def parse_comparison(self):
        node = self.parse_additive()

        while self.current().type in (
            TokenType.EQ,
            TokenType.NE,
            TokenType.LT,
            TokenType.LE,
            TokenType.GT,
            TokenType.GE,
        ):
            operator = self.advance()
            right = self.parse_additive()
            node = BinaryNode(node, operator.type, right)

        return node

    def parse_additive(self):
        node = self.parse_multiplicative()

        while self.current().type in (
            TokenType.PLUS,
            TokenType.MINUS,
        ):
            operator = self.advance()
            right = self.parse_multiplicative()
            node = BinaryNode(node, operator.type, right)

        return node

    def parse_multiplicative(self):
        node = self.parse_power()

        while self.current().type in (
            TokenType.STAR,
            TokenType.SLASH,
            TokenType.MODULO,
        ):
            operator = self.advance()
            right = self.parse_power()
            node = BinaryNode(node, operator.type, right)

        return node

    def parse_power(self):
        node = self.parse_unary()

        if self.match(TokenType.POWER):
            operator = self.tokens[self.position - 1]
            right = self.parse_power()
            node = BinaryNode(node, operator.type, right)

        return node

    def parse_unary(self):
        if self.current().type in (
            TokenType.PLUS,
            TokenType.MINUS,
            TokenType.NOT,
        ):
            operator = self.advance()
            operand = self.parse_unary()
            return UnaryNode(operator.type, operand)

        return self.parse_primary()

    def parse_primary(self):
        token = self.current()

        if self.match(TokenType.LPAREN):
            node = self.parse_or()
            self.expect(TokenType.RPAREN)
            return node

        if self.match(TokenType.STRING):
            return StringNode(token.value)

        if self.match(TokenType.NUMBER):
            if "." in token.value:
                return NumberNode(float(token.value))
            return NumberNode(int(token.value))

        if token.type == TokenType.IDENTIFIER:
            if token.value == "True":
                self.advance()
                return BooleanNode(True)

            if token.value == "False":
                self.advance()
                return BooleanNode(False)

            name = self.advance().value
            if self.match(TokenType.LPAREN):
                args = []
                if self.current().type != TokenType.RPAREN:
                    args.append(self.parse_or())
                    while self.match(TokenType.COMMA):
                        args.append(self.parse_or())
                self.expect(TokenType.RPAREN)
                return CallNode(name, args)

            return IdentifierNode(name)

        raise ParserError(
            f"Unexpected token {token.type.name} "
            f"at position {token.position}"
        )


def parse(source: str):
    """Parse Kolos source code into an AST."""
    return Parser(source).parse()