def dailyTemperatures(temperatures):
    stack = []
    res = []
    for i in range(len(temperatures) - 1, -1, -1):
        if not stack:
            res.append(0)
        elif stack and stack[-1][0] > temperatures[i]:
            res.append(stack[-1][1] - i)
        else:
            while stack and stack[-1][0] <= temperatures[i]:
                stack.pop()
            if not stack:
                res.append(0)
            else:
                res.append(stack[-1][1] - i)
        stack.append([temperatures[i], i])
    res.reverse()
    return res
