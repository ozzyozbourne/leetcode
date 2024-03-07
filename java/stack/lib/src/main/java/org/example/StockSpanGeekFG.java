package org.example;

import java.util.Deque;
import java.util.ArrayDeque;

public final class StockSpanGeekFG {

    public static int[] calculateSpan(final int arr[], final int n) {
        final Deque<int[]> stack = new ArrayDeque<>(n);
        final int[] res = new int[n];

        for (int i = 0; i < n; i++) {
            if (stack.isEmpty())
                res[i] = i + 1;
            else if (stack.peek()[0] > arr[i])
                res[i] = i - stack.peek()[1];
            else {
                while (!stack.isEmpty() && stack.peek()[0] <= arr[i])
                    stack.pop();
                if (stack.isEmpty())
                    res[i] = i + 1;
                else
                    res[i] = i - stack.peek()[1];
            }
            stack.push(new int[] { arr[i], i });
        }

        return res;
    }
}
