package lc227

import (
	"testing"
)

func TestCalculate(t *testing.T) {
	testValues := []struct {
		input    string
		expected int
	}{
		{"3+2*2", 7},
		{" 3/2 ", 1},
		{" 3+5 / 2 ", 5},
	}
	for _, v := range testValues {
		res := calculate(v.input)
		if res != v.expected {
			t.Errorf("Actual -> %d\tExpected -> %d\n", res, v.expected)
		}
	}
}
