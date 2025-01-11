package lc_1400

import (
	"math/bits"
)

func canConstruct(s string, k int) bool {
	switch {
	case len(s) < k:
		return false
	case len(s) == k:
		return true
	default:
		oddCount := 0
		for _, c := range s {
			oddCount ^= 1 << (c - 'a')
		}
		return bits.OnesCount(uint(oddCount)) <= k
	}
}
