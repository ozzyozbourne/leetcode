const std = @import("std");

pub fn max_score(s: []const u8)i32 {
    var ones:i32, var zeros:i32, var best:i32, var i:usize = .{0, 0, std.math.minInt(i32), 0};

    while (i < s.len - 1): (i += 1) {
        if (s[i] == '1') { ones += 1; }
        else { zeros += 1; }
        best = @max(best, zeros - ones);
    }

    if (s[s.len - 1] == '1') { ones += 1; } 
    return best + ones;
    
}
test "lc_1422_test" {

    const TestCase = struct {
        s: []const u8,
        expected: i32
    };

    const testcases = [_]TestCase{
        .{ 
            .s = "011101",
            .expected = 5
        },
        .{
            .s = "00111",
            .expected = 5
        },
        .{
            .s = "1111",
            .expected = 3
        },
    }; 

    for(testcases) |t|{ try std.testing.expectEqual( t.expected, max_score(t.s) ); }
}

