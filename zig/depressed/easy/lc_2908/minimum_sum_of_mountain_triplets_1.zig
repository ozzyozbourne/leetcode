const std = @import("std");

pub fn minimum_sum(gpa: std.mem.Allocator, nums: []const i32) i32 {
    var res: i32, var lm: []i32, var rm: []i32 = .{
        std.math.maxInt(i32), 
        gpa.alloc(i32, nums.len) catch unreachable, 
        gpa.alloc(i32, nums.len) catch unreachable
    };
    defer {
        gpa.free(lm);
        gpa.free(rm);
    }

    lm[0] = nums[0];
    rm[nums.len - 1] = nums[nums.len - 1];

    for (1..nums.len) |i| { lm[i] = @min(lm[i - 1], nums[i]); }

    var i:i32 = @intCast(nums.len - 2);
    while (i >= 0) : ( i -= 1) { rm[@intCast(i)] = @min(rm[@intCast(i + 1)], nums[@intCast(i)]); }

    for (1..nums.len - 1) |it| {
        if (lm[it - 1] < nums[it] and rm[it + 1] < nums[it]) { res = @min(res, nums[it] + lm[it - 1] + rm[it + 1]); }
    }
    return if (res == std.math.maxInt(i32))  -1  else  res ;
}

test "lc_2908_tests" {
    const testcases = [_]struct{
        nums: []const i32,
        expected: i32
    }{
        .{
            .nums = &.{8, 6, 1, 5, 3},
            .expected = 9
        },
        .{
            .nums = &.{5, 4, 8, 7, 10, 2},
            .expected = 13
        },
        .{
            .nums = &.{6, 5, 4, 3, 4, 5},
            .expected = -1
        }
    };
    for (testcases) |tc| {
        try std.testing.expectEqual(tc.expected, minimum_sum(std.testing.allocator, tc.nums));
    }
}
