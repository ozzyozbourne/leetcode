def nextGreaterElement(n):
    s = str(n)
    s_list = [c for c in s]
    l = len(s)
    if l == 1:
        return -1
    i = l - 2
    while i >= 0 and s_list[i] >= s_list[i + 1]:
        i -= 1
    if i < 0:
        return -1
    j = l - 1
    while s_list[j] <= s_list[i]:
        j -= 1
    s_list[i], s_list[j] = s_list[j], s_list[i]
    s_list[i + 1:] = reversed(s_list[i + 1:])

    ans = ''.join(s_list)
    return int(ans) if int(ans) < 2 ** 31 else -1
