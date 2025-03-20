const std = @import("std");

pub fn max_increasing(usage_limits: []i32) i32 {
    std.mem.sort(i32, usage_limits, {}, std.sort.asc(i32));
    var count: i32, var sum: i32 = .{0, 0};

    for (usage_limits) |val| {
        sum += val;
        if (sum >= @divTrunc((count + 1) * (count + 2), 2)) { count += 1; }
    }
    return count;
}

test "lc_2790_test" {
    var var1 = [_]i32{2, 1, 2};
    var var2 = [_]i32{1, 2, 5};
    var var3 = [_]i32{1, 1};

    const testcases = [_]struct{
        usage_limits: []i32,
        expected: i32
    }{
        .{
            .usage_limits = &var1,
            .expected = 2
        },
        .{
            .usage_limits = &var2,
            .expected = 3
        },
        .{
            .usage_limits = &var3,
            .expected = 1
        }
    };

    for(testcases) |tc| { try std.testing.expectEqual(tc.expected , max_increasing(tc.usage_limits)); }
}

