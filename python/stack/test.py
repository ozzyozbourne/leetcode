import unittest
from nearest_greater_or_smaller import nearest_greater_to_the_right

class Test(unittest.TestCase):
    
    def test_one(self):
        self.assertEqual([3, 4, 4, -1], nearest_greater_to_the_right([1, 3, 2,4]))
    
    def test_two(self):
        self.assertEqual([-1, -1, -1, -1, -1], nearest_greater_to_the_right([5, 4, 3, 2, 1]))

    def test_three(self):
        self.assertEqual([70, 80, 80, -1, -1], nearest_greater_to_the_right([20, 70, 30, 80, 60]))

if __name__ == "__main__":
    unittest.main()


