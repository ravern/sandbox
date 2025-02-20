const std = @import("std");

const Color = enum {
    red,
    green,
    blue,
    yellow,
    brown,
    // ...
};

// The integer representation of enums can be overrided.
const Operation = enum(u8) {
    add = 0,
    sub = 1,
    mul = 2,
    div = 3,
    rem = 4,
    shift_left = 5,
    shift_right = 6,

    // Enums can have methods too!
    fn name(self: Operation) []const u8 {
        switch (self) {
            .add => return "add",
            .sub => return "sub",
            .mul => return "mul",
            .div => return "div",
            .rem => return "rem",
            .shift_left => return "shift_left",
            .shift_right => return "shift_right",
        }
    }
};

pub fn main() !void {
    std.debug.print("red: {any}\n", .{Color.red});
    std.debug.print("blue: {any}\n", .{Color.blue});

    std.debug.print("mul: {any}\n", .{Operation.mul});
    std.debug.print("mul (tag value): {}\n", .{@intFromEnum(Operation.mul)});
    std.debug.print("shift_left: {any}\n", .{Operation.shift_left});
    std.debug.print("shift_left (tag value): {}\n", .{@intFromEnum(Operation.shift_left)});

    // You can also use enums in switch statements.
    const some_color = Color.red;
    switch (some_color) {
        .red => std.debug.print("some_color is red\n", .{}),
        .green => std.debug.print("some_color is green\n", .{}),
        .blue => std.debug.print("some_color is blue\n", .{}),
        .yellow => std.debug.print("some_color is yellow\n", .{}),
        .brown => std.debug.print("some_color is brown\n", .{}), // try removing this and compiling
    }
}
