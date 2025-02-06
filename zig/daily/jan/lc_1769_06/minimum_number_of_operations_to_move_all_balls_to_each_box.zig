const std = @import("std");

pub fn min_operations(gpa: std.mem.Allocator, boxes: []const u8) ![]const i32{
    var cnt:i32, var ops:i32, var res:[]i32 = .{0, 0, try gpa.alloc(i32, boxes.len)}; 
    @memset(res, 0);

    for (boxes, 0..) |box, i| {
        res[i] += ops;
        cnt += if (box == '1') 1 else 0;
        ops += cnt;
    }

    cnt, ops = .{0, 0};

    var i:i32 = @intCast(boxes.len - 1);
    while(i >= 0):(i -= 1)  {
        res[@intCast(i)] += ops;
        cnt += if (boxes[@intCast(i)] == '1') 1 else 0;
        ops += cnt;
    }

    return res;
}

test "lc_1769_test" {

    const TestCase = struct {
        boxes: []const u8,
        expected: []const i32,
    };

    const testcases = [_]TestCase{
        .{
            .boxes = "110",
            .expected = &[_]i32{1, 1, 3}
        },
        .{
            .boxes = "001011",
            .expected = &[_]i32{11,8,5,4,3,4}
        }
    };

    for (testcases) |t| {
        const res = try min_operations(std.testing.allocator, t.boxes);
        defer std.testing.allocator.free(res);
        try std.testing.expectEqualSlices(i32, t.expected, res);
    }
}
