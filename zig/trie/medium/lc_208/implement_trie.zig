const std = @import("std");

pub const Trie = struct {
    children: std.AutoHashMap(u8, *Trie),
    is_leaf: bool,
    arena: std.mem.Allocator,

    pub fn init(arena: std.mem.Allocator) !*Trie {
        const self = try arena.create(Trie);
        self.* = .{
            .children = std.AutoHashMap(u8, *Trie).init(arena),
            .is_leaf = false,
            .arena = arena,
        };
        return self;
    }

    pub fn insert(self: *Trie, word: []const u8) !void {
        var current = self;
        for (word) |c| {
            const gop = try current.children.getOrPut(c);
            if (!gop.found_existing) {
                gop.value_ptr.* = try init(self.arena);
            }
            current = gop.value_ptr.*;
        }
        current.is_leaf = true;
    }

    pub fn search(self: *const Trie, word: []const u8) !bool {
        var current = self;
        for (word) |c| {
            if (!current.children.contains(c)) { return false; }
            current = current.children.get(c).?;
        }
        return current.is_leaf;
    }

    pub fn starts_with(self: *const Trie, word: []const u8) !bool {
        var current = self;
        for (word) |c| {
            if (!current.children.contains(c)) { return false; }
            current = current.children.get(c).?;
        }
        return true;
    }
};

test "lc_208_test" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();

    const trie = try Trie.init(arena.allocator());

    try trie.insert("apple");
    try std.testing.expect(try trie.search("apple"));
    try std.testing.expect(!(try trie.search("app")));
    try std.testing.expect(try trie.starts_with("app"));

    try trie.insert("app");
    try std.testing.expect(try trie.starts_with("app"));
}
