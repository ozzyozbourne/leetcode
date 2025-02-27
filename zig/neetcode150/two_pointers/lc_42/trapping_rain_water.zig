const std = @import("std");

pub fn trap(heights: []const i32) i32 {
    var l: usize, var r: usize = .{ 0, heights.len - 1 };
    var l_max: i32, var r_max: i32, var res: i32 = .{ heights[l], heights[r], 0 };
    while (l < r) {
        if (l_max < r_max) {
            l += 1; 
            l_max = @max(l_max, heights[l]);
            res += l_max - heights[l];
        } else {
            r = if (r != 0) r - 1 else 0; 
            r_max = @max(r_max, heights[r]);
            res += r_max - heights[r];
        }
    }
    return res;
}

test "lc_42_test" {
    const TestCase = struct {
        heights: []const i32, 
        expected: i32
    };

    const testcase = [_]TestCase{
        .{
            .heights = &.{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1},
            .expected = 6
        },
        .{
            .heights = &.{4, 2, 0, 3, 2, 5},
            .expected = 9
        }
    };

    for (testcase) |tc| { try std.testing.expectEqual(tc.expected, trap(tc.heights)); }
}
