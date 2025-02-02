const std = @import("std");

pub fn rob(nums: []const i32) i32 {
    var rob1:i32, var rob2:i32 = .{0, 0};
    for (nums) |n| {
        const temp = @max(n + rob1, rob2);
        rob1 = rob2;
        rob2 = temp;
    }
    return rob2;
}

test "lc_198_test" {
    const TestCase = struct {
        nums: []const i32,
        expected: i32
    };
    const testcases = [_]TestCase{
        .{
            .nums = &[_]i32{1,2,3,1},
            .expected = 4 
        },
        .{
            .nums = &[_]i32{2,7,9,3,1},
            .expected = 12
        }
    };
    for (testcases) |t| {
        try std.testing.expectEqual(t.expected, rob(t.nums));
    }
}
