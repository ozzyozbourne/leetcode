const std = @import("std");

pub fn top_k_frequent_using_bucket_sort(gpa: std.mem.Allocator, nums: []const i32, k: i32) ![]const i32 {
    var counter = std.AutoHashMap(i32, usize).init(gpa);
    var freq = try gpa.alloc(std.ArrayList(i32), nums.len + 1);

    for (freq) |*bucket| { bucket.* = std.ArrayList(i32).init(gpa); }
    var res = std.ArrayList(i32).init(gpa);   

    defer {
        for (freq) |*bucket| { bucket.deinit(); }
        gpa.free(freq);
        counter.deinit();
    }

    for (nums) |n| {
        const entry = try counter.getOrPut(n);
        if (entry.found_existing) { entry.value_ptr.* += 1; }
        else { entry.value_ptr.* = 1; }
    }

    var it = counter.iterator();
    while (it.next()) |entry| { try freq[entry.value_ptr.*].append(entry.key_ptr.*); } 

    var i = freq.len - 1;
    while (i > 0): (i -= 1) {
        for (freq[i].items) |n| {
            try res.append(n);
            if (res.items.len == @as(usize, @intCast(k))) { return res.toOwnedSlice(); }
        }
    }
    
    return res.toOwnedSlice();
}

pub fn top_k_frequent_using_min_heap(gpa: std.mem.Allocator, nums: []const i32, k: i32) ![]const i32 {
    var counter = std.AutoHashMap(i32, usize).init(gpa);
    defer counter.deinit();

    for (nums) |n| {
        const entry = try counter.getOrPut(n);
        if (entry.found_existing) { entry.value_ptr.* += 1; }
        else { entry.value_ptr.* = 1; }
    }

    const Item = struct {
        freq: usize,
        value: i32,

        pub fn less_than(_: void, a: @This(), b: @This()) std.math.Order {
            if (a.freq < b.freq) return .lt;
            if (a.freq > b.freq) return .gt;
            return .eq;
        }
    };

    var heap = std.PriorityQueue(Item, void, Item.less_than).init(gpa, {});
    defer heap.deinit();

    var it = counter.iterator();
    while (it.next()) |entry| {
        try heap.add(.{ .freq = entry.value_ptr.*, .value = entry.key_ptr.* });
        if (heap.count() > @as(usize, @intCast(k))) { _= heap.remove(); }
    }

    var res = try gpa.alloc(i32, @as(usize, @intCast(k)));
    for (0..heap.count()) |i| { res[i] = heap.remove().value; }    
    
    std.mem.reverse(i32, res);
    return res;
}

test "lc_374_test" {
    const TestCase = struct {
        nums: []const i32, 
        k: i32,
        expected: []const i32
    };

    const testcases = [_]TestCase{
        .{
            .nums = &.{1,1,1,2,2,3},
            .k = 2,
            .expected = &.{1, 2}
        },
        .{
            .nums = &.{1},
            .k = 1,
            .expected = &.{1}
        }
    };
    
    for (testcases) |tc| {
        const res = try top_k_frequent_using_bucket_sort(std.testing.allocator, tc.nums, tc.k);
        defer std.testing.allocator.free(res);
        
        try std.testing.expectEqualSlices(i32, tc.expected, res);
    }

    for (testcases) |tc| {
        const res = try top_k_frequent_using_min_heap(std.testing.allocator, tc.nums, tc.k);
        defer std.testing.allocator.free(res);
        
        try std.testing.expectEqualSlices(i32, tc.expected, res);
    }
}
