package lc84

func largestRectangleArea(heights []int) int {
	stack, maxArea := [][2]int{}, 0
	for i, v := range heights {
		start := i
		for len(stack) != 0 && stack[len(stack)-1][1] > v {
			pop := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			popArea := pop[1] * (i - pop[0])
			if popArea > maxArea {
				maxArea = popArea
			}
			start = pop[0]
		}
		stack = append(stack, [2]int{start, v})
	}

	for _, pop := range stack {
		popArea := (len(heights) - pop[0]) * pop[1]
		if popArea > maxArea {
			maxArea = popArea
		}
	}
	return maxArea
}
