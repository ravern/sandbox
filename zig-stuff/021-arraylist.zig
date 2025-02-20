const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const allocator = gpa.allocator();

    // Initialise the ArrayList, passing in the allocator it will use to dynamically
    // allocate memory for its items.
    var some_array_list = std.ArrayList(i32).init(allocator);
    defer some_array_list.deinit(); // REMEMBER TO DEINIT WHAT YOU INIT!!

    // Append some items to the list. Notice how we need to use `try` here, since the
    // memory allocation can fail.
    try some_array_list.append(3);
    try some_array_list.append(8);
    try some_array_list.append(4);
    try some_array_list.append(39);

    // Remove some items in the list. Notice how we don't allocate memory here, so we
    // don't need to use `try`. But we need to assign the result to something.
    _ = some_array_list.orderedRemove(1);

    // Iterate through the array list.
    for (some_array_list.items) |item| {
        std.debug.print("array list item: {}\n", .{item});
    }
}
