from testing import assert_equal

def addBinary(a: String, b: String) -> String: 
    (con, carry, i, j, res): (List[Int], Int, Int, Int, String)  = [], 0, len(a) - 1, len(b) - 1, String("")
    while i >= 0 or j >= 0 or carry:
        total = carry
        if i >= 0: 
            total += Int(a[i])
            i -= 1;
        if j >= 0:
            total += Int(b[j])
            j -= 1;
        con.append(total % 2)
        carry = total // 2
    for v in reversed(con): res += String(v)
    return res

def test_lc67():
    testcases = [(String("11"), String("1"), String("100")), (String("1010"), String("1011"), String("10101"))]
    for a, b, exp in testcases: assert_equal(addBinary(a, b), exp)

