const std = @import("std");

pub fn find_all_concatenated_words_in_a_dict(gpa: std.mem.Allocator, words: []const []const u8) ![]const []const u8 {
    var map = std.StringHashMap(void).init(gpa);
    defer map.deinit();
    var res = std.ArrayList([]const u8).init(gpa);

    for (words) |w| { try map.put(w,{}); }

    for (words) |word| {
        var dp = try gpa.alloc(bool, word.len + 1);
        @memset(dp, false);
        dp[0] = true;

        for (1..dp.len) |i| {
            var j:usize = if (i == word.len) 1 else 0;
            while (!dp[i] and j < i) {
                dp[i] = dp[j] and map.contains(word[j..i]);
                j += 1;
            }
        }
        if (dp[word.len]) { try res.append(word); }
        gpa.free(dp);
    }

    return res.toOwnedSlice();
}

test "lc_472" {
    const testcases = [_]struct{
        input: []const []const u8,
        expected: []const []const u8
    }{
        .{
            .input = &.{"cat", "cats", "catsdogcats", "dog", "dogcatsdog", "hippopotamuses", "rat", "ratcatdogcat"},
            .expected = &.{"catsdogcats", "dogcatsdog", "ratcatdogcat"}
        },
        .{
            .input = &.{ "cat", "dog", "catdog" },
            .expected = &.{ "catdog" }
        }
    };

    for (testcases) |tc| {
        const res = try find_all_concatenated_words_in_a_dict(std.testing.allocator, tc.input);
        defer std.testing.allocator.free(res);

        try std.testing.expectEqual(tc.expected.len, res.len);
        for (tc.expected, res) |exp, act| { try std.testing.expectEqualStrings(exp, act); }
    }
}
