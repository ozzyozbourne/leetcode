package org.example;

import java.util.Deque;
import java.util.ArrayDeque;
import java.util.List;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Arrays;

public final class LargestRectangleInHistogram {
    private record Pair(int l, int r) {
    }

    private final int cLeft = -1;
    private int cRight;

    public int largestRectangleArea(int[] arr) {
        final Deque<Pair> sl = new ArrayDeque<>(arr.length);
        final Deque<Pair> sr = new ArrayDeque<>(arr.length);
        final List<Integer> l = new ArrayList<>(arr.length);
        final List<Integer> r = new ArrayList<>(arr.length);
        cRight = arr.length;

        for (int i = 0; i < arr.length; i++) {
            l.add(nsl(sl, arr, i));
            r.add(nsr(sr, arr, arr.length - 1 - i));
        }
        Collections.reverse(r);
        for (int i = 0; i < arr.length; i++)
            arr[i] = (r.get(i) - l.get(i) - 1) * arr[i];
        return Arrays.stream(arr).max().getAsInt();
    }

    private int nsl(final Deque<Pair> stack, final int[] arr, final int i) {
        int res;
        if (stack.isEmpty())
            res = cLeft;
        else if (stack.peek().l < arr[i])
            res = stack.peek().r;
        else {
            while (!stack.isEmpty() && stack.peek().l >= arr[i])
                stack.pop();
            if (stack.isEmpty())
                res = cLeft;
            else
                res = stack.peek().r;
        }
        stack.push(new Pair(arr[i], i));
        return res;
    }

    private int nsr(final Deque<Pair> stack, final int[] arr, final int i) {
        int res;
        if (stack.isEmpty())
            res = cRight;
        else if (stack.peek().l < arr[i])
            res = stack.peek().r;
        else {
            while (!stack.isEmpty() && stack.peek().l >= arr[i])
                stack.pop();
            if (stack.isEmpty())
                res = cRight;
            else
                res = stack.peek().r;
        }
        stack.push(new Pair(arr[i], i));
        return res;
    }
}
