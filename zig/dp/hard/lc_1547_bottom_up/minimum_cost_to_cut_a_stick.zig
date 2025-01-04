const std = @import("std");

pub fn min_cost(allocator:std.mem.Allocator, n:i32, cuts: []const i32 )!i32 {
    var cuts_lists = std.ArrayList(i32).init(allocator);
    defer cuts_lists.deinit();

    try cuts_lists.append(0);
    try cuts_lists.appendSlice(cuts);
    try cuts_lists.append(n);

    std.mem.sort(i32, cuts_lists.items, {}, std.sort.asc(i32));
    const m = cuts_lists.items.len;

    var dp = try allocator.alloc([]i32, m);
    defer {
        for (dp) |row| allocator.free(row);
        defer allocator.free(dp);
    }

    for (dp) |*row| {
        row.* = try allocator.alloc(i32, m);
        @memset(row.*, 0);
    }

    for (2..m) |len| {
        for (0..m - len) |i|{
            const j = i + len;
            dp[i][j] = cuts_lists.items[j] - cuts_lists.items[i];
            var min:i32 = std.math.maxInt(i32);
            for (i+1..j) |k| { min = @min(min, dp[i][k] + dp[k][j]); }
            dp[i][j] += min;
        }
    }

    return dp[0][m - 1];
}

test "lc_1547_test" {
    const TestCase = struct {
        cuts: []const i32,
        n: i32,
        expected: i32
    };

    const testcases = [_]TestCase{
        .{ 
            .cuts = &[_]i32{6, 4},
            .n = 7,
            .expected = 10
        },
        .{
            .cuts = &[_]i32{5,6,1,4,2},
            .n = 9,
            .expected = 22
        },
    }; 

    for(testcases) |t|{
        try std.testing.expectEqual(t.expected, min_cost(std.testing.allocator, t.n, t.cuts));
    }
}
