package lc85

import (
	"testing"
)

func TestMaximalRectangle(t *testing.T) {
	var tests = []struct {
		matrix   [][]byte
		expected int
	}{
		{
			[][]byte{
				{1, 0, 1, 0, 0},
				{1, 0, 1, 1, 1},
				{1, 1, 1, 1, 1},
				{1, 0, 0, 1, 0},
			},
			6,
		},
		{
			[][]byte{{0}},
			0,
		},
		{
			[][]byte{{1}},
			1,
		},
	}
	for _, v := range tests {
		res := maximalRectangle(v.matrix)
		if res != v.expected {
			t.Errorf("Actual -> %d\tExpected -> %d\n", res, v.expected)

		}
	}
}
