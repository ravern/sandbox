const std = @import("std");

fn Vector2D(comptime T: type) type {
    return struct {
        x: T,
        y: T,
    };
}

pub fn main() void {
    const vector_of_i32s = Vector2D(i32){ .x = 1, .y = 2 };
    const vector_of_f32s = Vector2D(f32){ .x = 1.0, .y = 2.0 };

    std.debug.print("vector_of_i32s: {any}\n", .{vector_of_i32s});
    std.debug.print("vector_of_f32s: {any}\n", .{vector_of_f32s});
}
