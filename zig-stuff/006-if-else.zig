const std = @import("std");

pub fn main() !void {
    const x = 5;
    if (x > 7) {
        std.debug.print("x is greater than 7!\n", .{});
    } else {
        std.debug.print("x is smaller than or equal to 7...\n", .{});
    }

    // If-else can also be used as expressions rather than statements.
    const y = if (x > 4) 10 else 20;
    std.debug.print("y: {}\n", .{y});
}
