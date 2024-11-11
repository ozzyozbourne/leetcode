package org.example;


import java.util.Arrays;
import java.util.Comparator;

public final class RemoveIntervals {

    public static void main(final String[] args) {

    }

    public static int eraseOverlapIntervals(final int[][] intervals) {
        Arrays.sort(intervals, Comparator.comparingInt(a -> a[1]));
        var answer = -1;
        var prev = intervals[0];
        for(final int[] interval: intervals)
            if (interval[0] < prev[1]) answer += 1;
            else prev = interval;
        return answer;
    }

    public static int countWays(final int[][] ranges) {
        Arrays.sort(ranges, Comparator.comparingInt(a -> a[0]));
        final int MOD = 1_000_000_007;
        long res = 1;
        int lastKnownEnd = -1;

        for (final int[] range : ranges)
            if (range[0] > lastKnownEnd) {
                res = (res * 2) % MOD;
                lastKnownEnd = range[1];
            } else lastKnownEnd = Math.max(lastKnownEnd, range[1]);
        return (int) res;
    }
}


