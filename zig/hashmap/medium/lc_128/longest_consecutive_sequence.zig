const std = @import("std");

pub fn longest_consective(gpa: std.mem.Allocator, nums: []const i32) !i32 {
    var set: std.AutoHashMap(i32, void), var longest: i32 = .{std.AutoHashMap(i32, void).init(gpa), 0};
    defer set.deinit();
    
    for (nums) |n| {
        try set.put(n, {});
    }

    var it = set.keyIterator();
    while (it.next()) |n| {
        if (!set.contains(n.* - 1)) {
            var length:i32 = 1;
            while (set.contains(n.* + length)) { length += 1; }
            longest = @max(longest, length);
        }
    }

    return longest;
}

test "lc_128_test" {
    const TestCase = struct {
        nums: []const i32,
        expected: i32
    };

    const testcases = [_]TestCase{
        .{
            .nums = &.{100,4,200,1,3,2},
            .expected = 4
        },
        .{
            .nums = &.{0,3,7,2,5,8,4,6,0,1},
            .expected = 9
        }
    };

    for (testcases) |t| {
        try std.testing.expectEqual(t.expected, try longest_consective(std.testing.allocator, t.nums));
    }
}
