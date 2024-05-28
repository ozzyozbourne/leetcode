package org.example;

public final class MakeTheStringGreat {
     public static String makeGood(final String s) {
        final StringBuilder stack = new StringBuilder();
        for(final char ch: s.toCharArray())
            if (!stack.isEmpty() &&
                    stack.charAt(stack.length() - 1) != ch &&
                    Character.toLowerCase(stack.charAt(stack.length() - 1)) == Character.toLowerCase(ch))
                stack.deleteCharAt(stack.length() - 1);
            else stack.append(ch);
        return stack.toString();
    }
    
}
