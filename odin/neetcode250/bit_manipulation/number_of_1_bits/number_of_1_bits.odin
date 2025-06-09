package number_of_1_bits

import "core:log"
import "core:testing"

hamming_weight :: proc(n: int) -> (res: int) {
	n := n
	for n > 0 {
		if n & 1 == 1 {
			res += 1
		}

		n >>= 1
	}
	return res
}


@(test)
lc_191 :: proc(t: ^testing.T) {
	testcases := []struct {
		n:        int,
		expected: int,
	}{{11, 3}, {128, 1}, {2147483645, 30}}

	for testcase in testcases {
		actual := hamming_weight(testcase.n)
		log.infof("\nActual Value -> %d Expected Value -> %d\n", actual, testcase.expected)
		testing.expect(t, actual == testcase.expected)
	}

}
