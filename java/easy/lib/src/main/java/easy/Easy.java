package easy;

import java.util.*;
import java.util.function.Function;
import java.util.function.Supplier;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public final class Easy {

    private Easy() {}
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

    public static int lengthOfLastWord(final String s) {
       return  Arrays.asList(s.split(" ")).getLast().length();
    }

    public static final class LC136SingleNumber{
        public static int singleNumber(int[] nums) {
          return Arrays.stream(nums).reduce(0, (x, y) -> x ^ y);
        }
    }

    public static void main(String[] args) {
isArraySpecial(new int[]{4,3,1,6}, new int[][]{{0,2},{2,3}});    }

    public static List<List<Integer>> generate(final int numRows) {
        final List<List<Integer>> res = new ArrayList<>();
        res.add(List.of(1));

        for(int i = 1; i <= numRows; i++) {
            final List<Integer> temp = new ArrayList<>();
            temp.add(0);
            temp.addAll(res.getLast());
            temp.add(0);
            final List<Integer> row = new ArrayList<>();
            for(int j = 0; j <= res.getLast().size(); j++)
                row.add(temp.get(j) + temp.get(j + 1));
            res.add(row);
        }return res;
    }

    public static List<Integer> getRow(final int rowIndex) {
        int[] res = new int[]{1};
        for(int i = 0; i < rowIndex; i++){
            final int[] nextRow = new int[res.length + 1];
            for(int j = 0; j < res.length; j++){
                nextRow[j] += res[j];
                nextRow[j + 1] += res[j];
            }res = nextRow;
        }return Arrays.stream(res).boxed().collect(Collectors.toList());
    }

      public class ListNode {
          int val;
          ListNode next;
          ListNode() {}
          ListNode(int val) { this.val = val; }
          ListNode(int val, ListNode next) { this.val = val; this.next = next; }
      }

    final class Solution {
        public ListNode reverseList(final ListNode head) {
            return rec(head, null);
        }

        private static ListNode rec (ListNode head, final ListNode acc){
            return switch(head) {
                case null -> acc;
                default -> {
                    final ListNode next = head.next;
                    head.next = acc;
                    yield rec(next, head);
                }
            };
        }
        public static ListNode mergeTwoLists(final ListNode l1,  final ListNode l2) {
            if (l1 == null && l2 == null) return null;
            else if (l2 == null) return l1;
            else if (l1 == null) return l2;
            else {
                if (l1.val < l2.val){
                    l1.next = mergeTwoLists(l1.next, l2);
                    return l1;
                }else {
                    l2.next = mergeTwoLists(l1, l2.next);
                    return l2;
                }
            }
        }

    }

    private static boolean inner(final Integer n, final Integer r){
        if (r != 0) return false;
        else return switch (n.compareTo(1)){
                case -1 -> false;
                case 0 -> true;
                default -> inner(n / 3, n % 3);
            };
    }

    private final List<List<Integer>> res = new ArrayList<>();
    private final List<Integer> sub = new ArrayList<>();

    public List<List<Integer>> subsets(int[] nums) {
        subsetsGetter(nums, 0);
        return res;
    }

    void subsetsGetter(int[] nums, int i){
        if (i >= nums.length) {
            res.add(new ArrayList<>(sub));
            return;
        }

        sub.add(nums[i]);
        subsetsGetter(nums , i + 1);

        sub.removeLast();
        subsetsGetter(nums , i + 1);

    }

    public static int[] maxSubsequence(final int[] nums, final int k) {
        final PriorityQueue<int[]> heap = new PriorityQueue<>(Comparator.comparingInt(a -> a[0]));
        final List<int[]> res = new ArrayList<>(k);

        for (int i = 0; i < nums.length; i++){
            heap.add(new int[]{nums[i], i});
            if (heap.size() >= k)
                heap.poll();
        };
        while(!heap.isEmpty()) res.add(heap.poll());
        res.sort(Comparator.comparingInt(a -> a[1]));
        return res.stream().mapToInt(a -> a[0]).toArray();
    }

    public double champagneTower(final int poured, final int query_row, final int query_glass) {
        List<Double> res = new ArrayList<>(List.of(0.0));
        for(int i = 1; i <= query_row; i++){
            final Double[] curr_row = new Double[res.size() + 1];
            for(int j = 0; j < res.size(); j++){
                final double extra = res.get(i);
                if (extra > 0) {
                    curr_row[i] += 0.5 * extra;
                    curr_row[i + 1] += 0.5 * extra;
                }
            }
            res = Arrays.asList(curr_row);
        }
        return Math.min(1, res.get(query_glass));
    }

    public int numberOfSubarrays(final int[] nums, final int k) {
        final List<Integer> window = Stream.generate(() -> 0).limit(k).collect(Collectors.toList());
        int starting_points = 1, res = 0;
        for(final int n: nums){
            if ((n & 1) == 1){
                window.removeFirst();
                window.add(starting_points);
                starting_points = 1;
            }else starting_points += 1;
            res += window.getFirst();
        }return res;
    }

    public static int numSubarraysWithSum(final int[] nums, final int goal) {
        final Function<Integer, Integer> lessThanEqual = gl -> {
            if (gl < 0) return 0;
            int sum = 0, res = 0, l = 0;
            for(int r = 0; r < nums.length; r++) {
                sum += nums[r];
                while (sum > gl && l <=r){
                    sum -= nums[l];
                    l += 1;
                }res = r - l + 1;
            }return res;
        };
        return lessThanEqual.apply(goal) - lessThanEqual.apply(goal -1);
    }

    public boolean threeConsecutiveOdds(final int[] arr) {
        int count = 0;
        for (final int num : arr)
            if (count == 3) return true;
            else if ((num & 1) == 1) count++;
            else count = 0;
        return count == 3;
    }

    public int getCommon(final int[] nums1, final int[] nums2) {
        int l = 0, r = 0;
        while(l < nums1.length && r < nums2.length){
            switch (Integer.compare(nums1[l], nums2[r])){
                case -1 -> l += 1;
                case 1  -> r += 1;
                default -> {return nums1[l];}

            }
        }return -1;
    }

    public static boolean[] isArraySpecial(final int[] nums, final int[][] queries) {
        if (nums.length == 1) return new boolean[queries.length];

        var prefixSum = new int[nums.length];
        var res = new boolean[queries.length];

        for(int i = 1; i < nums.length; i++){
            prefixSum[i] = prefixSum[i - 1];
            if( (nums[i] & 1) == (nums[i - 1] & 1) ) prefixSum[i] += 1;
        }

        for(int i = 0; i < queries.length; i++)
            if ( prefixSum[queries[i][1]] - prefixSum[queries[i][0]] == 0 ) res[i] = true;
        System.out.println(Arrays.toString(prefixSum));
        return res;
    }

    public static boolean isHappy(final int n) {
        if (n == 1) return true;
        final Function<Integer, Integer> sos  = a -> {
            int sum = 0;
            while (a != 0){
                sum += n % 10 * n % 10;
                a /= 10;
            }return sum;
        };
        int slow = n, fast = sos.apply(n);
        while (fast != 1 && fast != slow){
            slow = sos.apply(slow);
            fast = sos.andThen(sos).apply(slow);
        }
        return fast == 1;
    }

    public int smallestValue(int n) {
        final Function<Integer, boolean[]> getPrime = (a) -> {
            final var ip = new boolean[a+1];
            Arrays.fill(ip, true);
            for(int p = 2; p * p <= a; p++)
                if(ip[p])
                    for(int i = p * p; i <= a; i += p)
                        ip[i] = false;
            return ip;
        };
        final var isPrime = getPrime.apply(n);

        final Function<Integer, Integer> sop = a -> {
            int sum = 0;
            for(int i = 2; i <= a; i++)
                if (isPrime[i] && a%i == 0)
                    sum += i;
            return sum;
        };

        var prev = n;
        while (!isPrime[n]){
            n = sop.apply(n);
            if (prev == n) break;
            prev = n;
        }return n;
    }

    
}
