import unittest
from io import StringIO
from contextlib import redirect_stdout

from runtime.proto_vm import interpreter


class TestProtoVM(unittest.TestCase):
    def test_eval_simple(self):
        self.assertEqual(interpreter.eval_expr('1+2*3'), 7)

    def test_eval_pow(self):
        self.assertEqual(interpreter.eval_expr('2**10'), 1024)

    def test_eval_unary(self):
        self.assertEqual(interpreter.eval_expr('-5 + 3'), -2)

    def test_syntax_error(self):
        with self.assertRaises(interpreter.EvalError):
            interpreter.eval_expr('import os')

    def test_diagnostic(self):
        output = StringIO()

        with redirect_stdout(output):
            interpreter.print_diagnostic(7)

        result = output.getvalue()

        self.assertIn("Kolos Runtime Diagnostic", result)
        self.assertIn("Version:     v0.0.1", result)
        self.assertIn("Runtime:     ONLINE", result)
        self.assertIn("REPL:        ONLINE", result)
        self.assertIn("Evaluator:   ONLINE", result)
        self.assertIn("Security:    ENABLED", result)
        self.assertIn("Executions:  7", result)

    def test_comparisons(self):
        self.assertTrue(interpreter.eval_expr('5 > 3'))
        self.assertFalse(interpreter.eval_expr('5 < 3'))
        self.assertTrue(interpreter.eval_expr('5 == 5'))
        self.assertTrue(interpreter.eval_expr('5 != 4'))
        self.assertTrue(interpreter.eval_expr('5 >= 5'))
        self.assertTrue(interpreter.eval_expr('5 <= 6'))

    def test_logical_and_or(self):
        self.assertTrue(interpreter.eval_expr('5 > 3 and 2 < 4'))
        self.assertFalse(interpreter.eval_expr('5 > 3 and 2 > 4'))
        self.assertTrue(interpreter.eval_expr('5 > 3 or 2 > 10'))
        self.assertTrue(interpreter.eval_expr('5 < 3 or 2 < 10'))

    def test_logical_not(self):
        self.assertFalse(interpreter.eval_expr('not (5 > 3)'))
        self.assertTrue(interpreter.eval_expr('not (5 < 3)'))


if __name__ == '__main__':
    unittest.main()
