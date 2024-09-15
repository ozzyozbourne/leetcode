const std = @import("std");
const gpa = std.heap.GeneralPurposeAllocator(.{}){};
var allocator = gpa.allocator();
pub fn main() !void {
    const ptr1 = try allocator.alloc(i32, 10); // Allocate memory for 10 integers
    allocator.free(ptr1); // Free this memory to avoid leaks

    const ptr2 = try allocator.alloc(u8, 1024); // Allocate memory for 1024 bytes
    _ = ptr2;

    if (gpa.detectLeaks()) {
        std.debug.print("\n ----- \nMemory leak detected\n ---- \n", .{});
    } else {
        std.debug.print("No memory leaks detected\n", .{});
    }
}
