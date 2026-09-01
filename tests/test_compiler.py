import unittest

from compilers.parser import parse
from compilers.compiler import compile_ast


class TestCompiler(unittest.TestCase):

    def test_arithmetic(self):
        bytecode = compile_ast(parse("2 + 3 * 4"))

        self.assertEqual(
            bytecode,
            [
                "PUSH", 2,
                "PUSH", 3,
                "PUSH", 4,
                "MUL",
                "ADD",
            ],
        )

    def test_power(self):
        bytecode = compile_ast(parse("2 ** 10"))

        self.assertEqual(
            bytecode,
            [
                "PUSH", 2,
                "PUSH", 10,
                "POW",
            ],
        )

    def test_modulo(self):
        bytecode = compile_ast(parse("10 % 3"))

        self.assertEqual(
            bytecode,
            [
                "PUSH", 10,
                "PUSH", 3,
                "MOD",
            ],
        )

    def test_comparison(self):
        bytecode = compile_ast(parse("5 > 3"))

        self.assertEqual(
            bytecode,
            [
                "PUSH", 5,
                "PUSH", 3,
                "GT",
            ],
        )

    def test_logical(self):
        bytecode = compile_ast(parse("5 > 3 and 2 < 4"))

        self.assertEqual(
            bytecode,
            [
                "PUSH", 5,
                "PUSH", 3,
                "GT",
                "PUSH", 2,
                "PUSH", 4,
                "LT",
                "AND",
            ],
        )

    def test_boolean(self):
        bytecode = compile_ast(parse("True"))

        self.assertEqual(
            bytecode,
            [
                "PUSH", True,
            ],
        )

    def test_unary_minus(self):
        bytecode = compile_ast(parse("-5"))

        self.assertEqual(
            bytecode,
            [
                "PUSH", 5,
                "NEG",
            ],
        )

    def test_unary_not(self):
        bytecode = compile_ast(parse("not True"))

        self.assertEqual(
            bytecode,
            [
                "PUSH", True,
                "NOT",
            ],
        )

    def test_identifier(self):
        bytecode = compile_ast(parse("x"))
        self.assertEqual(bytecode, ["LOAD", "x"])

    def test_variable_declaration(self):
        bytecode = compile_ast(parse("let x = 42;"))
        self.assertEqual(bytecode, ["PUSH", 42, "STORE", "x"])

    def test_assignment(self):
        bytecode = compile_ast(parse("x = 100;"))
        self.assertEqual(bytecode, ["PUSH", 100, "STORE", "x"])

    def test_print(self):
        bytecode = compile_ast(parse("print 5;"))
        self.assertEqual(bytecode, ["PUSH", 5, "PRINT"])


if __name__ == "__main__":
    unittest.main()
