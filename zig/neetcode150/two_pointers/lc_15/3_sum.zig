const std = @import("std");

pub fn three_sum(gpa: std.mem.Allocator, nums: []i32) ![]const [3]i32 {
    std.mem.sort(i32, nums, {}, std.sort.asc(i32));
    var res = std.ArrayList([3]i32).init(gpa);

    for (nums, 0..) |n, i| {
        if (n > 0) { break; }
        if (i > 0 and n == nums[i - 1]) { continue; }

        var l: usize, var r: usize = .{ i + 1, nums.len - 1 };
        while (l < r) {
            const sum = n + nums[l] + nums[r];
            if (sum > 0) { r -= 1; }
            else if (sum < 0) { l += 1; }
            else { 
                try res.append([_]i32{n, nums[l], nums[r]}); 
                l += 1; 
                r -= 1;
                while (nums[l] == nums[l - 1] and l < r) { l += 1; }
            }
        }
    }
    return try res.toOwnedSlice();   
}

test "lc_15_test" {
    const TestCase = struct {
        nums: []i32,
        expected: []const [3]i32
    };

    var arr1 = [_]i32{-1, 0, 1, 2, -1, -4};
    var arr2 = [_]i32{0, 1, 1};
    var arr3 = [_]i32{0, 0, 0};

    const testcases = [_]TestCase{
        .{
            .nums = &arr1,
            .expected = &.{
                .{-1, -1, 2},
                .{-1, 0, 1}
            }
        },
        .{
            .nums = &arr2,
            .expected = &.{}
        },
        .{
            .nums = &arr3,
            .expected = &.{.{0, 0, 0}}
        }
    };

    for (testcases) |tc| {
        const res = try three_sum(std.testing.allocator, tc.nums);
        defer std.testing.allocator.free(res);
        
        try std.testing.expectEqualSlices([3]i32, tc.expected, res);
    }
}
