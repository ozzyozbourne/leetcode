const std = @import("std");

pub fn max_profit(prices: []const i32) i32 {
     var l: usize, var r: usize, var maxp: i32 = .{ 0, 1, 0 };
     while (r < prices.len) {
         if (prices[l] < prices[r]) { maxp = @max(maxp, prices[r] - prices[l]); } 
         else { l = r; }
         r += 1;
     }
     return maxp;
}

test "lc_121_test" {
    const testcases = [_] struct{
        prices: []const i32,
        expected: i32
    }{
        .{
            .prices = &.{7, 1, 5, 3, 6, 4}, 
            .expected = 5
        },
        .{
            .prices = &.{7, 6, 4, 3, 1}, 
            .expected = 0
        }
    };

    for (testcases) |tc| { try std.testing.expectEqual(tc.expected, max_profit(tc.prices)); }
}
