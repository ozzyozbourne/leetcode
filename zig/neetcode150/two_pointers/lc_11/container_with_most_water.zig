const std = @import("std");

pub fn max_area(heights: []const i32) i32 {
    var l: usize, var r: usize, var res: i32 = .{ 0, heights.len - 1, 0 };
    while (l < r) {
        const area = @min(heights[l], heights[r]) * @as(i32, @intCast(r - l)); 
        res = @max(res, area);
        if (heights[l] <= heights[r]) { l += 1; }
        else { r = if (r != 0) r - 1 else 0; }
    }
    return res;
}

test "lc_11_test" {
    const TestCase = struct {
        heights: []const i32, 
        expected: i32
    };

    const testcases = [_]TestCase{
        .{
            .heights = &.{1, 8, 6, 2, 5, 4, 8, 3, 7},
            .expected = 49
        },
        .{
            .heights = &.{1, 1},
            .expected = 1
        }
    };

    for (testcases) | tc | { try std.testing.expectEqual(tc.expected, max_area(tc.heights)); }
}
