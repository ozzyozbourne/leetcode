package org.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import static org.example.NextGreaterElementI.nextGreaterElement;

public final class NextGreaterElementITests {

    @Test
    public void One() {
        assertArrayEquals(new int[] { -1, 3, -1 },
                nextGreaterElement(new int[] { 4, 1, 2 }, new int[] { 1, 3, 4, 2 }));
    }

    @Test
    public void Two() {
        assertArrayEquals(new int[] { 3, -1 },
                nextGreaterElement(new int[] { 2, 4 }, new int[] { 1, 2, 3, 4 }));
    }

}
