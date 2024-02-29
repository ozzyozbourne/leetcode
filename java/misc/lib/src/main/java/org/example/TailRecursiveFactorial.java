package org.example;

public final class TailRecursiveFactorial {
    public static int tailRecursiveFactorial(int n, int r) {
        if (n <= 1)
            return r;
        return tailRecursiveFactorial(n - 1, n * r);
    }
}
