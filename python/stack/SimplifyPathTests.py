import unittest
from SimplifyPath import simplifyPath


class TestFact(unittest.TestCase):
    def test_one(self):
        self.assertEqual("/home", simplifyPath("/home/"))

    def test_two(self):
        self.assertEqual("/", simplifyPath("/../"))

    def test_three(self):
        self.assertEqual("/home/foo", simplifyPath("/home//foo/"))


if __name__ == "__main__":
    unittest.main()
