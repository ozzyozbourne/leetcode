const std = @import("std");

pub fn is_anagram(s: []const u8, t: []const u8) bool {
    if (s.len != t.len){ return false; }
    var counter = [_]i32{0} ** 26; 

    for (s, t)|sc, tc| { 
        counter[sc - 'a'] += 1;
        counter[tc - 'a'] -= 1;
    }

    for(counter)|val| {
        if (val != 0){ return false; }
    }

    return true;
}


test "lc_242_test" {

    const TestCase = struct{
        s: []const u8,
        t: []const u8,
        expected: bool
    };

    const testcases = [_]TestCase{
        .{
            .s = "anagram",
            .t = "nagaram",
            .expected = true
        },
        .{
            .s = "rat",
            .t = "car",
            .expected = false
        }
    };
    
    for (testcases) |tc| { try std.testing.expectEqual(tc.expected, is_anagram(tc.s, tc.t)); }
}
