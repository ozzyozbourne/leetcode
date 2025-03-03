const std = @import("std");

pub fn character_replacement(gpa: std.mem.Allocator, s: []const u8, k: usize) usize {
    var l: usize, var res: usize, var maxf: usize = .{ 0, 0, 0 };

    var counter = std.AutoHashMap(u8, usize).init(gpa);
    defer counter.deinit();

    for (s, 0..) |c, r| {
       const gop = counter.getOrPut(c) catch unreachable;
       if (gop.found_existing) { gop.value_ptr.* += 1; }
       else { gop.value_ptr.* = 1; }
       maxf = @max(maxf, gop.value_ptr.*);
       while (r - l + 1 - maxf > k) { 
           const ptr = counter.getPtr(s[l]); 
           ptr.?.* -= 1;
           l += 1; 
       }
       res = @max(res, r - l + 1);
    }

    return res;
}

test "lc_424_test" {
    const testcases = [_]struct{
        s: []const u8, 
        k: usize, 
        expected: usize
    } {
        .{
            .s = "ABAB",
            .k = 2, 
            .expected = 4
        },
        .{
            .s = "AABABBA",
            .k = 1, 
            .expected = 4
        }
    };

    for (testcases) |tc| {
        try std.testing.expectEqual(tc.expected, character_replacement(std.testing.allocator, tc.s, tc.k));
    }
}
