const std = @import("std");

pub fn length_of_the_longest_substring(gpa: std.mem.Allocator, s: []const u8) i32 {
    var map = std.AutoHashMap(u8, usize).init(gpa);
    defer map.deinit();

    var l: usize, var res: i32 = .{ 0, 0 };
    for (s, 0..) |c, i| {
       if (map.contains(c)) { l = @max(map.get(c).? + 1, l); } 
       map.put(c, i) catch unreachable;
       res = @max(res, @as(i32, @intCast(i - l + 1)));
    }
    return res;
}

test "lc_1_test" {
    const testcases = [_]struct{
        input: []const u8,
        expected: i32
    }{
        .{
            .input = "abcabcbb",
            .expected = 3
        },
        .{
            .input = "bbbb",
            .expected = 1
        },
        .{
            .input = "pwwkew",
            .expected = 3
        }
    };

    for (testcases) |tc| { 
        try std.testing.expectEqual(tc.expected, length_of_the_longest_substring(std.testing.allocator, tc.input)); 
    }
}
