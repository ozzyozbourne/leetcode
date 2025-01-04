const std = @import("std");

pub fn min_cost(allocator: std.mem.Allocator, n: i32, cuts: []const i32) !i32 {
    var map = std.AutoHashMap([2]i32, i32).init(allocator);
    defer map.deinit();

    const dfs = struct {
        fn f(l: i32, r: i32, cuts_slice: []const i32, dp: *std.AutoHashMap([2]i32, i32) )!i32 {
            if (r - l == 1) { return 0; }
            else if (dp.get([2]i32{l, r})) |value| { return value; }
            else {
                var res:i32 = std.math.maxInt(i32);

                for (cuts_slice) |cut| {
                    if (l < cut and cut < r){
                        res = @min(res, ( (r - l) + try f(l, cut, cuts_slice, dp) + try f(cut, r, cuts_slice, dp) ));
                    }
                }

                if (res == std.math.maxInt(i32)) {res = 0;}

                try dp.put([2]i32{l, r}, res);
                return res;
            }
        }
    }.f;
    return dfs(0, n, cuts, &map);
}


test "lc_1547_test" {
    const TestCase = struct {
        cuts: []const i32,
        n: i32,
        expected: i32
    };

    const testcases = [_]TestCase{
        .{ 
            .cuts = &[_]i32{6, 4},
            .n = 7,
            .expected = 10
        },
        .{
            .cuts = &[_]i32{5,6,1,4,2},
            .n = 9,
            .expected = 22
        },
    }; 

    for(testcases) |t|{
        try std.testing.expectEqual(t.expected, min_cost(std.testing.allocator, t.n, t.cuts));
    }
}
