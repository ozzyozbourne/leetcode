package org.example;

import java.util.*;

public final class MinimumStringLengthAfterRemovingSubstrings {

    public static int minLength(final String s) {
        final Deque<Character> stack = new ArrayDeque<>(s.length());
        final Set<String> set = Set.of("AB", "CD");
        for(int i = 0; i < s.length(); i++) {
            final char c = s.charAt(i);
            if (!stack.isEmpty() && set.contains(String.valueOf(stack.peek()) + c)) stack.pop();
            else stack.push(c);
        }
        return stack.size();
    }
}
