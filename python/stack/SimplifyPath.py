def simplifyPath(s):
    stack, cur = [], ""
    for c in s + "/":
        if c == "/":
            if cur == "..":
                if stack: stack.pop()
            elif cur != "" and cur != ".":
                stack.append(cur)
            cur = ""
        else:
            cur += c
    return "/" + "/".join(stack)
