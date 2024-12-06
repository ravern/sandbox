const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    var arena = std.heap.ArenaAllocator.init(gpa.allocator());
    defer arena.deinit();

    const allocator = arena.allocator();

    const file = try std.fs.cwd().openFile("input", .{});
    defer file.close();

    var is_parsing_updates = false;
    var rules = std.AutoHashMap(usize, std.ArrayList(usize)).init(allocator);
    var updates = std.ArrayList(std.ArrayList(usize)).init(allocator);

    const reader = file.reader();
    while (try reader.readUntilDelimiterOrEofAlloc(allocator, '\n', 1024)) |line| {
        if (line.len == 0) {
            is_parsing_updates = true;
        } else if (is_parsing_updates) {
            var pages = std.ArrayList(usize).init(allocator);
            var it = std.mem.split(u8, line, ",");
            while (it.next()) |page_str| {
                const page = try std.fmt.parseInt(usize, page_str, 10);
                try pages.append(page);
            }
            try updates.append(pages);
        } else {
            var it = std.mem.split(u8, line, "|");
            const before_page = try std.fmt.parseInt(usize, it.next() orelse return error.InvalidInput, 10);
            const after_page = try std.fmt.parseInt(usize, it.next() orelse return error.InvalidInput, 10);

            const rule_entry = try rules.getOrPut(before_page);
            if (rule_entry.found_existing) {
                try rule_entry.value_ptr.append(after_page);
            } else {
                rule_entry.value_ptr.* = std.ArrayList(usize).init(allocator);
                try rule_entry.value_ptr.append(after_page);
            }
        }
    }

    var part_one: usize = 0;

    for (updates.items) |update| {
        var before_pages = std.AutoHashMap(usize, void).init(allocator);
        var after_pages = std.AutoHashMap(usize, void).init(allocator);

        for (update.items) |page| {
            try after_pages.put(page, {});
        }

        var is_valid = true;

        var index: usize = 0;
        while (is_valid and index < update.items.len) : (index += 1) {
            const curr_page = update.items[index];

            _ = after_pages.remove(curr_page);

            if (rules.get(curr_page)) |curr_after_pages| {
                for (curr_after_pages.items) |curr_after_page| {
                    if (before_pages.get(curr_after_page)) |_| {
                        is_valid = false;
                        break;
                    }
                }
            }

            try before_pages.put(curr_page, {});
        }

        if (is_valid) {
            part_one += update.items[update.items.len / 2];
        }
    }

    std.debug.print("part one: {}\n", .{part_one});

    var part_two: usize = 0;

    for (updates.items) |update| {
        var is_valid = false;

        while (!is_valid) {
            var before_pages = std.AutoHashMap(usize, usize).init(allocator);
            var after_pages = std.AutoHashMap(usize, usize).init(allocator);

            for (update.items, 0..) |page, index| {
                try after_pages.put(page, index);
            }

            is_valid = true;

            var index: usize = 0;
            while (is_valid and index < update.items.len) : (index += 1) {
                const curr_page = update.items[index];
                const curr_page_index = after_pages.fetchRemove(curr_page).?.value;

                if (rules.get(curr_page)) |curr_after_pages| {
                    for (curr_after_pages.items) |curr_after_page| {
                        if (before_pages.get(curr_after_page)) |before_page_index| {
                            is_valid = false;
                            const tmp = update.items[before_page_index];
                            update.items[before_page_index] = curr_page;
                            update.items[curr_page_index] = tmp;
                            break;
                        }
                    }
                }

                try before_pages.put(curr_page, curr_page_index);
            }
        }

        part_two += update.items[update.items.len / 2];
    }

    part_two -= part_one;

    std.debug.print("part two: {}\n", .{part_two});
}
