const std = @import("std");
const linked_list = @import("./linked_list.zig");
const LinkedList = linked_list.LinkedList;

pub fn main() !void {
    var list = LinkedList(usize).init(std.heap.general_purpose_allocator);
    try list.append(2);
    try list.append(3);
    try list.prepend(1);

    std.debug.print("{}\n", .{ .list = list });
}
