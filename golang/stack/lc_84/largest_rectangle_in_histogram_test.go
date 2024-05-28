package lc84

import (
	"testing"
)

func TestLargestRectangleArea(t *testing.T) {
	tests := []struct {
		heights  []int
		expected int
	}{
		{[]int{2, 1, 5, 6, 2, 3}, 10},
		{[]int{2, 4}, 4},
	}

	for _, val := range tests {
		res := largestRectangleArea(val.heights)
		if res != val.expected {
			t.Errorf("Actual -> %d\tExpected -> %d\n", res, val.expected)
		}
	}
}
