import unittest
from runtime.proto_vm.pipeline import execute_source


class TestStatements(unittest.TestCase):
    def test_let_and_assign(self):
        src = "let x = 10; let y = 20; let z = x + y; z;"
        self.assertEqual(execute_source(src), 30)

    def test_reassignment(self):
        src = "let a = 5; a = a * 2; a = a + 3; a;"
        self.assertEqual(execute_source(src), 13)

    def test_block(self):
        src = "let a = 1; { let b = 2; a = a + b; } a;"
        self.assertEqual(execute_source(src), 3)


if __name__ == '__main__':
    unittest.main()
