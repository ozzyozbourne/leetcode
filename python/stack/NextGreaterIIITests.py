import unittest
from NextGreaterIII import nextGreaterElement

class TestFact(unittest.TestCase):
    def test_one(self):
        self.assertEqual(21, nextGreaterElement(12))

    def test_two(self):
        self.assertEqual(-1, nextGreaterElement(21))


if __name__ == "__main__":
    unittest.main()
