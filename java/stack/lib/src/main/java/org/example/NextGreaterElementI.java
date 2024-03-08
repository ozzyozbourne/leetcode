package org.example;

import java.util.Deque;
import java.util.ArrayDeque;
import java.util.Map;
import java.util.HashMap;
import java.util.Arrays;

public final class NextGreaterElementI {

    public static int[] nextGreaterElement(int[] arr, final int[] nums2) {
        final Map<Integer, Integer> map = new HashMap<>();
        final Deque<Integer> stack = new ArrayDeque<>(arr.length);
        final int[] res = new int[arr.length];
        Arrays.fill(res, -1);
        for (int i = 0; i < arr.length; i++)
            map.put(arr[i], i);
        for (int i = 0; i < nums2.length; i++) {
            while (!stack.isEmpty() && stack.peek() < nums2[i])
                res[map.get(stack.pop())] = nums2[i];
            if (map.containsKey(nums2[i]))
                stack.push(nums2[i]);
        }
        return res;
    }
}
