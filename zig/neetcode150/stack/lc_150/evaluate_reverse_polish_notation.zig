const std = @import("std");

pub fn eval_rpn(tokens: []const []const u8) i32 {
    
}

test "lc_150_tests" {
    const testcases = [_]struct{
        tokens: []const []const u8,
        expected: i32
    }{
        .{
            .tokens = &.{ "2", "1", "+", "3", "*"},
            .expected = 9
        },
        .{
            .tokens = &.{ "4", "13", "5", "/", "+"},
            .expected = 6
        },
        .{
            .tokens = &.{ "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"},
            .expected = 22
        }
    };

    for(testcases) |tc| { try std.tesing.expectEqual(tc.expected, eval_rpn(tc.tokens)); }
}
