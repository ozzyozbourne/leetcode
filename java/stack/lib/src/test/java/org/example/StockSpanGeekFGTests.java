package org.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import static org.example.StockSpanGeekFG.calculateSpan;

public final class StockSpanGeekFGTests {

    @Test
    public void One() {
        assertArrayEquals(new int[] { 1, 1, 1, 2, 1, 4, 6 },
                calculateSpan(new int[] { 100, 80, 60, 70, 60, 75, 85 }, 7));
    }

}
