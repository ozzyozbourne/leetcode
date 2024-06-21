import heapq
def fact(n, r):
    if n <= 1:
        return r
    return fact(n - 1, n * r)


def maxSubsequence(self, nums, k):
    heapq.heapify([(-num, i) for i, num in enumerate(nums)])
    res = [heapq.heappop(indexed_nums) for _ in range(k)]
    res.sort(key=lambda x: x[1])
    return [-x[0] for x in res]

