const std = @import("std");

pub fn product_except_self(gpa: std.mem.Allocator, nums: []const i32) ![]const i32 {
    var res = try gpa.alloc(i32, nums.len);
    @memset(res, 0);

    var prefix: i32, var postfix: i32, var j = .{ 1, 1, @as(i32, @intCast(nums.len)) - 1};

    for (nums, 0..) |n, i| {
        res[i] = prefix;
        prefix *= n;
    }
    var it = std.mem.reverseIterator(nums);

    while (it.next()) |n| {
        res[@intCast(j)] *= postfix;
        postfix *= n;
        j -= 1;
    }
    return res;
}

test "lc_238_test" {
    const TestCase = struct {
        nums: []const i32,
        expected: []const i32
    };

    const testcases = [_]TestCase{
        .{
            .nums = &.{1,2,3,4},
            .expected = &.{24,12,8,6}
        },
        .{
            .nums = &.{-1,1,0,-3,3},
            .expected = &.{0,0,9,0,0}
        }
    };

    for (testcases) |tc| {
        const res = try product_except_self(std.testing.allocator, tc.nums);
        defer std.testing.allocator.free(res);

        try std.testing.expectEqualSlices(i32, tc.expected, res);
    }
}
