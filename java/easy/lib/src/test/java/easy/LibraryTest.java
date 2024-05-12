package easy;

import org.junit.jupiter.api.Test;

import static easy.Easy.bestHand;
import static org.junit.jupiter.api.Assertions.*;

public final class LibraryTest {

    @Test void testLC2347One() {
        assertEquals(bestHand(new int[]{13,2,3,1,9}, new char[] {'a', 'a', 'a', 'a', 'a'}), "Flush");
    }

    @Test void testLC2347Two() {
        assertEquals(bestHand(new int[]{4,4,2,4,4}, new char[] {'d', 'a', 'a', 'b', 'c'}), "Three of a Kind");
    }

    @Test void testLC2347Three() {
        assertEquals(bestHand(new int[]{10,10,2,12,9}, new char[] {'a', 'b', 'c', 'a', 'd'}), "Pair");
    }
}
