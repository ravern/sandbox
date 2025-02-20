const std = @import("std");

pub fn main() !void {
    // We first create the general-purpose allocator.
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    // Then we get the general `std.mem.Allocator` struct from it.
    // This is what we'll call to (de)allocate memory.
    const allocator = gpa.allocator();

    // Let's allocate 16 bytes of memory.
    const some_bytes: []u8 = try allocator.alloc(u8, 16);

    // Maybe put a string into it.
    std.mem.copyForwards(u8, some_bytes, "Hello, my world!");

    // What's in the memory?
    std.debug.print("{s}\n", .{some_bytes});
}
