package lc1475

import (
	"slices"
	"testing"
)

func TestFinalPrices(t *testing.T) {
	testValues := []struct {
		input    []int
		expected []int
	}{
		{[]int{8, 4, 6, 2, 3}, []int{4, 2, 4, 2, 3}},
		{[]int{1, 2, 3, 4, 5}, []int{1, 2, 3, 4, 5}},
		{[]int{10, 1, 1, 6}, []int{9, 0, 1, 6}},
	}
	for _, v := range testValues {
		res := finalPrices(v.input)
		if slices.Compare(res, v.expected) != 0 {
			t.Errorf("Actual -> %d\tExpected -> %d\n", res, v.expected)
		}
	}
}
