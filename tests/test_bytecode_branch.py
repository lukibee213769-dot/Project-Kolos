import unittest

from runtime.bytecode import assemble_text, VM


class TestBranching(unittest.TestCase):
    def test_jz_branch(self):
        asm = """
PUSH 0
JZ is_zero
PUSH 1
JMP end
is_zero:
PUSH 2
end:
HALT
"""
        bc = assemble_text(asm)
        vm = VM(bc)
        self.assertEqual(vm.run(), 2)


if __name__ == '__main__':
    unittest.main()
