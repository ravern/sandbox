const std = @import("std");

const MyError = error{
    FooReason,
    BarReason,
    AnotherReason,
};

pub fn myFunction(x: i32) MyError!i32 {
    if (x < 43) {
        // Simply return the error if you encounter an error condition, instead
        // of returning the result.
        return MyError.FooReason;
    } else if (x > 43) {
        return MyError.BarReason;
    } else {
        return 33;
    }
}

// Here the error type is inferred, instead of being explicitly defined.
pub fn myOtherFunction(x: i32) !i32 {
    if (x < 99) {
        return MyError.AnotherReason;
    } else if (x > 99) {
        return MyError.BarReason;
    } else {
        return 22;
    }
}

pub fn main() !void {
    // This will propagate the error up to the caller of this function.
    // In the case of the `main` function, it would end the program.
    const my_function_result = try myFunction(43);
    std.debug.print("The result of myFunction is: {}\n", .{my_function_result});

    // Instead of propagating the error, you can also handle it.
    const my_other_function_result = myOtherFunction(43) catch |err| {
        switch (err) {
            MyError.FooReason => std.debug.print("FooReason\n", .{}),
            MyError.BarReason => std.debug.print("BarReason\n", .{}),
            MyError.AnotherReason => std.debug.print("AnotherReason\n", .{}),
        }
        // Return early from the main function.
        return;
    };
    std.debug.print("The result of myOtherFunction is: {}\n", .{my_other_function_result});
}
