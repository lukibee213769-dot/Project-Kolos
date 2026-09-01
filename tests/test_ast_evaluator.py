import unittest

from compilers.parser import parse
from runtime.proto_vm.ast_evaluator import (
    evaluate,
    EvaluationError,
)


class TestASTEvaluator(unittest.TestCase):

    def test_arithmetic(self):
        self.assertEqual(evaluate(parse("2 + 3 * 4")), 14)

    def test_power(self):
        self.assertEqual(evaluate(parse("2 ** 10")), 1024)

    def test_unary(self):
        self.assertEqual(evaluate(parse("-5 + 3")), -2)

    def test_comparison(self):
        self.assertTrue(evaluate(parse("5 > 3")))
        self.assertFalse(evaluate(parse("5 < 3")))

    def test_logical(self):
        self.assertTrue(
            evaluate(parse("5 > 3 and 2 < 4"))
        )

    def test_logical_not(self):
        self.assertFalse(
            evaluate(parse("not (5 > 3)"))
        )

    def test_boolean(self):
        self.assertTrue(evaluate(parse("True")))
        self.assertFalse(evaluate(parse("False")))

    def test_identifier(self):
        self.assertEqual(
            evaluate(parse("x + 5"), {"x": 10}),
            15,
        )

    def test_undefined_variable(self):
        with self.assertRaises(EvaluationError):
            evaluate(parse("missing"))

    def test_modulo(self):
        self.assertEqual(
            evaluate(parse("10 % 3")),
            1,
        )


if __name__ == "__main__":
    unittest.main()
