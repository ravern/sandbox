const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const allocator = gpa.allocator();

    // Initialise the ArrayListUnmanaged. Notice we don't need to pass any allocator here,
    // instead we just need to pass it in `deinit`.
    var some_array_list = std.ArrayListUnmanaged(i32){};
    defer some_array_list.deinit(allocator); // REMEMBER TO DEINIT WHAT YOU INIT!!

    // Append some items to the list. Notice how we need to pass the allocator here, and also
    // use `try` here, since the memory allocation can fail.
    try some_array_list.append(allocator, 3);
    try some_array_list.append(allocator, 8);
    try some_array_list.append(allocator, 4);
    try some_array_list.append(allocator, 39);

    // Remove some items in the list. Notice how we don't allocate memory here, so we
    // don't need to use `try` or pass any allocator. But we need to assign the result
    // to something.
    _ = some_array_list.orderedRemove(1);

    // Iterate through the array list.
    for (some_array_list.items) |item| {
        std.debug.print("array list item: {}\n", .{item});
    }
}
