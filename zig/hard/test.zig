const std = @import("std");

pub fn main() void {
    const tuple =std.meta.Tuple(&.{struct{s1: []const u8, s2: []const u8}, bool});
    const s = tuple{ .{.s1 = "one", .s2 = "tow"}, true }; 
    std.debug.print("{s} = {s}, {s} \n", .{ @typeName(tuple), s.@"0".s1, s.@"0".s2});

    const Tuple = struct{a1: []const u8, a2: []const u8};

    const map = std.hash_map.HashMap(Tuple, bool, std.hash_map.AutoContext(Tuple), 80);

    std.debug.print("{s}\n", .{ @typeName(map) });
}
