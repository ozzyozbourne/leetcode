const std = @import("std");
const tup = struct{i32, i32};
const q = std.DoublyLinkedList(tup);

pub fn getFood(arena: std.mem.Allocator, grid: []const []const u8) !i32 {
    const dir: [4]tup, var steps: i32 = .{ .{ .{-1, 0}, .{1, 0}, .{0, -1}, .{0, 1} }, 1};    
    var queue = q{};

    var grd_cpy = try arena.alloc([]u8, grid.len);
    for (0..grid.len) |i| { grd_cpy[i] = try arena.dupe(u8, grid[i]); }

    outer: for (grid, 0..) |row, i| {
        for (row, 0..) |col, j| {
            if (col == '*') {
                const node = try arena.create(q.Node);
                node.* = q.Node{ .data = .{@intCast(i), @intCast(j)} };
                queue.append(node);
                break :outer;
            }
        }
    }

    while (queue.len != 0){
        for (0..queue.len) |_| {
            const r: i32, const c: i32 = queue.popFirst().?.*.data;
            for (dir) |t| {
               const cx: i32, const rx: i32 = .{ c + t[0], r + t[1] };
               if (rx >= 0 and @as(usize, @intCast(rx)) < grd_cpy.len 
                   and cx >= 0 and @as(usize, @intCast(cx)) < grd_cpy[0].len 
                   and grd_cpy[@intCast(rx)][@intCast(cx)] != 'X') {
                    if (grd_cpy[@intCast(rx)][@intCast(cx)] == '#') {
                        return steps;
                    }
                    grd_cpy[@intCast(rx)][@intCast(cx)] = 'X';
                    const node = try arena.create(q.Node);
                    node.* = q.Node{ .data = .{rx, cx} };
                    queue.append(node);

               }
            }
        }
        steps += 1;
    }
    return -1;
} 

test "lc_1730_tests"{
    const testcases = [_]struct{
        grid: []const []const u8,
        expected: i32
    }{
        .{
            .grid = &.{ 
                &.{'X','X','X','X','X','X'},
                &.{'X','*','O','O','O','X'},
                &.{'X','O','O','#','O','X'},
                &.{'X','X','X','X','X','X'}
            },
            .expected = 3
        }, 
        .{
            .grid = &.{
                &.{'X','X','X','X','X'},
                &.{'X','*','X','O','X'},
                &.{'X','O','X','#','X'},
                &.{'X','X','X','X','X'}
            },
            .expected = -1
        },
        .{
            .grid = &.{
                &.{'X','X','X','X','X','X','X','X'},
                &.{'X','*','O','X','O','#','O','X'},
                &.{'X','O','O','X','O','O','X','X'},
                &.{'X','O','O','O','O','#','O','X'},
                &.{'X','X','X','X','X','X','X','X'}
            },
            .expected = 6
        },
        .{
            .grid = &.{
                &.{'X','X','X','X','X'},
                &.{'X','*','X','O','X'},
                &.{'X','O','X','#','X'},
                &.{'O','O','O','O','O'}
            },
            .expected = 5
        }
    };

    for (testcases) |tc| { 
        var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
        defer arena.deinit();

        try std.testing.expectEqual(tc.expected, try getFood(arena.allocator(), tc.grid)); 
    }
}
