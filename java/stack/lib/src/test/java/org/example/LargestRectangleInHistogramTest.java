package org.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

public final class LargestRectangleInHistogramTest {

    @Test
    public void One() {
        final LargestRectangleInHistogram lHistogram = new LargestRectangleInHistogram();
        assertEquals(10, lHistogram.largestRectangleArea(new int[] { 2, 1, 5, 6, 2, 3, 7 }));
    }

}
