import unittest
from runtime.proto_vm.pipeline import execute_source


class TestControlFlow(unittest.TestCase):
    def test_if_true_branch(self):
        src = """
        let x = 10;
        let res = 0;
        if x > 5 {
            res = 100;
        } else {
            res = 200;
        }
        res;
        """
        self.assertEqual(execute_source(src), 100)

    def test_if_false_branch(self):
        src = """
        let x = 3;
        let res = 0;
        if x > 5 {
            res = 100;
        } else {
            res = 200;
        }
        res;
        """
        self.assertEqual(execute_source(src), 200)

    def test_if_without_else(self):
        src = """
        let x = 10;
        let res = 0;
        if x > 5 {
            res = 42;
        }
        res;
        """
        self.assertEqual(execute_source(src), 42)

    def test_while_loop(self):
        src = """
        let sum = 0;
        let i = 1;
        while i <= 10 {
            sum = sum + i;
            i = i + 1;
        }
        sum;
        """
        self.assertEqual(execute_source(src), 55)

    def test_nested_while_loop(self):
        src = """
        let total = 0;
        let i = 0;
        while i < 3 {
            let j = 0;
            while j < 3 {
                total = total + 1;
                j = j + 1;
            }
            i = i + 1;
        }
        total;
        """
        self.assertEqual(execute_source(src), 9)


if __name__ == '__main__':
    unittest.main()
