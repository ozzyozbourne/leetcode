import unittest
from NextGreater1 import nextGreaterElement


class TestFact(unittest.TestCase):
    def test_one(self):
        self.assertEqual(
            [-1, 3, -1], nextGreaterElement([4, 1, 2], [1, 3, 4, 2])
        )

    def test_two(self):
        self.assertEqual(
            [3, -1], nextGreaterElement([2, 4], [1, 2, 3, 4])
        )


if __name__ == "__main__":
    unittest.main()
