const std  = @import("std");


test "lc_87" {
    const testcases = [_]struct{
        s1: []const u8,
        s2: []const u8, 
        expected: bool
    }{
        .{
            .s1 = "great",
            .s2 = "rgeat",
            .expected = true
        },
        .{
            .s1 = "abcde",
            .s2 = "caebd",
            .expected = false
        },
        .{
            .s1 = "a",
            .s2 = "a",
            .expected = true
        }
    };
    
    for (testcases) |tc| {
        var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
        defer arena.deinit();
        try std.testing.expectEqual(tc.expected, is_scramble_bottom_up(arena.allocator(), tc.s1, tc.s2));
        try std.testing.expectEqual(tc.expected, is_scramble_top_down(arena.allocator(), tc.s1, tc.s2));
    }

}
