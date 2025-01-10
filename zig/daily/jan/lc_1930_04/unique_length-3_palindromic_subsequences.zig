const std = @import("std");

pub fn count_palindromic_subsequences(gpa: std.mem.Allocator, s: []const u8) !i32 {
    var f, var l, var res:i32 = .{ [_]i32{-1} ** 26, [_]i32{-1} ** 26, 0 };

    for (s, 0..) |c, i| {
        const cur = c - 'a'; 
        if(f[cur] == -1){ f[cur] = @intCast(i); }
        l[cur] = @intCast(i);
    }

    var set = std.AutoHashMap(u8, void).init(gpa);
    defer set.deinit();

    for(0..26) |i|{
        if (f[i] == -1) { continue; }

        set.clearRetainingCapacity();
        var start: usize, const end: usize = .{ @intCast(f[i] + 1), @intCast(l[i]) };

        while(start < end): (start += 1) { try set.put(s[start], {}); }
        res += @intCast(set.count());
    }

    return res;
}

test "lc_1930_test" {

    const TestCase = struct {
        s: []const u8,
        expected: i32
    };

    const testcases = [_]TestCase{
        .{ 
            .s = "aabca",
            .expected = 3
        },
        .{ 
            .s = "adc",
            .expected = 0
        },
        .{
            .s = "bbcbaba",
            .expected = 4
        }
    }; 

    for(testcases) |t|{ try std.testing.expectEqual( t.expected, count_palindromic_subsequences(std.testing.allocator, t.s) ); }
}

