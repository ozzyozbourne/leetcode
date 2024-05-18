package org.example;

import org.junit.jupiter.api.Test;

import static org.example.LongestValidParenthesis.longestValidParentheses;

import static org.junit.jupiter.api.Assertions.assertEquals;

public final class LongestValidParenthesisTest {

    @Test
    public void One() {
        assertEquals(2, longestValidParentheses("(()"));
    }

    @Test
    public void Two() {
        assertEquals(4, longestValidParentheses(")()())"));
    }

    @Test
    public void Three() {
        assertEquals(0, longestValidParentheses(""));
    }
}
