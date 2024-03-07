package org.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.util.ArrayList;
import java.util.List;

public final class OnlineStockPlanLT901Tests {

    @Test
    public void One() {
        final OnlineStockPlanLT901 oLt901 = new OnlineStockPlanLT901();
        final List<Integer> res = new ArrayList<>();
        res.add(oLt901.next(100));
        res.add(oLt901.next(80));
        res.add(oLt901.next(60));
        res.add(oLt901.next(70));
        res.add(oLt901.next(60));
        res.add(oLt901.next(75));
        res.add(oLt901.next(85));
        assertEquals(List.of(1, 1, 1, 2, 1, 4, 6), res);
    }

    @Test
    public void Two() {
        final OnlineStockPlanLT901 oLt901 = new OnlineStockPlanLT901();
        final List<Integer> res = new ArrayList<>();
        res.add(oLt901.next(31));
        res.add(oLt901.next(41));
        res.add(oLt901.next(48));
        res.add(oLt901.next(59));
        res.add(oLt901.next(79));
        assertEquals(List.of(1, 2, 3, 4, 5), res);
    }

}
