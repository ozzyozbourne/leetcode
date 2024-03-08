import unittest
from DailyTemperatures import dailyTemperatures

class TestFact(unittest.TestCase):
    def test_one(self):
        self.assertEqual(
            [1,1,1,0], dailyTemperatures([30,40,50,60])
        )

    def test_two(self):
        self.assertEqual(
            [1,1,4,2,1,1,0,0], dailyTemperatures([73,74,75,71,69,72,76,73])
        )
    
    def test_three(self):
        self.assertEqual(
            [1,1,0], dailyTemperatures([30,60,90])
        )



if __name__ == "__main__":
    unittest.main()
