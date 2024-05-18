package org.example;

import java.util.ArrayDeque;
import java.util.Deque;

public final class LongestValidParenthesis {

    public static int longestValidParentheses(final String s) {
        final Deque<Integer> stack = new ArrayDeque<>(s.length());
        stack.push(-1);
        int res = 0;
        for(int i = 0; i < s.length(); i++)
            if (s.charAt(i) == '(') stack.push(i);
            else{
                stack.pop();
                if(stack.isEmpty()) stack.push(-1);
                else res = Math.max(res, i - stack.peek());
            }
        return res;
    }
}
