package org.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import static org.example.NearestGreaterOrSmaller.*;

public final class NearestGreaterOrSmallerTests {

    @Test
    public void One() {
        assertArrayEquals(new int[] { -1, -1, -1, -1, -1 }, nearestSmallerToLeft(new int[] { 5, 4, 3, 2, 1 }));
    }

    @Test
    public void Two() {
        assertArrayEquals(new int[] { -1, 20, 20, 30, 30 }, nearestSmallerToLeft(new int[] { 20, 70, 30, 80, 60 }));
    }

    @Test
    public void Three() {
        assertArrayEquals(new int[] { -1, 1, 1, 2 }, nearestSmallerToLeft(new int[] { 1, 3, 2, 4 }));
    }

    @Test
    public void Four() {
        assertArrayEquals(new int[] { -1, 2, -1, -1 }, nearestSmallerToRight(new int[] { 1, 3, 2, 4 }));
    }

    @Test
    public void Five() {
        assertArrayEquals(new int[] { 4, 3, 2, 1, -1 }, nearestSmallerToRight(new int[] { 5, 4, 3, 2, 1 }));
    }

    @Test
    public void Six() {
        assertArrayEquals(new int[] { -1, 30, -1, 60, -1 }, nearestSmallerToRight(new int[] { 20, 70, 30, 80, 60 }));
    }

    @Test
    public void Seven() {
        assertArrayEquals(new int[] { -1, -1, 3, -1 }, nearestGreaterToLeft(new int[] { 1, 3, 2, 4 }));
    }

    @Test
    public void Eight() {
        assertArrayEquals(new int[] { -1, 5, 4, 3, 2 }, nearestGreaterToLeft(new int[] { 5, 4, 3, 2, 1 }));
    }

    @Test
    public void Nine() {
        assertArrayEquals(new int[] { -1, -1, 70, -1, 80 }, nearestGreaterToLeft(new int[] { 20, 70, 30, 80, 60 }));
    }

    @Test
    public void Ten() {
        assertArrayEquals(new int[] { 3, 4, 4, -1 }, nearestGreaterToRight(new int[] { 1, 3, 2, 4 }));
    }

    @Test
    public void Eleven() {
        assertArrayEquals(new int[] { -1, -1, -1, -1, -1 }, nearestGreaterToRight(new int[] { 5, 4, 3, 2, 1 }));
    }

    @Test
    public void Twelve() {
        assertArrayEquals(new int[] { 70, 80, 80, -1, -1 }, nearestGreaterToRight(new int[] { 20, 70, 30, 80, 60 }));
    }

}
