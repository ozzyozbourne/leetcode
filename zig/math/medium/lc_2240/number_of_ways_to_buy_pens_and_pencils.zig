const std = @import("std");

pub fn ways_to_buy_pens_pencils(t: i64, c1: i64, c2: i64) i64 {
    var res: i64, var i: i64 = .{0, 0};
    while(true){
        const v = i * c1;
        if (t < v) { break; }
        const l = t - v;
        res += @divTrunc(l, c2) + 1;
        i += 1;
    }
    return res;
}

test "lc_2240_test" {
    const TestCase = struct {
        t: i64,
        c1: i64, 
        c2:i64,
        expected: i64
    };

    const testcases = [_]TestCase{
        .{
            .t = 20, 
            .c1 = 10,
            .c2 = 5,
            .expected = 9
        },
        .{
            .t = 5, 
            .c1 = 10,
            .c2 = 10,
            .expected = 1
        }
    };

    for (testcases) |t| {
        try std.testing.expectEqual(t.expected, ways_to_buy_pens_pencils(t.t, t.c1, t.c2));
    }
}
