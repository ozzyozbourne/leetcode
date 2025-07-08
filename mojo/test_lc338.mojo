from testing import assert_equal

def countBits(n: Int) -> List[Int]:
    res = [0] * (n + 1)
    for i in range(n + 1): res[i] = res[i >> 1] + (i & 1)
    return res

def test_lc338():
    for inp, exp in [(2, List[Int](0,1,1)), (5, List[Int](0,1,1,2,1,2))]: assert_equal(countBits(inp), exp)
