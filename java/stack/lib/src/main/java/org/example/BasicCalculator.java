package org.example;

import java.util.ArrayDeque;
import java.util.Deque;

public final class BasicCalculator {
    public int calculate(final String s) {
        final Deque<Integer> stack = new ArrayDeque<>();
        int curr = 0, output = 0, sign = 1;
        for(final char ch: s.toCharArray())
            switch (ch) {
                case ' ' -> {}
                case '+' -> {
                    output += curr * sign;
                    curr = 0;
                    sign = 1;
                }
                case '-' -> {
                    output += curr * sign;
                    curr = 0;
                    sign = -1;
                }
                case '(' -> {
                    stack.push(output);
                    stack.push(sign);
                    output = 0;
                    sign = 1;
                }
                case ')' -> {
                    output += curr * sign;
                    output *= stack.pop();
                    output += stack.pop();
                    curr = 0;
                }
                default -> curr = curr * 10 + (ch - '0') ;
            }
        return output + curr * sign ;
    }
}
