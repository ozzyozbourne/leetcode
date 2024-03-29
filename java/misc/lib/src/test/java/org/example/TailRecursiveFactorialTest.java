package org.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

public final class TailRecursiveFactorialTest {
    @Test
    void someLibraryMethodReturnsTrue() {
        assertEquals(TailRecursiveFactorial.tailRecursiveFactorial(5, 1), 120);
        assertEquals(TailRecursiveFactorial.tailRecursiveFactorial(4, 1), 24);
        assertEquals(TailRecursiveFactorial.tailRecursiveFactorial(3, 1), 6);
    }
}
