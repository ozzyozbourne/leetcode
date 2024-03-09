package org.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import static org.example.NextGreaterII.nextGreaterElements;

public final class NextGreaterIITests {

    @Test
    public void One() {
        assertArrayEquals(new int[] { 2, -1, 2 },
                nextGreaterElements(new int[] { 1, 2, 1 }));
    }

    @Test
    public void Two() {
        assertArrayEquals(new int[] { 2, 3, 4, -1, 4 },
                nextGreaterElements(new int[] { 1, 2, 3, 4, 3 }));
    }

}
