package org.example;

public final class TailRecursiveFactorial {

    public static int tailRecursiveFactorial(int number, int result) {
        if (number <= 1)
            return result;
        return tailRecursiveFactorial(number - 1, result * number);
    }
}
