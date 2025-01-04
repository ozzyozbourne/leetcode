package lc1547

import (
	"math"
)

func minCost(n int, cuts []int) int {
	// Using a map for memoization
	dp := make(map[[2]int]int)

	var dfs func(l, r int) int
	dfs = func(l, r int) int {
		// Base case: if segment length is 1
		if r-l == 1 {
			return 0
		}

		// Check if already computed
		if val, exists := dp[[2]int{l, r}]; exists {
			return val
		}

		// Initialize result to max possible value
		res := math.MaxInt32
		found := false

		// Try each cut position
		for _, c := range cuts {
			if l < c && c < r {
				found = true
				cost := (r - l) + dfs(l, c) + dfs(c, r)
				res = min(res, cost)
			}
		}

		// If no valid cuts found, cost is 0
		if !found {
			res = 0
		}

		// Memoize and return result
		dp[[2]int{l, r}] = res
		return res
	}

	return dfs(0, n)
}
