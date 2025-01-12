package lc_816

func wordSubsets(w1 []string, w2 []string) []string {
	count := func(w string) [26]int {
		freq := [26]int{}
		for _, c := range w {
			freq[c-'a'] += 1
		}
		return freq
	}

	all := func(a, b [26]int) bool {
		for i := 0; i < 26; i++ {
			if a[i] < b[i] {
				return false
			}
		}
		return true
	}

	bmax, res := [26]int{}, []string{}
	for _, w := range w2 {
		freq := count(w)
		for i := 0; i < 26; i++ {
			bmax[i] = max(bmax[i], freq[i])
		}
	}

	for _, w := range w1 {
		if all(count(w), bmax) {
			res = append(res, w)
		}
	}
	return res
}
