const std = @import("std");

pub fn word_break(gpa: std.mem.Allocator, s: []const u8, word_dict: []const []const u8) !bool {


}

test "lc_139_tests" {
    const testcases = [_]struct{
        s: []const u8,
        word_dict: []const []const u8,
        expected: bool
    }{
        .{
            .s = "leetcode",
            .word_dict = &.{"leet", "code"},
            .expected= true
        },
        .{
            .s = "applepenapple",
            .word_dict = &.{"apple", "pen"},
            .expected= true
        },
        .{
            .s = "catsandog",
            .word_dict = &.{"cats", "dog", "sand", "and", "cat"},
            .expected= true
        }
    };

    for (testcases) |tc| { try std.testing.expectEqual(tc.expected, try word_break(std.testing.allocator, tc.s, tc.word_dict)); }
}
