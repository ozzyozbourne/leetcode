package single_number

import "core:log"
import "core:testing"

single_number :: proc(nums: []int) -> (res: int) {
	for num in nums {
		res ~= num
	}
	return res
}

@(test)
lc_136 :: proc(t: ^testing.T) {
	testcases := []struct {
		nums:     []int,
		expected: int,
	}{{{2, 2, 1}, 1}, {{4, 1, 2, 1, 2}, 4}, {{1}, 1}}

	for testcase in testcases {
		actual := single_number(testcase.nums)
		log.infof("\nActual Value -> %d Expected Value -> %d\n", actual, testcase.expected)
		testing.expect(t, actual == testcase.expected)
	}
}
