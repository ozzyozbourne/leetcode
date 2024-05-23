package org.example;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public final class NumberofStudentsUnabletoEatLunchTest {

    @Test
    public void One() {

        assertEquals(0, NumberofStudentsUnabletoEatLunch
                .countStudents(new int[] { 1,1,0,0 }, new int[] { 0,1,0,1}));
    }

    @Test
    public void Two() {

        assertEquals(3, NumberofStudentsUnabletoEatLunch
                .countStudents(new int[] { 1,1,1,0,0,1 }, new int[] {1,0,0,0,1,1}));
    }
}
