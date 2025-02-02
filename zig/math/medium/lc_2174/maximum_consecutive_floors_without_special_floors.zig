const std = @import("std");

pub fn max_consecutive(top: i32, bottom: i32, special: [] i32) i32 {
    std.mem.sort(i32, special, {}, std.sort.asc(i32));
    var res:i32 = 0;

    res = @max(res, top - special[special.len - 1]);
    res = @max(res, special[0] - bottom);

    for (0..special.len - 1) |i| {
        res = @max(res, special[i + 1] - special[i] - 1);
    }
    return res;
}

test "lc_2174_test" {
    const TestCase = struct {
        bottom: i32,
        top: i32, 
        special: []i32,
        expected: i32
    };

    var s1 = [_]i32{7,8,6};
    var s2 = [_]i32{4,6};

    const testcase = [_]TestCase{
        .{
            .bottom = 6,
            .top = 8,
            .special = &s1,
            .expected = 0
        },
        .{
            .bottom = 2,
            .top = 9,
            .special = &s2,
            .expected = 3
        }
    };

    for (testcase) |t| {
        try std.testing.expectEqual(t.expected, max_consecutive(t.top, t.bottom, t.special));
    }
}
