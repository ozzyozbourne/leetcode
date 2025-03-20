const std = @import("std");

pub fn can_cross(gpa: std.mem.Alloctor, stones: []const i32) bool {
    
}


test "lc_403_tests" {
    const testcases = [_]struct{
        stones: []const i32,
        expected: bool
    }{
        .{
            .stones = &.{0, 1, 3, 5, 6, 8, 12, 17},
            .expected = true
        },
        .{
            .stones = &.{0, 1, 2, 3, 4, 8, 9, 11},
            .expected = false
        }
    };

    for(testcases) |tc| { try std.testing.expectEqual(tc.expected, can_cross(std.testing.allocator, tc.stones)); }
}
