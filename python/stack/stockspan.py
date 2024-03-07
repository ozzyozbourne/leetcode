def calculateSpan(a, n):
    stack = []
    res = []
    for i in range(n):
        if not stack:
            res.append(i + 1)
        elif stack[-1][0] > a[i]:
            res.append(i - stack[-1][1])
        else:
            while stack and stack[-1][0] <= a[i]:
                stack.pop()
            if not stack:
                res.append(i + 1)
            else:
                res.append(i - stack[-1][1])
        stack.append([a[i], i])
    return res
