const std = @import("std");

pub fn lessThan(_: void, a: usize, b:usize) std.math.Order{
    if (a < b) { return .lt; }         
    if (a > b) { return .gt; }
    return .eq;
}

pub const DinnerPlates = struct{
    capacity: usize, 
    arr: std.ArrayList(?i32),
    is_empty: std.PriorityQueue(usize, void, lessThan),

    pub fn init(arena: std.mem.Allocator, capacity: usize) DinnerPlates {
        return DinnerPlates{
            .capacity = capacity,
            .arr = std.ArrayList(?i32).init(arena),
            .is_empty = std.PriorityQueue(usize, void, lessThan).init(arena, {})
        };
    }

    pub fn push(self: *DinnerPlates, val: i32) void{
        while (self.is_empty.count() > 0){
            const i = self.is_empty.remove();
            if (i < self.arr.items.len) {
                self.arr.items[i] = val;
                return;
            }
        }
        self.arr.append(val) catch unreachable;
    }

    pub fn pop(self: *DinnerPlates) i32 {
        while(self.arr.items.len > 0) {
            if (self.arr.pop()) |val| {
                if (val) |v| { return v; }
            }            
        }
        return -1;
    }

    pub fn pop_at_stack(self: *DinnerPlates, index: usize) i32 {
        if (index > self.arr.items.len / self.capacity ) { return -1; }
        const start = self.capacity * index;
        const end = @min(self.arr.items.len, start + self.capacity);
        var iter: usize = 1;
        for (start..end) |_| {
           if (self.arr.items[end - iter]) |val| {
               self.arr.items[end - iter] = null;
               self.is_empty.add(end - iter) catch unreachable;
               return val;
           }  
           iter += 1;
        }
        return -1;
    }
};

test "lc_1127_tests" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();

    var d = DinnerPlates.init(arena.allocator(), 2);
    d.push(1);
    d.push(2);
    d.push(3);
    d.push(4);
    d.push(5);

    try std.testing.expectEqual(2, d.pop_at_stack(0));

    d.push(20);
    d.push(21);

    try std.testing.expectEqual(20, d.pop_at_stack(0));
    try std.testing.expectEqual(21, d.pop_at_stack(2));

    try std.testing.expectEqual(5, d.pop());
    try std.testing.expectEqual(4, d.pop());
    try std.testing.expectEqual(3, d.pop());
    try std.testing.expectEqual(1, d.pop());
    try std.testing.expectEqual(-1, d.pop());

}
