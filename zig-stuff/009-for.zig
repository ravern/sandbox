const std = @import("std");

pub fn main() !void {
    var some_array = [_]i32{ 1, 2, 3, 4, 5 };
    for (some_array) |item| {
        std.debug.print("array item: {}\n", .{item});
    }

    // Slices work too!
    const some_slice = some_array[1..4];
    for (some_slice) |item| {
        std.debug.print("slice item: {}\n", .{item});
    }

    // You can also iterate over pointers to each element rather than the value.
    for (some_slice) |*item| {
        std.debug.print("slice item pointer: {*}\n", .{item});
    }

    std.debug.print("{any}\n", .{some_slice});
}
