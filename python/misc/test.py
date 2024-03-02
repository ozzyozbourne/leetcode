import unittest
from tail_rec_fact import fact

class TestFact(unittest.TestCase):

    def test_one(self):
        self.assertEqual(120, fact(5, 1))

    def test_two(self):
        self.assertEqual(24,  fact(4, 1))

    def test_three(self):
        self.assertEqual(6,   fact(3, 1))    


if __name__ == "__main__":
           unittest.main()        
