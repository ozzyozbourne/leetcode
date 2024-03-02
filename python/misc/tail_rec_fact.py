def fact(n, r):
    if n <= 1:
        return r
    return fact(n - 1, n * r)
