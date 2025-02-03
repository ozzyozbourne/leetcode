const std = @import("std");

pub fn shifting_letters(gpa: std.mem.Allocator, s: []const u8, shifts: []const [3]i32) ![]const u8{
    var diff: []i32, var shift_sum: i32, var res: []u8 = .{ try gpa.alloc(i32, s.len + 1), 0, try gpa.alloc(u8, s.len) };
    defer gpa.free(diff);
    @memset(diff, 0);

    for (shifts) |shift|  {
        diff[@intCast(shift[0])] += if (shift[2] == 1) 1 else -1;
        diff[@intCast(shift[1] + 1)] += if (shift[2] == 1) -1 else 1;
    }
    
    for (s, 0..) |c, i| {
        const index: i32 = @intCast(c - 'a');
        shift_sum += diff[i];
        res[i] = 'a' + @as(u8, @intCast(@mod(index + shift_sum, 26)));
    }

    return res;
}

test "lc_2381_test" {

    const TestCase = struct {
        s: []const u8,
        shifts: []const [3]i32,
        expected: []const u8
    };

    const testcases = [_]TestCase{
        .{
            .s = "abc",
            .shifts = &[_][3]i32{ .{0, 1, 0}, .{1, 2, 1}, .{0, 2, 1} },
            .expected = "ace"
        },
        .{
            .s = "dztz",
            .shifts = &[_][3]i32{ .{0, 0, 0}, .{1, 1, 1} },
            .expected = "catz"
        }
    };

    for(testcases) |t| {
        const res = try shifting_letters(std.testing.allocator, t.s, t.shifts);
        defer std.testing.allocator.free(res);
        
        try std.testing.expectEqualSlices(u8, t.expected, res);
    }
}
