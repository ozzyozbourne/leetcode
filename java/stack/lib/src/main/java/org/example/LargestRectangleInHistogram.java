package org.example;

import java.util.ArrayDeque;
import java.util.Deque;

public final class LargestRectangleInHistogram {


    public static int largestRectangleArea(final int[] heights) {
      final Deque<int[]> stack = new ArrayDeque<>(heights.length);
      int maxArea = 0;
      for (int i = 0, start = i; i < heights.length; i++) {
          while (!stack.isEmpty() && stack.peek()[1] > heights[i]) {
              final int[] pop = stack.pop();
              maxArea = Math.max(maxArea, pop[1] * (i - pop[0]));
              start = pop[0];
          }
          stack.push(new int[]{start, heights[i]});
      }
      while (!stack.isEmpty()) {
          final int[] pop = stack.pop();
          maxArea = Math.max(maxArea, pop[1] * (heights.length - pop[0]));
      }
      return maxArea;
    }


}
