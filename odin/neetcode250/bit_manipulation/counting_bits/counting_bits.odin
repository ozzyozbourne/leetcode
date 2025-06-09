package counting_bits

import "core:fmt"
import "core:log"
import "core:testing"

count_bits :: proc(n: int) -> (res: [dynamic]int) {
	for i in 0 ..= n {
		cur, i := 0, i
		for i > 0 {
			if i & 1 == 0 {
				cur += 1
			}
			i >>= 1
		}
		append(&res, cur)
	}
	return res
}

@(test)
lc_338 :: proc(t: ^testing.T) {
	testcases := []struct {
		n:        int,
		expected: []int,
	}{{2, {0, 1, 1}}, {5, {0, 1, 1, 2, 1, 2}}}

	for testcase in testcases {
		actual := count_bits(testcase.n)
		testing.expect(
			t,
			len(actual) == len(testcase.expected),
			fmt.aprintf(
				"Actual %d and Expected % array len mismatch",
				len(actual),
				len(testcase.expected),
			),
		)

		for i in 0 ..< len(actual) {
			act, exp := actual[i], testcase.expected[i]
			log.infof("\nActual Value -> %d Expected Value -> %d\n", act, exp)
			testing.expect(t, act == exp)
		}
	}
}
