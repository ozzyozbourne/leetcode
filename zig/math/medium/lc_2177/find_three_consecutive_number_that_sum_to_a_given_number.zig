const std = @import("std");

pub fn number_of_three(gpa: std.mem.Allocator, n: i32) ![]const i32 {
    if (@rem(n, 3) != 0) {
        return try gpa.alloc(i32, 0);
    } else {
        const res = try gpa.alloc(i32, 3);
        res[1] = @divTrunc(n, 3);
        res[0] = res[1] - 1;
        res[2] = res[1] + 1;
        return res;
    }
}

test "lc_2177_test" {
    const TestCase = struct {
        n: i32,
        expected: []const i32
    };
    
    const testcases = [_]TestCase{
        .{
            .n = 33,
            .expected = &[_]i32{10,11,12}
        },
        .{
            .n = 4,
            .expected = &[_]i32{}
        }
    };

    for (testcases) |t| {
        const res = try number_of_three(std.testing.allocator, t.n);
        defer std.testing.allocator.free(res);

        try std.testing.expectEqualSlices(i32, t.expected, res);
    }
}
