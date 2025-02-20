const std = @import("std");

pub fn main() !void {
    // Zig values can never be `null`, unless they are explicitly marked as optional.
    const some_int: i32 = 34;
    const some_optional_int: ?i32 = 34;
    const some_optional_int_null: ?i32 = null;

    std.debug.print("some_int: {}\n", .{some_int});
    std.debug.print("some_optional_int: {any}\n", .{some_optional_int});
    std.debug.print("some_optional_int_null: {any}\n", .{some_optional_int_null});

    // You can check if an optional is null using an if-else, and unwrap it at the same time.
    if (some_optional_int) |an_int| {
        std.debug.print("some_optional_int is not null: {}\n", .{an_int});
    } else {
        std.debug.print("some_optional_int is null\n", .{});
    }

    // You can also unwrap it with a default value.
    const an_int = some_optional_int orelse 0;
    std.debug.print("an_int: {}\n", .{an_int});

    // If you know an optional is definitely not null, you can unwrap it using `.?`.
    std.debug.print("some_optional_int is definitely not null: {}\n", .{some_optional_int.?});

    // But this will crash if you attempt to unwrap a null.
    //                ------------ try to uncomment the code below -------------
    // std.debug.print("some_optional_int_null is definitely not null: {}\n", .{some_optional_int_null.?});

    // Optionals aren't for free. They take up more space.
    std.debug.print("size of i32: {}\n", .{@sizeOf(i32)});
    std.debug.print("size of ?i32: {}\n", .{@sizeOf(?i32)});

    // However, they're free if the underlying value is a pointer! Any guesses why?
    std.debug.print("size of *i32: {}\n", .{@sizeOf(*i32)});
    std.debug.print("size of ?*i32: {}\n", .{@sizeOf(?*i32)});
}
