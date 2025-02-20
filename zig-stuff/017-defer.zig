const std = @import("std");

pub fn main() !void {
    std.debug.print("normal 1\n", .{});
    defer std.debug.print("defer 1\n", .{});
    std.debug.print("normal 2\n", .{});
    defer std.debug.print("defer 2\n", .{});
    std.debug.print("normal 3\n", .{});
    defer std.debug.print("defer 3\n", .{});
}
