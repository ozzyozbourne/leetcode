const std = @import("std");

pub fn check_inclusion(s1: []const u8, s2: []const u8) bool {
    if (s1.len > s2.len) { return false; }
    var s1_count: [26]i32, var s2_count: [26]i32, var matches: usize, var l: usize = 
        .{ [_]i32{0} ** 26, [_]i32{0} ** 26, 0, 0 };
    
    for(0..s1.len) |i| {
        s1_count[s1[i] - 'a'] += 1;
        s2_count[s2[i] - 'a'] += 1;
    }

    for (s1_count, s2_count) |s1_c, s2_c| { matches += if (s1_c == s2_c) 1 else 0; }

    for (s1.len..s2.len) | r | {
        if (matches == 26) { return true; }

        var index = s2[r] - 'a';
        s2_count[index] += 1;
        if (s2_count[index] == s1_count[index]) { matches += 1; }
        else if (s1_count[index] + 1 == s2_count[index]) { matches -= 1; }

        index = s2[l] - 'a';
        s2_count[index] -= 1;
        if (s2_count[index] == s1_count[index]) { matches += 1; }
        else if (s1_count[index] - 1 == s2_count[index]) { matches -= 1; }
        
        l += 1; 
    }
    return matches == 26;
}

test "lc_567_test" {
    const testcases = [_]struct {
        s1: []const u8,
        s2: []const u8,
        expected: bool
    }{
        .{
            .s1 = "ab",
            .s2 = "eidbaooo",
            .expected = true
        },
        .{
            .s1 = "ab",
            .s2 = "eidboaoo",
            .expected = false
        }
    };

    for (testcases) |tc| { try std.testing.expectEqual(tc.expected , check_inclusion(tc.s1, tc.s2)); }
}
