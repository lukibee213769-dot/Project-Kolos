import unittest
from runtime.proto_vm.pipeline import execute_source


class TestFunctions(unittest.TestCase):
    def test_simple_function(self):
        src = """
        fn double(n) {
            return n * 2;
        }
        double(21);
        """
        self.assertEqual(execute_source(src), 42)

    def test_multiple_args(self):
        src = """
        fn add3(a, b, c) {
            return a + b + c;
        }
        add3(10, 20, 30);
        """
        self.assertEqual(execute_source(src), 60)

    def test_recursive_factorial(self):
        src = """
        fn fact(n) {
            if n <= 1 {
                return 1;
            } else {
                return n * fact(n - 1);
            }
        }
        fact(5);
        """
        self.assertEqual(execute_source(src), 120)

    def test_recursive_fibonacci(self):
        src = """
        fn fib(n) {
            if n <= 1 {
                return n;
            }
            return fib(n - 1) + fib(n - 2);
        }
        fib(7);
        """
        self.assertEqual(execute_source(src), 13)

    def test_function_local_scope(self):
        src = """
        let x = 100;
        fn update() {
            let x = 50;
            return x;
        }
        let res = update();
        res + x;
        """
        self.assertEqual(execute_source(src), 150)


if __name__ == '__main__':
    unittest.main()
