const std = @import("std");

pub fn min_window(gpa: std.mem.Allocator, s: []const u8, t: []const u8) ![]const u8 {
    var counter_t = std.AutoArrayHashMap(u8, usize).init(gpa);
    defer counter_t.deinit();

    for(t) |c| {
        const gop = try counter_t.getOrPut(c);
        if(gop.found_existing) { gop.value_ptr.* += 1; }
        else { gop.value_ptr.* = 1; }
    }

    var counter_s = std.AutoArrayHashMap(u8, usize).init(gpa);
    defer counter_s.deinit();
    
    var have: usize, const need:usize, var res:[2]usize, var reslen: usize, var l: usize = 
        .{0, counter_t.count(), .{0, 0}, std.math.maxInt(usize), 0};

    for (s, 0..) |c, r| {
       const gop = try counter_s.getOrPut(c); 
       if (gop.found_existing) { gop.value_ptr.* += 1; }
       else { gop.value_ptr.* = 1; }
       if (counter_t.contains(s[r]) and counter_s.get(s[r]).? == counter_t.get(s[r]).?) { have += 1; }
       while (have == need) {
            if (r - l + 1 < reslen) {
                res = .{l, r};
                reslen = r - l + 1;
            }
            counter_s.getEntry(s[l]).?.value_ptr.* -= 1;
            if (counter_t.contains(s[l]) and counter_s.get(s[l]).? < counter_t.get(s[l]).?) { have -= 1; }
            l += 1;
       }

    }
    if (reslen == std.math.maxInt(usize)){ return gpa.dupe(u8, ""); } 
    else { return gpa.dupe(u8, s[res[0]..res[1] + 1]); }
    
}

test "lc_76_test" {
    const testcases = [_]struct{
        s: []const u8, 
        t: []const u8,
        expected: []const u8
    }{
        .{
            .s = "ADOBECODEBANC",
            .t = "ABC",
            .expected = "BANC"
        },
        .{
            .s = "a", 
            .t = "a", 
            .expected =  "a"
        },
        .{
            .s = "a",
            .t = "aa", 
            .expected = ""
        }
    };
    for (testcases) |tc| {
        const res = try min_window(std.testing.allocator, tc.s, tc.t);
        defer std.testing.allocator.free(res);
        try std.testing.expectEqualStrings(tc.expected, res);
    }
}
