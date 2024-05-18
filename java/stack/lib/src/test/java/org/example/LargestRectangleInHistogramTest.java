package org.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

public final class LargestRectangleInHistogramTest {

    @Test
    public void One() {

        assertEquals(10, LargestRectangleInHistogram.largestRectangleArea(new int[] { 2, 1, 5, 6, 2, 3, 7 }));
    }

    @Test
    public void Two() {

        assertEquals(4, LargestRectangleInHistogram.largestRectangleArea(new int[] { 2, 4 }));
    }

}
