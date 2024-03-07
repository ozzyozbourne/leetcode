import unittest
from stockspan import calculateSpan


class TestFact(unittest.TestCase):
    def test_one(self):
        self.assertEqual(
            [1, 1, 1, 2, 1, 4, 6], calculateSpan([100, 80, 60, 70, 60, 75, 85], 7)
        )


if __name__ == "__main__":
    unittest.main()
