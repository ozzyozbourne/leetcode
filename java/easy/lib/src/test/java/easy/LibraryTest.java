package easy;

import org.junit.jupiter.api.Test;

import static easy.Easy.*;
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

    @Test void testLC2660One() {
        assertEquals(isWinner(new int[]{4,10,7,9}, new int[] {6,5,2,3}), 1);
    }

    @Test void testLC2660Two() {
        assertEquals(isWinner(new int[]{3,5,7,6}, new int[] {8,10,10,2}), 2);
    }

    @Test void testLC2660Three() {
        assertEquals(isWinner(new int[]{2,3}, new int[] {4,1}), 0);
    }

    @Test void testLC58One() {
        assertEquals(lengthOfLastWord("Hello World"), 5);
    }

    @Test void testLC58Two() {
        assertEquals(lengthOfLastWord("   fly me   to   the moon  "), 4);
    }

    @Test void testLC58Three() {
        assertEquals(lengthOfLastWord("luffy is still joyboy"), 6);
    }
}
