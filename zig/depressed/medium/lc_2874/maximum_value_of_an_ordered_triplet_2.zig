const std = @import("std");

pub fn maximum_triplet_value(nums: []const i32) i32 {
    var imax: i32, var dmax: i32, var res: i32 = .{0, 0, 0};
    for (nums) |n| {
        res = @max(res, dmax * n);
        dmax = @max(dmax, imax - n);
        imax = @max(imax, n);
    }
    return res;
}


test "lc_2873_tests" {
    const testcases = [_]struct{
        nums: []const i32, 
        expected: i32,
    }{
        .{
            .nums = &.{12, 6, 1, 2, 7},
            .expected = 77
        },
        .{
            .nums = &.{1, 10, 3, 4, 19},
            .expected = 133
        },
        .{
            .nums = &.{1, 2, 3},
            .expected = 0
        }
    };

    for (testcases) |tc| {
        try std.testing.expectEqual(tc.expected, maximum_triplet_value(tc.nums));
    }
}
