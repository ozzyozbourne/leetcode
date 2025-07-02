from testing import assert_equal

def singleNumber(nums: List[Int]) -> Int:
    res = 0
    for n in nums: res ^= n
    return res

def test_lc136():
    for input, expected in [(List[Int](4, 1, 2, 1, 2), 4), (List[Int](2, 2, 1), 1), (List[Int](1), 1)]: 
        assert_equal(singleNumber(input), expected)
