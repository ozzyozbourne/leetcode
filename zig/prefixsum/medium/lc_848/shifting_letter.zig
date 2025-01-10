const std = @import("std");

pub fn shifting_letter(gpa: std.mem.Allocator, s: []const u8, shifts: []const i32 ) ![]const u8 {
    var ans, var x: i32 = .{ std.ArrayList(u8).init(gpa), 0 };

    for (shifts) |shift| { x += shift; }
    x = @mod(x, 26); 

    for (s, 0..) |c, i| {
        const index:i32 = @intCast(c - 'a');
        const char:u8 = @intCast(@mod(index + x, 26));
        try ans.append('a' + char);
        x = @mod(x - shifts[i], 26);
    }

    return ans.toOwnedSlice();
}

test "lc_848_test" {

    const TestCase = struct {
        s: []const u8,
        shifts: []const i32,
        expected: []const u8
    };

    const testcases = [_]TestCase{
        .{ 
            .s = "abc",
            .shifts = &[_]i32{ 3, 5, 9 },
            .expected = "rpl"
        },
        .{ 
            .s = "aaa",
            .shifts = &[_]i32{ 1, 2, 3 },
            .expected = "gfd"
        }
    }; 

    for(testcases) |t|{ 
        const res = try shifting_letter(std.testing.allocator, t.s, t.shifts);
        defer std.testing.allocator.free(res);

        try std.testing.expectEqualStrings( t.expected,  res); 
    }
}
