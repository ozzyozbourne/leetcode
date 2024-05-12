package easy;

import java.util.Arrays;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public final class Easy {

    public static String bestHand(final int[] ranks, final char[] suits) {
       return switch (IntStream.range(0, suits.length).mapToObj(i -> suits[i]).collect(Collectors.toSet()).size()) {
            case 1 -> "Flush";
            default -> {
                var counter = new int[13];
                Arrays.stream(ranks).forEach(rank -> counter[rank - 1] += 1);
                yield switch (Arrays.stream(counter).max().getAsInt()){
                    case 1 -> "High Card";
                    case 2 -> "Pair";
                    default -> "Three of a Kind";
                };
            }
        };
    }
}
