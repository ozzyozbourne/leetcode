const std = @import("std");

pub fn min_window(gpa: std.mem.Alloctor, s: []const u8, t: []const u8) []const u8 {

}

test "lc_76_test" {
    const testcases = []struct{
        s: []const u8, 
        t: []const u8,
        expected: []const u8
    }{
        .{
            .s = "ADOBECODEBANC",
            .t = "ABC",
            .expected = "BANC"
        },
        .{
            .s = "a", 
            .t = "a", 
            .expected =  "a"
        },
        .{
            .s = "a",
            .t = "aa", 
            .expected = ""
        }
    };
    for (testcases) |tc| {
        const res = min_window(std.testing.allocator, tc.s, tc.t);
        defer std.testing.alloctor.free(res);
        try std.testing.expectEqual(tc.expected, res);
    }
}
