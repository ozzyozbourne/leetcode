def isValid(s):
    stack, maps = [], {"]": "[", "}": "{", ")": "("}
    for c in s:
        if c in maps:
            if stack and stack[-1] == maps[c]:
                stack.pop()
            else:
                return False
        else:
            stack.append(c)
    return True if not stack else False
