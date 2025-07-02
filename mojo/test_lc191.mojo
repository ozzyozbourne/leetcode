from testing import assert_equal

def hammingWeight(owned n: Int) -> Int:
    res = 0
    while n:
        res += 1 if (n & 1) == 1 else 0
        n >>= 1
    return res

def test_lc191():
    for inp, exp in [(11, 3), (128, 1), (2147483645, 30)]: assert_equal(hammingWeight(inp), exp)
