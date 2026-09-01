import unittest

from compilers.lexer import Lexer, LexerError, TokenType


class TestLexer(unittest.TestCase):
    def test_arithmetic(self):
        tokens = Lexer("2 + 3 * 4").tokenize()

        self.assertEqual(
            [token.type for token in tokens],
            [
                TokenType.NUMBER,
                TokenType.PLUS,
                TokenType.NUMBER,
                TokenType.STAR,
                TokenType.NUMBER,
                TokenType.EOF,
            ],
        )

    def test_comparisons_and_logic(self):
        tokens = Lexer("5 > 3 and 2 < 4").tokenize()

        self.assertEqual(
            [token.type for token in tokens],
            [
                TokenType.NUMBER,
                TokenType.GT,
                TokenType.NUMBER,
                TokenType.AND,
                TokenType.NUMBER,
                TokenType.LT,
                TokenType.NUMBER,
                TokenType.EOF,
            ],
        )

    def test_identifier_and_power(self):
        tokens = Lexer("x ** 2 + 10").tokenize()

        self.assertEqual(
            [token.type for token in tokens],
            [
                TokenType.IDENTIFIER,
                TokenType.POWER,
                TokenType.NUMBER,
                TokenType.PLUS,
                TokenType.NUMBER,
                TokenType.EOF,
            ],
        )

    def test_keywords(self):
        tokens = Lexer("and or not").tokenize()

        self.assertEqual(
            [token.type for token in tokens],
            [
                TokenType.AND,
                TokenType.OR,
                TokenType.NOT,
                TokenType.EOF,
            ],
        )

    def test_invalid_character(self):
        with self.assertRaises(LexerError):
            Lexer("2 + @").tokenize()


if __name__ == "__main__":
    unittest.main()
