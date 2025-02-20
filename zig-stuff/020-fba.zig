const std = @import("std");

pub fn main() !void {
    var buf: [16]u8 = undefined;

    // We first create the fixed buffer allocator.
    var fba = std.heap.FixedBufferAllocator.init(&buf);

    // Then we get the general `std.mem.Allocator` struct from it.
    // This is what we'll call to (de)allocate memory.
    const allocator = fba.allocator();

    // Let's allocate 16 bytes of memory.
    const some_bytes: []u8 = try allocator.alloc(u8, 16);
    defer allocator.free(some_bytes);

    // Maybe put a string into it.
    std.mem.copyForwards(u8, some_bytes, "Hello, my world!");

    // What's in the memory?
    std.debug.print("{s}\n", .{some_bytes});
}
