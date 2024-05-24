package org.example;

public final class CrawlerLogFolder {

    public static int minOperations(final String[] logs) {
        int step = 0;
        for (final String log : logs) {
            switch (log) {
                case "../":
                    step -= (step > 0) ? 1 : 0;
                    break;
                case "./": break;
                default: step++;
            }
        }
        return step;
    }
}
