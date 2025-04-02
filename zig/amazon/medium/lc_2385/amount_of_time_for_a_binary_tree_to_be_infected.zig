const std = @import("std");

pub const TreeNode = struct {
    val: i32, 
    left: ?*TreeNode,
    right: ?*TreeNode,

    pub fn init(val: i32) TreeNode {
        return TreeNode {
            .val = val, 
            .left = null, 
            .right = null,
        };
    }
};

pub fn amount_of_time(arena: std.mem.Allocator, root: ?*TreeNode, start: i32) !i32 {
    const convert = struct {
        fn c(alloc: std.mem.Allocator, current: ?*TreeNode, parent: i32, tree_adj_list: std.AutoHashMap(i32, std.AutoHashMap(i32, void))) !void {
            var mut_add = tree_adj_list;
            if(current == null) { return; }
            
            const adj_list_gop = try mut_add.getOrPut(current.?.*.val);
            adj_list_gop.value_ptr.* = std.AutoHashMap(i32, void).init(alloc);

            if (parent != 0) { try adj_list_gop.value_ptr.*.put(parent, {}); }
            if (current.?.left)  |v| { try adj_list_gop.value_ptr.*.put(v.val, {}); }
            if (current.?.right) |v| { try adj_list_gop.value_ptr.*.put(v.val, {}); }

            try c(alloc, current.?.left, current.?.*.val, tree_adj_list);
            try c(alloc, current.?.right, current.?.*.val, tree_adj_list);
        }
    };

    const adj = std.AutoHashMap(i32, std.AutoHashMap(i32, void)).init(arena);
    try convert.c(arena, root, 0, adj);
    return start;
}

test "lc_2385_tests" {
    const testcases = [_]struct{
        input: []const ?i32,
        start: i32,
        expected: i32
    }{
        .{
            .input = &.{ 1, 5, 3, null, 4, 10, 6, 9, 2 },
            .start = 3, 
            .expected = 4
        },
        .{
            .input = &.{ 1 },
            .start = 1, 
            .expected = 1
        },
    };

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    
    for (testcases)|tc| {
        const root = try build_tree(arena.allocator(), tc.input);
        try std.testing.expectEqual(tc.expected, try amount_of_time(arena.allocator(), root, tc.start));
        arena.deinit();

    }
}

fn build_tree(arena: std.mem.Allocator, input: []const ?i32) !?*TreeNode {
    if (input.len == 0) { return null; }
    if (input[0] == null) { return null; }

    const root = try arena.create(TreeNode);
    root.* = TreeNode.init(input[0].?);

    const qt = std.DoublyLinkedList(*TreeNode);
    var queue = qt{};
    
    var node = qt.Node{.data = root};
    queue.append(&node);

    var i: usize = 1;

    while (i < input.len and queue.len > 0) {
        const current = queue.popFirst();
        //process the left child 
        if (i < input.len) {
            if (input[i]) |val| {
                const left_node = try arena.create(TreeNode);
                left_node.* = TreeNode.init(val);
                current.?.data.left.?.* = left_node.*;
                
                var insert = qt.Node{.data = left_node};
                queue.append(&insert);
            }
            i += 1;
        }
        //process the right child
        if (i < input.len) {
            if (input[i]) |val| {
                const right_node = try arena.create(TreeNode);
                right_node.* = TreeNode.init(val);
                current.?.data.right.?.* = right_node.*;

                var insert = qt.Node{.data = right_node};
                queue.append(&insert);
            }
            i += 1;
        }
    }
    return root;
}
