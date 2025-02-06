const std = @import("std");

pub fn word_subsets(gpa: std.mem.Allocator, w1: []const []const u8, w2: []const []const u8) ![]const []const u8 {
   const count = struct {
       fn f(gpa_1: std.mem.Allocator, w:[]const u8) ![]i32 {
            var freq = try gpa_1.alloc(i32, 32);
            @memset(freq, 0);
            for(w) |c| { freq[@intCast(c - 'a')] += 1; }
            return freq;
       }
   }.f;

   const all = struct {
       fn f(a: []const i32, b: []const i32) bool {
           for(0..26) |i|{
               if (a[i] < b[i]) { return false; }
           }
           return true;
       }
   }.f;

   var bmax = try gpa.alloc(i32, 26); 
   defer gpa.free(bmax);
   @memset(bmax, 0);

   var res = std.ArrayList([]const u8).init(gpa);
   defer res.deinit();

   for(w2) |w| {
       const freq = try count(gpa, w);
       defer gpa.free(freq);

       for(0..26) |i| { bmax[i] = @max(bmax[i], freq[i]); }
   }

   for(w1) |w| {
       const freq = try count(gpa, w);
       defer gpa.free(freq);

       if(all(freq, bmax)) { try res.append(w); }
   } 

   var ans = try gpa.alloc([]const u8, res.items.len);
   for (res.items, 0..) |v, i| {
        ans[i] = v;
   }
   return ans;

}

test "lc_816_test" {

    const TestCase = struct {
        w1: []const []const u8,
        w2: []const []const u8,
        expected: []const []const u8,
    };

    const testcases = [_]TestCase{
        .{ 
            .w1 = &[_][]const u8 { "amazon", "apple", "facebook", "google", "leetcode" },
            .w2 = &[_][]const u8 { "e", "o" },
            .expected = &[_][]const u8 { "facebook", "google", "leetcode" }
        },
        .{ 
            .w1 = &[_][]const u8 { "amazon", "apple", "facebook", "google", "leetcode" },
            .w2 = &[_][]const u8 { "lc", "eo" },
            .expected = &[_][]const u8 { "leetcode" }
        },
        .{ 
            .w1 = &[_][]const u8 { "acaac", "cccbb", "aacbb", "caacc", "bcbbb" },
            .w2 = &[_][]const u8 { "c", "cc", "b" },
            .expected = &[_][]const u8 { "cccbb" }
        }
    }; 
    

    for(testcases) |t|{
        const res = try word_subsets(std.testing.allocator, t.w1, t.w2);
        defer std.testing.allocator.free(res);

        try std.testing.expectEqual(res.len, t.expected.len);
        for (res, t.expected) |actual, expected| { try std.testing.expectEqualSlices(u8, actual, expected); }
    }
}

