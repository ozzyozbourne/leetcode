package org.example;

import java.util.Deque;
import java.util.ArrayDeque;
import java.util.Arrays;

public final class NextGreaterII {

    public static int[] nextGreaterElements(final int[] nums) {
        final int[] res = new int[nums.length];
        Arrays.fill(res, -1);
        final Deque<Integer> stack = new ArrayDeque<>(nums.length);
        for (int i = 0; i < nums.length * 2; i++) {
            while (!stack.isEmpty() && nums[stack.peek()] < nums[i % nums.length])
                res[stack.pop()] = nums[i % nums.length];
            if (i < nums.length)
                stack.push(i);
        }
        return res;
    }
}
