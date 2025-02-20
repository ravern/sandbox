const std = @import("std");

pub fn main() !void {
    var some_array = [_]i32{ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };

    // Slices are a view into an array.
    const some_slice: []i32 = some_array[2..6];
    std.debug.print("some_slice: {any}\n", .{some_slice});

    // Slices are represented by a pointer to the first element and a length.
    std.debug.print("some_slice.ptr: {*}\n", .{some_slice.ptr});
    std.debug.print("some_slice.len: {}\n", .{some_slice.len});
    std.debug.print("some_slice.ptr == &some_array[2]: {}\n", .{@intFromPtr(some_slice.ptr) == @intFromPtr(&some_array[2])});

    // Slices should be treated as pointers to arrays. Modifying the slice modifies the original array.
    some_slice[2] = 33;
    std.debug.print("some_slice: {any}\n", .{some_slice});
    std.debug.print("some_array: {any}\n", .{some_array});

    // Slices can be sliced further.
    const some_subslice = some_slice[1..3];
    std.debug.print("some_subslice: {any}\n", .{some_subslice});

    // Just like arrays, slices can have sentinel values.
    var some_sentinel_array = [10:0]i32{ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };

    const some_sentinel_slice: [:0]i32 = some_sentinel_array[2..10];
    std.debug.print("some_sentinel_slice: {any}\n", .{some_sentinel_slice});
    std.debug.print("some_sentinel_slice[8]: {}\n", .{some_sentinel_slice[8]});
}
