const std = @import("std");

fn sum(comptime T: type, values: []const T) T {
    var result: T = 0;
    for (values) |value| {
        result += value;
    }
    return result;
}

pub fn main() void {
    const some_i32s = [_]i32{ 1, 2, 3, 4, 5 };
    std.debug.print("sum of i32s: {}\n", .{sum(i32, &some_i32s)});

    const some_f32s = [_]f32{ 1.0, 2.0, 3.0, 4.0, 5.0 };
    std.debug.print("sum of f32s: {}\n", .{sum(f32, &some_f32s)});

    const some_u64s = [_]u64{ 1, 2, 3, 4, 5 };
    std.debug.print("sum of u64s: {}\n", .{sum(u64, &some_u64s)});
}
