const std = @import("std");

pub fn foo(x: i32, y: f32) f32 {
    std.debug.print("inside the foo function... x: {}, y: {}\n", .{ x, y });
    return @as(f32, @floatFromInt(x + 2)) * y;
}

pub fn main() !void {
    std.debug.print("foo(3, 5.34) returned {}\n", .{foo(3, 5.34)});
}
