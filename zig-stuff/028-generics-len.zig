const std = @import("std");

fn Vector(comptime T: type, comptime len: usize) type {
    return struct {
        values: [len]T,
    };
}

const Vector_i32_2D = Vector(i32, 2);
const Vector_f32_4D = Vector(f32, 4);
pub fn main() void {
    const vector_of_2_i32s = Vector_i32_2D{ .values = [_]i32{ 1, 2 } };
    const vector_of_4_f32s = Vector_f32_4D{ .values = [_]f32{ 1.0, 2.0, 3.0, 4.0 } };

    std.debug.print("vector_of_2_i32s: {any}\n", .{vector_of_2_i32s});
    std.debug.print("vector_of_4_f32s: {any}\n", .{vector_of_4_f32s});
}
