const std = @import("std");

pub fn main() !void {
    // ints
    const my_32_bit_int: i32 = -42;
    const my_64_bit_int: i64 = -323;
    const my_32_bit_unsigned_int: u32 = 3424;
    const my_64_bit_unsigned_int: u64 = 34;

    // more ints ...?
    const my_17_bit_int: i17 = 17;
    const my_38_bit_unsigned_int: i38 = 38;

    // floats
    const my_32_bit_float: f32 = 3.14;
    const my_64_bit_float: f64 = 3.14159;

    // bool
    const my_bool: bool = true;

    // string
    const my_string: []const u8 = "Hello, world!";

    std.debug.print("32-bit int: {}\n", .{my_32_bit_int});
    std.debug.print("64-bit int: {}\n", .{my_64_bit_int});
    std.debug.print("32-bit unsigned int: {}\n", .{my_32_bit_unsigned_int});
    std.debug.print("64-bit unsigned int: {}\n", .{my_64_bit_unsigned_int});
    std.debug.print("17-bit int: {}\n", .{my_17_bit_int});
    std.debug.print("38-bit unsigned int: {}\n", .{my_38_bit_unsigned_int});
    std.debug.print("32-bit float: {}\n", .{my_32_bit_float});
    std.debug.print("64-bit float: {}\n", .{my_64_bit_float});
    std.debug.print("bool: {}\n", .{my_bool});
    std.debug.print("string: {s}\n", .{my_string});
}
