const std = @import("std");

pub fn group_anagrams(arena: std.mem.Allocator, strs: []const []const u8) ![]const []const []const u8 {
     var map = std.AutoHashMap([26]u8, std.ArrayList([]const u8)).init(arena);
     
     for (strs) |s| {
         var counter = [_]u8{0} ** 26;
         for (s) |c| { counter[c - 'a'] += 1; }

         const gop = try map.getOrPut(counter);
         if (!gop.found_existing) { gop.value_ptr.* = std.ArrayList([]const u8).init(arena); }

         try gop.value_ptr.append(s);
     }
    
     var res = try arena.alloc([][]const u8, map.count()); 
     var it = map.valueIterator();
     var i:usize = 0; 

     while (it.next()) |group_list| {
        res[i] = try group_list.toOwnedSlice();
        std.mem.sort([]const u8, res[i], {}, comptime stringLessThan);
        i += 1;
     }
     return res;
} 


fn stringLessThan(_: void, a: []const u8, b: []const u8) bool { return std.mem.lessThan(u8, a, b); }

test "lc_49_test" {
    const TestCase = struct{
        strs: []const []const u8,
        expected: []const []const []const u8
    };

    const testcases = [_]TestCase{
        .{
            .strs = &.{"eat","tea","tan","ate","nat","bat"},
            .expected = &.{ 
                &.{"bat"}, 
                &.{"nat","tan"}, 
                &.{"ate","eat","tea"} 
            }
        },
        .{
            .strs = &.{""},
            .expected = &.{ 
                &.{""} 
            }
        },
        .{
            .strs = &.{"a"},
            .expected = &.{ 
                &.{"a"} 
            }
        }
    };

    for (testcases) |tc| {
        var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
        defer arena.deinit();
        
        const res = try group_anagrams(arena.allocator(), tc.strs);
        
        try std.testing.expectEqual(tc.expected.len, res.len);
        for (res, tc.expected) | res_inner, tc_inner | { try std.testing.expectEqualSlices([]const u8, tc_inner, res_inner); }
    }
}
