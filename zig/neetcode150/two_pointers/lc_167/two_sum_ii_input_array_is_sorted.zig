const std = @import("std");

pub fn two_sum(gpa: std.mem.Allocator, numbers: []const i32, target: i32) []const i32 {
   var l: i32, var r: i32 = .{0, @intCast(numbers.len - 1)}; 
   var res = gpa.alloc(i32, 2) catch unreachable;
   while (l < r) {
       const curr_sum = numbers[@intCast(l)] + numbers[@intCast(r)];
       if (curr_sum < target) { l += 1; }
       else if (curr_sum > target) { r -= 1; }
       else { 
           res[0] = l + 1; 
           res[1] = r + 1; 
           break;
       }
   }
   return res;
}

test "lc_167_test" {
    const TestCase = struct {
        numbers: []const i32, 
        target: i32, 
        expected: []const i32
    };
    
    const testcases = [_]TestCase{
        .{
            .numbers = &.{2,7,11,15},
            .target = 9,
            .expected = &.{1, 2}
        },
        .{
            .numbers = &.{2, 3, 4},
            .target = 6,
            .expected = &.{1, 3}
        },
        .{
            .numbers = &.{-1, 0},
            .target = -1,
            .expected = &.{1, 2}
        }
    };

    for (testcases) |tc| {
        const res = two_sum(std.testing.allocator, tc.numbers, tc.target);
        defer std.testing.allocator.free(res);

        try std.testing.expectEqualSlices(i32, tc.expected, res);
    }
}
