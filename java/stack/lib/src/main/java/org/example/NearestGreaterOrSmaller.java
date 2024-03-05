package org.example;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;
import java.util.List;

public final class NearestGreaterOrSmaller {

    public static int[] nearestGreaterToRight(int arr[]) {

        final Deque<Integer> stack = new ArrayDeque<>(arr.length);
        final List<Integer> res = new ArrayList<>(arr.length);

        for (int i = arr.length - 1; i >= 0; i--) {
            if (stack.isEmpty())
                res.add(-1);
            else if (stack.peek() > arr[i])
                res.add(stack.peek());
            else {
                while (!stack.isEmpty() && stack.peek() <= arr[i])
                    stack.pop();
                if (stack.isEmpty())
                    res.add(-1);
                else
                    res.add(stack.peek());
            }
            stack.push(arr[i]);
        }

        for (int i = 0; i < arr.length; i++)
            arr[i] = res.get(arr.length - 1 - i);
        return arr;
    };

    public static int[] nearestGreaterToLeft(int arr[]) {

        final Deque<Integer> stack = new ArrayDeque<>(arr.length);
        final List<Integer> res = new ArrayList<>(arr.length);

        for (int i = 0; i < arr.length; i++) {
            if (stack.isEmpty())
                res.add(-1);
            else if (stack.peek() > arr[i])
                res.add(stack.peek());
            else {
                while (!stack.isEmpty() && stack.peek() <= arr[i])
                    stack.pop();
                if (stack.isEmpty())
                    res.add(-1);
                else
                    res.add(stack.peek());
            }
            stack.push(arr[i]);
        }
        for (int i = 0; i < arr.length; i++)
            arr[i] = res.get(i);
        return arr;
    };

    public static int[] nearestSmallerToRight(int arr[]) {

        final Deque<Integer> stack = new ArrayDeque<>(arr.length);
        final List<Integer> res = new ArrayList<>(arr.length);

        for (int i = arr.length - 1; i >= 0; i--) {
            if (stack.isEmpty())
                res.add(-1);
            else if (stack.peek() < arr[i])
                res.add(stack.peek());
            else {
                while (!stack.isEmpty() && stack.peek() >= arr[i])
                    stack.pop();
                if (stack.isEmpty())
                    res.add(-1);
                else
                    res.add(stack.peek());
            }
            stack.push(arr[i]);
        }

        for (int i = 0; i < arr.length; i++)
            arr[i] = res.get(arr.length - 1 - i);
        return arr;

    };

    public static int[] nearestSmallerToLeft(int arr[]) {

        final Deque<Integer> stack = new ArrayDeque<>(arr.length);
        final List<Integer> res = new ArrayList<>(arr.length);

        for (int i = 0; i < arr.length; i++) {
            if (stack.isEmpty())
                res.add(-1);
            else if (stack.peek() < arr[i])
                res.add(stack.peek());
            else {
                while (!stack.isEmpty() && stack.peek() >= arr[i])
                    stack.pop();
                if (stack.isEmpty())
                    res.add(-1);
                else
                    res.add(stack.peek());
            }
            stack.push(arr[i]);
        }

        for (int i = 0; i < arr.length; i++)
            arr[i] = res.get(i);
        return arr;

    }

}
