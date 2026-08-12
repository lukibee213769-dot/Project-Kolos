import unittest

from runtime.bytecode import assemble, VM


class TestBytecodeVM(unittest.TestCase):
    def run_expr(self, expr):
        bc = assemble(expr)
        vm = VM(bc)
        return vm.run()

    def test_add_mul(self):
        self.assertEqual(self.run_expr('1+2*3'), 7)

    def test_pow(self):
        self.assertEqual(self.run_expr('2**10'), 1024)

    def test_unary(self):
        self.assertEqual(self.run_expr('-5 + 3'), -2)


if __name__ == '__main__':
    unittest.main()
