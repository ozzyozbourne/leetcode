const std = @import("std");

pub fn constains_duplicate(gpa: std.mem.Allocator, nums: []const i32) bool {
    var hash_map = std.AutoHashMap(i32, void).init(gpa);
    defer hash_map.deinit();

    for (nums) |num| {
        if (hash_map.contains(num)) { return true; }
        hash_map.put(num, {}) catch unreachable;
    }

    return false;
}

test "lc_217_test" {
    const TestCase = struct {
        i: []const i32,
        expected: bool
    };
    const testcases = [_]TestCase{
        .{
            .i = &[_]i32{1,2,3,1},
            .expected = true

        },
        .{
            .i = &[_]i32{1,2,3,4},
            .expected = false

        },
        .{
            .i = &[_]i32{1,1,1,3,3,4,3,2,4,2},
            .expected = true

        }
    };
    for (testcases) |tc| { try std.testing.expectEqual(tc.expected, constains_duplicate(std.testing.allocator, tc.i)); }
}
