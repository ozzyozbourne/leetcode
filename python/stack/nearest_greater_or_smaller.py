def nearest_greater_to_the_right(arr):
    stack, res = [], []
    for i in range(len(arr)-1, -1, -1):
        if not stack:
            res.append(-1)
        elif stack[-1] > arr[i]:
            res.append(stack[-1])
        else:
            while stack and stack[-1] <= arr[i]:
                stack.pop()
            if not stack:
                res.append(-1)
            else:
                res.append(stack[-1])
        stack.append(arr[i])
    res.reverse()
    return res


def nearest_greater_to_the_left(arr):
    stack, res = [], []
    for i in range(len(arr)):
        if not stack:
            res.append(-1)
        elif stack[-1] > arr[i]:
            res.append(stack[-1])
        else:
            while stack and stack[-1] <= arr[i]:
                stack.pop()
            if not stack:
                res.append(-1)
            else:
                res.append(stack[-1])
        stack.append(arr[i])
    return res


def nearest_smaller_to_the_right(arr):
    stack, res = [], []
    for i in range(len(arr)-1, -1, -1):
        if not stack:
            res.append(-1)
        elif stack[-1] < arr[i]:
            res.append(stack[-1])
        else:
            while stack and stack[-1] >= arr[i]:
                stack.pop()
            if not stack:
                res.append(-1)
            else:
                res.append(stack[-1])
        stack.append(arr[i])
    res.reverse()
    return res


def nearest_smaller_to_the_left(arr):
    stack, res = [], []
    for i in range(len(arr)):
        if not stack:
            res.append(-1)
        elif stack[-1] < arr[i]:
            res.append(stack[-1])
        else:
            while stack and stack[-1] >= arr[i]:
                stack.pop()
            if not stack:
                res.append(-1)
            else:
                res.append(stack[-1])
        stack.append(arr[i])
    return res


def ngr_mono(arr):
    st, res = [], [-1]*len(arr)
    for i in range(len(arr)):
        while st and arr[st[-1]] < arr[i]:
            res[st.pop()] = arr[i]
        st.append(i)
    return res
