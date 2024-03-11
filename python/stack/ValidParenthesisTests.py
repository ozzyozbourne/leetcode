import unittest
from ValidParenthesis import isValid


class TestFact(unittest.TestCase):
    def test_one(self):
        self.assertEqual(True, isValid("()"))

    def test_two(self):
        self.assertEqual(True, isValid("()[]{}"))

    def test_three(self):
        self.assertEqual(False, isValid("(]"))


if __name__ == "__main__":
    unittest.main()
