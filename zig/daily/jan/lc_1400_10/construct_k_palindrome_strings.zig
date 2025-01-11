const std = @import("std");

pub fn can_construct(s: []const u8, k: i32) bool {
   const k_usize:usize = @intCast(k); 

   if (s.len < k_usize) { return false; }
   else if (s.len == k_usize) { return true; }
   else {
       var odd_count: u26 = 0;
       for (s) |c| {
           odd_count ^= @as(u26, 1) << @as(u5, @intCast(c - 'a'));
       }
       return @as(usize, @popCount(odd_count)) <= k_usize;
   }
}

test "lc_1400_test" {
    const TestCase = struct {
        s: []const u8,
        k: i32,
        expected: bool
    };

    const testcases = [_]TestCase{
        .{ 
            .s = "annabelle",
            .k = 2,
            .expected = true
        },
        .{ 
            .s = "leetcode",
            .k = 3,
            .expected = false
        },
        .{ 
            .s = "true",
            .k = 4,
            .expected = true
        }
    }; 

    for(testcases) |t|{ try std.testing.expectEqual( t.expected, can_construct(t.s, t.k) ); }
}

