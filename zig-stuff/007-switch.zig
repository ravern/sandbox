const std = @import("std");

pub fn main() !void {
    const x = 34;
    switch (x) {
        1...5 => {
            std.debug.print("x is between 1 and 5!\n", .{});
        },
        6...10 => {
            std.debug.print("x is between 6 and 10!\n", .{});
        },
        else => {
            std.debug.print("x is not between 1 and 10...\n", .{});
        },
    }

    // Switch can also be used as an expression rather than a statement.
    const y = switch (x) {
        1...5 => 10,
        6...10 => 20,
        else => 30,
    };
    std.debug.print("y: {}\n", .{y});
}
