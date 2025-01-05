const std = @import("std");

pub fn vowel_strings(gpa: std.mem.Allocator, words: []const []const u8, queries: []const [2]usize) ![]i32 {
    var vowels = std.AutoHashMap(u8, void).init(gpa);
    defer vowels.deinit();

    try vowels.put('a', {});
    try vowels.put('e', {});
    try vowels.put('i', {});
    try vowels.put('o', {});
    try vowels.put('u', {});

    var prefix_vowel_count = try gpa.alloc(i32, words.len + 1);
    defer gpa.free(prefix_vowel_count);
    @memset(prefix_vowel_count, 0);

    for(words, 1..) |word, i|{
        prefix_vowel_count[i] = prefix_vowel_count[i - 1] + 
           @as(i32, if (vowels.contains(word[0]) and vowels.contains(word[word.len - 1])) 1 else 0);
    }

    var res = try gpa.alloc(i32, queries.len);

    for (queries, 0..) |query, i| {
        res[i] = prefix_vowel_count[query[1] + 1] - prefix_vowel_count[query[0]];
    }

    return res;
}

test "lc_2559_test" {

    const TestCase = struct {
        words: []const []const u8,
        queries: []const [2]usize,
        expected: []const i32,
    };

    const testcases = [_]TestCase{
        .{ 
            .words = &[_][]const u8{ "aba","bcb","ece","aa","e" },
            .queries = &[_][2]usize{ .{0, 2}, .{1, 4}, .{1, 1} },
            .expected = &[_]i32{ 2, 3, 0 }
        },
        .{ 
            .words = &[_][]const u8{ "a","e","i" },
            .queries = &[_][2]usize{ .{0, 2}, .{0, 1}, .{2, 2} },
            .expected = &[_]i32{ 3, 2, 1 }
        },
    }; 

    for(testcases) |t|{
        const res = try vowel_strings(std.testing.allocator, t.words, t.queries);
        defer std.testing.allocator.free(res);
        try std.testing.expectEqualSlices(i32 , t.expected, res);
    }
}

