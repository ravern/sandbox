const std = @import("std");

pub fn main() !void {
    var x: i32 = 0;
    while (x < 32) {
        std.debug.print("x: {}\n", .{x});
        x += 1;
    }

    // You can also pass an expression to perform each iteration.
    var y: i32 = 0;
    while (y < 32) : (y += 1) {
        std.debug.print("y: {}\n", .{y});
    }
}
