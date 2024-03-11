import unittest
from NextGreaterIV import secondGreaterElement


class TestFact(unittest.TestCase):
    def test_one(self):
        self.assertEqual([9, 6, 6, -1, -1], secondGreaterElement([2, 4, 0, 9, 6]))

    def test_two(self):
        self.assertEqual([-1, -1], secondGreaterElement([-3, -3]))


if __name__ == "__main__":
    unittest.main()

