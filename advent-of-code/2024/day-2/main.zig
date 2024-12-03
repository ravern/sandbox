const std = @import("std");

const Report = struct {
    levels: []const isize,

    fn isSafe(self: *const Report) bool {
        var is_desc: ?bool = null;
        var index: usize = 0;
        while (index < self.levels.len - 1) : (index += 1) {
            var delta = self.levels[index + 1] - self.levels[index];
            is_desc = is_desc orelse (delta < 0);
            if (is_desc.?) delta *= -1;
            if (delta < 1 or delta > 3) return false;
        }
        return true;
    }

    fn isSafeWithTolerance(self: *const Report) bool {
        var skip: usize = 0;
        while (skip < self.levels.len) : (skip += 1) {
            if (self.isSafeSkip(skip)) return true;
        }
        return false;
    }

    fn isSafeSkip(self: *const Report, skip: usize) bool {
        var is_desc: ?bool = null;
        var index: usize = 0;
        while (index < self.levels.len - 1) : (index += 1) {
            const curr_index = if (index < skip) index else index + 1;
            const next_index = if (index + 1 < skip) index + 1 else index + 2;
            if (next_index == self.levels.len) break;
            var delta = self.levels[next_index] - self.levels[curr_index];
            is_desc = is_desc orelse (delta < 0);
            if (is_desc.?) delta *= -1;
            if (delta < 1 or delta > 3) return false;
        }
        return true;
    }
};

fn isReportSafe(line: []const u8, tolerance: usize) bool {
    var it = std.mem.split(u8, line, " ");

    var prev = std.fmt.parseInt(isize, it.next().?, 10) catch unreachable;
    var curr = std.fmt.parseInt(isize, it.next().?, 10) catch unreachable;

    if (prev == curr) return false;

    var is_desc: ?bool = null;
    var curr_tolerance = tolerance;

    while (true) {
        var delta = curr - prev;

        const is_curr_desc = curr - prev < 0;
        if (is_curr_desc) delta *= -1;

        if (delta < 1 or delta > 3) {
            if (curr_tolerance == 0) return false;
            curr_tolerance -= 1;
            curr = std.fmt.parseInt(isize, it.next() orelse return true, 10) catch unreachable;
            continue;
        }

        if (is_desc) |is_prev_desc| {
            if (is_prev_desc != is_curr_desc) return false;
        } else {
            is_desc = is_curr_desc;
        }

        prev = curr;
        curr = std.fmt.parseInt(isize, it.next() orelse return true, 10) catch unreachable;
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    var arena = std.heap.ArenaAllocator.init(gpa.allocator());
    defer arena.deinit();

    const allocator = arena.allocator();

    const file = try std.fs.cwd().openFile("input", .{});
    defer file.close();

    var reports = std.ArrayList(Report).init(allocator);
    defer reports.deinit();

    const reader = file.reader();
    while (try reader.readUntilDelimiterOrEofAlloc(allocator, '\n', 1024)) |line| {
        var levels = std.ArrayList(isize).init(allocator);

        var it = std.mem.split(u8, line, " ");
        while (it.next()) |level_str| {
            try levels.append(std.fmt.parseInt(isize, level_str, 10) catch unreachable);
        }

        const report = Report{ .levels = try levels.toOwnedSlice() };
        try reports.append(report);
    }

    var part_one: usize = 0;
    for (reports.items) |report| {
        if (report.isSafe()) part_one += 1;
    }
    std.debug.print("part one: {d}\n", .{part_one});

    var part_two: usize = 0;
    for (reports.items) |report| {
        if (report.isSafeWithTolerance()) part_two += 1;
    }
    std.debug.print("part two: {d}\n", .{part_two});
}
