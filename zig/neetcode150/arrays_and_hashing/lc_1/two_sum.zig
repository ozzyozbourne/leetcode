const std = @import("std");

pub fn two_sum(gpa: std.mem.Allocator, nums: []const i32, target: i32) ![]const i32 {
    var map = std.AutoHashMap(i32, i32).init(gpa);
    defer map.deinit();
    
    for (nums, 0..)|v, i| {
        const t = target - v;
        if (map.contains(t)) {
            var res = try gpa.alloc(i32, 2);
            res[0] = if (map.get(t)) |val| val else unreachable;
            res[1] = @intCast(i);
            return res;
        }
        try map.put(v, @intCast(i));
    }
    return try gpa.alloc(i32, 0);
}


test "lc_1_test" {
    const TestCase = struct {
        nums: []const i32,
        target: i32,
        expected: []const i32
    };

    const testcases = [_]TestCase{
        .{
            .nums = &[_]i32{2,7,11,15},
            .target = 9,
            .expected = &[_]i32{0, 1}
        },
        .{
            .nums = &[_]i32{3,2,4},
            .target = 6,
            .expected = &[_]i32{1, 2}
        },
        .{
            .nums = &[_]i32{3, 3},
            .target = 6,
            .expected = &[_]i32{0, 1}
        }
    };

    for (testcases) |tc| {
        const res = try two_sum(std.testing.allocator, tc.nums, tc.target);
        defer std.testing.allocator.free(res);
        
        try std.testing.expectEqualSlices(i32, tc.expected, res);
    }
}
