const std = @import("std");

pub fn is_palindrome(s: []const u8) bool {
    var l: usize, var r: usize = .{0, s.len - 1};
    while (l < r) {
        while (l < r and !std.ascii.isAlphanumeric(s[l])) { l += 1; }
        while (l < r and !std.ascii.isAlphanumeric(s[r])) { r -= 1; }
        if (std.ascii.toLower(s[l]) != std.ascii.toLower(s[r])) { return false; }
        l += 1;
        r = if (r != 0) r - 1 else 0;
    }
    return true;
} 

test "lc_125_test" {
    const TestCase = struct{
        s: []const u8,
        expected: bool
    };

    const testcases = [_]TestCase{
        .{
            .s = "A man, a plan, a canal: Panama",
            .expected = true
        },
        .{
            .s = "race a car",
            .expected = false
        },
        .{
            .s = " ",
            .expected = true
        }, 
        .{
            .s = "a.",
            .expected = true
        }

    };

    for (testcases) |tc| { try std.testing.expectEqual(tc.expected, is_palindrome(tc.s)); }
}
