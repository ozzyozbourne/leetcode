import heapq


def secondGreaterElement(arr):
    n = len(arr)
    middle, stack = [[] for _ in range(n)], []

    for i in range(n):
        while stack and arr[stack[-1]] < arr[i]:
            middle[i].append(stack.pop())
        stack.append(i)

    ans = [-1] * n
    h = []

    for i in range(n):
        while h and arr[h[0][1]] < arr[i]:
            ans[heapq.heappop(h)[1]] = arr[i]
        for j in middle[i]:
            heapq.heappush(h, [arr[j], j])
    return ans
