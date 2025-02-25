const std = @import("std");

pub fn encode(arena: std.mem.Allocator, input: []const []const u8) ![]const u8 {
    var res = std.ArrayList(u8).init(arena); // array list of bytes
    for (input) |s| {
        try res.appendSlice(try std.fmt.allocPrint(arena, "{d}", .{s.len}));
        try res.append('#');
        try res.appendSlice(s);
    }
    return try res.toOwnedSlice();
}

pub fn decode(arena: std.mem.Allocator, input: []const u8) ![]const []const u8 {
   var res = std.ArrayList([]const u8).init(arena);
   var i: usize = 0;
   while (i < input.len) {
       var j = i;
       while (input[j] != '#') { j += 1; }
       const len = try std.fmt.parseInt(usize, input[i..j], 10);
       i = j + 1;
       j = i + len;
       try res.append(input[i..j]);
       i = j;
   }
   return try res.toOwnedSlice();
}

test "lc_premium" {
    const TestCase = struct {
        input: []const []const u8,
        expected: []const []const u8
    };

    const testcases = [_]TestCase{
        .{
            .input = &.{"neet","code","love","you"},
            .expected = &.{"neet","code","love","you"}
        },
        .{
            .input = &.{"we","say",":","yes"},
            .expected = &.{"we","say",":","yes"}
        }
    };

    for (testcases) |tc| {
        var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
        defer arena.deinit();

        const encoded = try encode(arena.allocator(), tc.input);
        const decoded = try decode(arena.allocator(), encoded);

        try std.testing.expectEqual(tc.expected.len, decoded.len);

        for (tc.expected, decoded) |expected_str, decoded_str| {
            try std.testing.expectEqualStrings(expected_str, decoded_str);
        }
    }
}
