package easy;

import java.util.Arrays;
import java.util.List;
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

    public static int isWinner(final int[] player1, final int[] player2) {
         return switch (score(Arrays.stream(player1).boxed().toList()).compareTo(score(Arrays.stream(player2).boxed().toList()))){
           case  1 -> 1;
           case -1 -> 2;
           default -> 0;
        };
    }

    private static Integer score(final List<Integer> player){
        return player.stream().reduce(new int[]{0, 0, 0}, Easy::updateAccumulator, (res, x) -> res)[2];
    }
    private static int[] updateAccumulator(final int[] res, final int x) {
        if(res[0] == 10 || res[1] == 10) {
            res[0] = res[1];
            res[1] = x;
            res[2] += 2 * x;
        }else{
            res[1] = x;
            res[2] += x;
        }return res;

    }
}
