const std = @import("std");

pub fn unqual_triplets(gpa: std.mem.Allocator, nums: []const i32) i32 {
    var res: i32, var left: i32, var right: i32 = .{ 0, 0, @intCast(nums.len) };
    var counter = std.AutoHashMap(i32, i32).init(gpa);
    defer counter.deinit();

    for (nums) |n| { 
        const gop = counter.getOrPut(@intCast(n)) catch unreachable;
        if (gop.found_existing) { gop.value_ptr.* += 1; }
        else { gop.value_ptr.* = 1; }
    }
    
    var it = counter.valueIterator();
    while (it.next()) |frq| {
        right -= frq.*; 
        res += left * frq.* * right;
        left += frq.*;
    }

    return res;
}

test "lc_2475_tests" {
    const testcases = [_]struct{
        nums: []const i32, 
        expected: i32
    }{
        .{
            .nums = &.{4, 4, 2, 4, 3},
            .expected = 3
        },
        .{
            .nums = &.{1, 1, 1, 1},
            .expected = 0
        }
    };

    for(testcases) |tc| {
        try std.testing.expectEqual(tc.expected, unqual_triplets(std.testing.allocator, tc.nums));
    }
}
