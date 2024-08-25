package easy;

public class Cut {

    public static void main(String[] args) {
        cut(0, 25, 0);
    }

    private static void cut(final int l, final int r, final int k) {
        if (l == r) {
            System.out.printf("K = %d   L = %d R = %d%n", k, l, r);
        }else {
            cut(l + 1, r,       k + 1);
            cut(l      , r + 1, k + 1);
        }
    }
}
