import unittest
from nearest_greater_or_smaller import (
        nearest_greater_to_the_right as ngr,
        nearest_greater_to_the_left as ngl,
        nearest_smaller_to_the_right as nsr,
        nearest_smaller_to_the_left as nsl,
        ngr_mono
        )


class Test(unittest.TestCase):

    def test_one(self):
        self.assertEqual([3, 4, 4, -1], ngr([1, 3, 2, 4]))

    def test_two(self):
        self.assertEqual([-1, -1, -1, -1, -1], ngr([5, 4, 3, 2, 1]))

    def test_three(self):
        self.assertEqual([70, 80, 80, -1, -1], ngr([20, 70, 30, 80, 60]))

    def test_four(self):
        self.assertEqual([-1, -1, 3, -1], ngl([1, 3, 2, 4]))

    def test_five(self):
        self.assertEqual([-1, 5, 4, 3, 2], ngl([5, 4, 3, 2, 1]))

    def test_six(self):
        self.assertEqual([-1, -1, 70, -1, 80], ngl([20, 70, 30, 80, 60]))

    def test_seven(self):
        self.assertEqual([-1, 1, 1, 2], nsl([1, 3, 2, 4]))

    def test_eight(self):
        self.assertEqual([-1, -1, -1, -1, -1], nsl([5, 4, 3, 2, 1]))

    def test_nine(self):
        self.assertEqual([-1, 20, 20, 30, 30], nsl([20, 70, 30, 80, 60]))

    def test_ten(self):
        self.assertEqual([-1, 2, -1, -1], nsr([1, 3, 2, 4]))

    def test_eleven(self):
        self.assertEqual([4, 3, 2, 1, -1], nsr([5, 4, 3, 2, 1]))

    def test_twelve(self):
        self.assertEqual([-1, 30, -1, 60, -1], nsr([20, 70, 30, 80, 60]))

    def test_thirteen(self):
        self.assertEqual([3, 4, 4, -1], ngr_mono([1, 3, 2, 4]))

    def test_fourteen(self):
        self.assertEqual([-1, -1, -1, -1, -1], ngr_mono([5, 4, 3, 2, 1]))

    def test_fifteen(self):
        self.assertEqual([70, 80, 80, -1, -1], ngr_mono([20, 70, 30, 80, 60]))


if __name__ == "__main__":
    unittest.main()
