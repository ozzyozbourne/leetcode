from testing import assert_equal

def possibleStringCount(n: String) -> Int:
    ans = 1
    for i in range(1, len(n)):
        if n[i - 1] == n[i]: ans += 1
    return ans

def test_lc3330():
    testcases: List[(String, Int)] = [(String("abbcccc"), 5), (String("abcd"), 1), (String("aaaa"), 4)]
    for i, exp in testcases: assert_equal(possibleStringCount(i), exp)
