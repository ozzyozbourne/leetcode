const std = @import("std");

pub fn ways_to_split_array(nums: []const i32)i32{
    var left:i32, var total: i32, var count: i32 = .{0, 0, 0};
    for(nums) |num| {
        total += num;
    }
    for(0..nums.len-1) |i| {
        left += nums[i];
        total -= nums[i];
        if (left >= total) {
            count += 1;
        }
    }
    return count;
}

test "lc_2270_test" {

    const TestCase = struct {
        s: []const i32,
        expected: i32
    };

    const testcases = [_]TestCase{
        .{ 
            .s = &[_]i32{10,4,-8,7},
            .expected = 2
        },
        .{
            .s = &[_]i32{2, 3, 1, 0},
            .expected = 2
        },
    }; 

    for(testcases) |t|{ try std.testing.expectEqual( t.expected, ways_to_split_array(t.s) ); }
}

