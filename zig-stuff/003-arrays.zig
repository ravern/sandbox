const std = @import("std");

pub fn main() !void {
    // Arrays have a fixed size.
    var my_int_array = [5]i32{ 1, 2, 3, 4, 5 };

    // You can use the `_` character to have the compiler infer the size.
    const my_other_int_array = [_]i32{ 1, 2, 3, 4, 5 };

    std.debug.print("my_int_array: {any}\n", .{my_int_array});
    std.debug.print("my_other_int_array: {any}\n", .{my_other_int_array});

    // `len` is the only field of an array.
    std.debug.print("length of my_int_array: {any}\n", .{my_int_array.len});

    // Access and modify items within an array using the `[]` syntax.
    std.debug.print("my_int_array[1]: {}\n", .{my_int_array[1]});

    // Arrays are copied by default.
    const my_copied_int_array = my_int_array;
    my_int_array[2] = 33;
    std.debug.print("my_copied_int_array: {any}\n", .{my_copied_int_array});

    // Arrays can have sentinel values.
    const my_int_sentinel_array = [5:0]i32{ 1, 2, 3, 4, 5 };

    std.debug.print("my_int_sentinel_array: {any}\n", .{my_int_sentinel_array});
    std.debug.print("my_int_sentinel_array.len: {any}\n", .{my_int_sentinel_array.len});
    std.debug.print("my_int_sentinel_array (with sentinel): {any}\n", .{@as([6]i32, @bitCast(my_int_sentinel_array))});
}
