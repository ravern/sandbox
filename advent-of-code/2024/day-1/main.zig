const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    var arena = std.heap.ArenaAllocator.init(gpa.allocator());
    defer arena.deinit();

    const allocator = arena.allocator();

    var left_list = std.ArrayList(usize).init(allocator);
    defer left_list.deinit();
    var right_list = std.ArrayList(usize).init(allocator);
    defer right_list.deinit();

    var left_map = std.AutoHashMap(usize, usize).init(allocator);
    defer left_map.deinit();
    var right_map = std.AutoHashMap(usize, usize).init(allocator);
    defer right_map.deinit();

    const file = try std.fs.cwd().openFile("input", .{});
    defer file.close();

    const reader = file.reader();
    while (try reader.readUntilDelimiterOrEofAlloc(allocator, '\n', 1024)) |line| {
        var it = std.mem.split(u8, line, "   ");

        const left = try std.fmt.parseInt(usize, it.next().?, 10);
        try left_list.append(left);
        const left_result = try left_map.getOrPut(left);
        if (left_result.found_existing) {
            left_result.value_ptr.* += 1;
        } else {
            left_result.value_ptr.* = 1;
        }

        const right = try std.fmt.parseInt(usize, it.next().?, 10);
        try right_list.append(right);
        const right_result = try right_map.getOrPut(right);
        if (right_result.found_existing) {
            right_result.value_ptr.* += 1;
        } else {
            right_result.value_ptr.* = 1;
        }
    }

    std.debug.assert(left_list.items.len == right_list.items.len);

    std.mem.sort(usize, left_list.items, {}, std.sort.asc(usize));
    std.mem.sort(usize, right_list.items, {}, std.sort.asc(usize));

    var part_one: usize = 0;
    for (left_list.items, right_list.items) |left, right| {
        part_one += @intCast(@abs(@as(isize, @intCast(left)) - @as(isize, @intCast(right))));
    }

    std.debug.print("part one: {d}\n", .{part_one});

    var done_map = std.AutoHashMap(usize, void).init(allocator);
    defer done_map.deinit();

    var part_two: usize = 0;

    var left_it = left_map.iterator();
    while (left_it.next()) |left_entry| {
        if (right_map.getEntry(left_entry.key_ptr.*)) |right_entry| {
            part_two += left_entry.key_ptr.* * left_entry.value_ptr.* * right_entry.value_ptr.*;
        }
        try done_map.put(left_entry.key_ptr.*, {});
    }

    var right_it = right_map.iterator();
    while (right_it.next()) |right_entry| {
        if (done_map.contains(right_entry.key_ptr.*)) {
            continue;
        }
        if (left_map.getEntry(right_entry.key_ptr.*)) |left_entry| {
            part_two += right_entry.key_ptr.* * right_entry.value_ptr.* * left_entry.value_ptr.*;
        }
    }

    std.debug.print("part two: {d}\n", .{part_two});
}
