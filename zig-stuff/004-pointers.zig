const std = @import("std");

pub fn main() !void {
    // Pointers work similarly to C or C++.
    var some_int: i32 = 42;

    const some_int_pointer: *i32 = &some_int;

    std.debug.print("some_int_pointer: {*}\n", .{some_int_pointer});

    // Zig distinguishes between pointers to single values and pointers to multiple values (C-style arrays).
    var some_int_array = [3]i32{ 1, 2, 3 };

    const single_int_pointer: *i32 = &some_int_array[0];
    const many_int_pointer: [*]i32 = &some_int_array;

    std.debug.print("single_int_pointer: {*}\n", .{single_int_pointer});
    std.debug.print("many_int_pointer: {*}\n", .{many_int_pointer});
    std.debug.print("single_int_pointer == many_int_pointer: {}\n", .{@intFromPtr(single_int_pointer) == @intFromPtr(many_int_pointer)});

    // Just like arrays, pointers to multiple values can have sentinel values.
    var some_int_sentinel_array = [3:0]i32{ 1, 2, 3 };

    const many_int_sentinel_pointer: [*:0]i32 = &some_int_sentinel_array;

    std.debug.print("many_int_sentinel_pointer[3]: {}\n", .{many_int_sentinel_pointer[3]});
}
