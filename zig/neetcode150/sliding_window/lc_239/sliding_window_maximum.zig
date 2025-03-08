const std = @import("std");

pub fn max_sliding_window(gpa: std.mem.Allocator, nums: []const i32, k: i32) ![]const i32 {
    var deque = std.ArrayList(usize).init(gpa);
    defer deque.deinit();

    var l: usize = 0;
    var res = std.ArrayList(i32).init(gpa); 

    for (nums, 0..) |n, r| {
        while (deque.items.len > 0 and nums[deque.items[deque.items.len - 1]] < n) { _= deque.pop(); }
        try deque.append(r);
        
        if (l > deque.items[0]) { _= deque.orderedRemove(0); }

        if (r + 1 >= k) {
            try res.append(nums[deque.items[0]]);
            l += 1;
        }
    }
    return res.toOwnedSlice();
}

test "lc_239_tests" {
    const testcases = [_]struct {
        nums: []const i32, 
        k: i32,
        expected: []const i32
    } {
        .{
            .nums = &.{1, 3, -1, -3, 5, 3, 6, 7},
            .k = 3, 
            .expected = &.{3, 3, 5, 5, 6, 7}
        },
        .{
            .nums = &.{1},
            .k = 1,
            .expected = &.{1}
        }
    };

    for (testcases) |tc| {
        const res = try max_sliding_window(std.testing.allocator, tc.nums, tc.k);
        defer std.testing.allocator.free(res);

        try std.testing.expectEqualSlices(i32, tc.expected, res);
    }
}
