import unittest

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


if __name__ == '__main__':
    unittest.main()
