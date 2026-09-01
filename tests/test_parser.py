import unittest

from compilers.parser import (
    BinaryNode,
    BooleanNode,
    IdentifierNode,
    ParserError,
    UnaryNode,
    parse,
)
from compilers.lexer import TokenType


class TestParser(unittest.TestCase):

    def test_arithmetic_precedence(self):
        tree = parse("2 + 3 * 4")

        self.assertIsInstance(tree, BinaryNode)
        self.assertEqual(tree.operator, TokenType.PLUS)

        self.assertEqual(tree.left.value, 2)

        self.assertIsInstance(tree.right, BinaryNode)
        self.assertEqual(tree.right.operator, TokenType.STAR)
        self.assertEqual(tree.right.left.value, 3)
        self.assertEqual(tree.right.right.value, 4)

    def test_power(self):
        tree = parse("2 ** 10")

        self.assertIsInstance(tree, BinaryNode)
        self.assertEqual(tree.operator, TokenType.POWER)
        self.assertEqual(tree.left.value, 2)
        self.assertEqual(tree.right.value, 10)

    def test_identifier(self):
        tree = parse("x + 5")

        self.assertIsInstance(tree, BinaryNode)
        self.assertIsInstance(tree.left, IdentifierNode)
        self.assertEqual(tree.left.name, "x")

    def test_unary(self):
        tree = parse("-5")

        self.assertIsInstance(tree, UnaryNode)
        self.assertEqual(tree.operator, TokenType.MINUS)
        self.assertEqual(tree.operand.value, 5)

    def test_parentheses(self):
        tree = parse("(2 + 3) * 4")

        self.assertEqual(tree.operator, TokenType.STAR)
        self.assertEqual(tree.left.operator, TokenType.PLUS)

    def test_comparison(self):
        tree = parse("5 > 3")

        self.assertEqual(tree.operator, TokenType.GT)

    def test_logical(self):
        tree = parse("5 > 3 and 2 < 4")

        self.assertEqual(tree.operator, TokenType.AND)
        self.assertEqual(tree.left.operator, TokenType.GT)
        self.assertEqual(tree.right.operator, TokenType.LT)

    def test_not(self):
        tree = parse("not (5 > 3)")

        self.assertIsInstance(tree, UnaryNode)
        self.assertEqual(tree.operator, TokenType.NOT)

    def test_boolean(self):
        tree = parse("True")

        self.assertIsInstance(tree, BooleanNode)
        self.assertTrue(tree.value)

    def test_invalid_syntax(self):
        with self.assertRaises(ParserError):
            parse("2 +")

    def test_unexpected_token(self):
        with self.assertRaises(ParserError):
            parse("2 3")


if __name__ == "__main__":
    unittest.main()
