package org.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import static org.example.NearestGreaterOrSmaller.*;

public final class NearestGreaterOrSmallerTests {

    @Test
    public void One() {
        assertEquals(new int[] { -1, -1, -1, -1, -1 }, nearestSmallerToLeft(new int[] { 5, 4, 3, 2, 1 }));
    }

    @Test
    public void Two() {
        assertEquals(new int[] { -1, 20, 20, 30, 30 }, nearestSmallerToLeft(new int[] { 20, 70, 30, 80, 60 }));
    }

    @Test
    public void Three() {
        assertEquals(new int[] { -1, 1, 1, 2 }, nearestSmallerToLeft(new int[] { 1, 3, 2, 4 }));
    }

    @Test
    public void Four() {
        assertEquals(new int[] { -1, 2, -1, -1 }, nearestSmallerToRight(new int[] { 1, 3, 2, 4 }));
    }

    @Test
    public void Five() {
        assertEquals(new int[] { 4, 3, 2, 1, -1 }, nearestSmallerToRight(new int[] { 5, 4, 3, 2, 1 }));
    }

    @Test
    public void Six() {
        assertEquals(new int[] { -1, 30, -1, 60, -1 }, nearestSmallerToRight(new int[] { 20, 70, 30, 80, 60 }));
    }

    @Test
    public void Seven() {
        assertEquals(new int[] { -1, -1, 3, -1 }, nearestGreaterToLeft(new int[] { 1, 3, 2, 4 }));
    }

    @Test
    public void Eight() {
        assertEquals(new int[] { -1, 5, 4, 3, 2 }, nearestGreaterToLeft(new int[] { 5, 4, 3, 2, 1 }));
    }

    @Test
    public void Nine() {
        assertEquals(new int[] { -1, -1, 70, -1, 80 }, nearestGreaterToLeft(new int[] { 20, 70, 30, 80, 60 }));
    }

    @Test
    public void Ten() {
        assertEquals(new int[] { 3, 4, 4, -1 }, nearestGreaterToRight(new int[] { 1, 3, 2, 4 }));
    }

    @Test
    public void Eleven() {
        assertEquals(new int[] { -1, -1, -1, -1, -1 }, nearestGreaterToRight(new int[] { 5, 4, 3, 2, 1 }));
    }

    @Test
    public void Twelve() {
        assertEquals(new int[] { 70, 80, 80, -1, -1 }, nearestGreaterToRight(new int[] { 20, 70, 30, 80, 60 }));
    }

}
