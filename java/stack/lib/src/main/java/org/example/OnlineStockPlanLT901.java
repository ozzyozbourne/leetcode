package org.example;

import java.util.Deque;
import java.util.ArrayDeque;

public final class OnlineStockPlanLT901 {

    private record Pair(int left, int right) {
    }

    private final Deque<Pair> stack = new ArrayDeque<>();
    private int counter = 1;

    public OnlineStockPlanLT901() {
    }

    public int next(int price) {
        final Pair pair = new Pair(price, counter);
        if (stack.isEmpty())
            price = pair.right;
        else if (stack.peek().left > price)
            price = pair.right - stack.peek().right;
        else {
            while (!stack.isEmpty() && stack.peek().left <= pair.left)
                stack.pop();
            if (stack.isEmpty())
                price = pair.right;
            else
                price = pair.right - stack.peek().right;
        }
        stack.push(pair);
        counter++;
        return price;
    }
}
