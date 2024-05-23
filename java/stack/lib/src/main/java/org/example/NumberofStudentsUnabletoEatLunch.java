package org.example;

public final class NumberofStudentsUnabletoEatLunch {

    public static int countStudents(final int[] students, final int[] sandwiches) {
        int zeroCount = 0, oneCount =  0;
        for(final int s: students){
            if (s == 0) zeroCount += 1;
            else oneCount += 1;
        }
        for(final int s: sandwiches){
            if(s == 0 && zeroCount == 0) return oneCount;
            if(s == 1 && oneCount  == 0) return zeroCount;
            if(s == 0)zeroCount -= 1;
            else oneCount -= 1;
        }
        return 0;
    }
}
