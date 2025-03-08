const std = @import("std");

pub fn is_valid(gpa: std.mem.Allocator, s: []const u8) bool {
    var stack = std.ArrayList(u8).init(gpa);
    defer stack.deinit();

    for (s) |c| {
        switch (c) {
            ')', '}', ']' => {
                const m: u8 = switch(c) {
                    ')' => '(',
                    '}' => '{',
                    else => '['
                };
                if (stack.items.len == 0 or stack.pop() != m) { return false; }
            },
            else => { stack.append(c) catch unreachable; }
        }
    }
    return stack.items.len == 0;
}

test "lc_20_tests" {
    const testcases = [_]struct{
        s: []const u8,
        expected: bool
    }{
        .{
            .s = "()",
            .expected = true
        },
        .{
            .s = "()[]{}",
            .expected = true
        },
        .{
            .s = "(]",
            .expected = false
        },
        .{
            .s = "([])",
            .expected = true
        }
    }; 

    for (testcases) |tc| { try std.testing.expectEqual(tc.expected, is_valid(std.testing.allocator, tc.s)); } 
}
