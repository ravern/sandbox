const std = @import("std");

const directions = [8][2]isize{
    .{ 0, 1 },
    .{ 1, 0 },
    .{ 0, -1 },
    .{ -1, 0 },
    .{ 1, 1 },
    .{ -1, -1 },
    .{ 1, -1 },
    .{ -1, 1 },
};

fn findWord(word: []const u8, word_search: [][]const u8, width: isize, height: isize, col: isize, row: isize) usize {
    var found: usize = 0;

    var direction_index: usize = 0;
    direction_loop: while (direction_index < directions.len) : (direction_index += 1) {
        const delta = directions[direction_index];

        var index: usize = 0;
        while (index < word.len) : (index += 1) {
            const curr_col = col + delta[0] * @as(isize, @intCast(index));
            const curr_row = row + delta[1] * @as(isize, @intCast(index));
            if (curr_col < 0 or curr_col >= width or curr_row < 0 or curr_row >= height or
                word_search[@intCast(curr_row)][@intCast(curr_col)] != word[index])
            {
                continue :direction_loop;
            }
        }

        found += 1;
    }

    return found;
}

fn findXmasCross(word_search: [][]const u8, width: isize, height: isize, col: isize, row: isize) bool {
    if (col < 1 or col >= width - 1 or row < 1 or row >= height - 1 or word_search[@intCast(row)][@intCast(col)] != 'A') {
        return false;
    }

    const cross = [4]u8{
        word_search[@intCast(row - 1)][@intCast(col - 1)],
        word_search[@intCast(row + 1)][@intCast(col + 1)],
        word_search[@intCast(row + 1)][@intCast(col - 1)],
        word_search[@intCast(row - 1)][@intCast(col + 1)],
    };

    if (std.mem.eql(u8, &cross, "MSMS") or
        std.mem.eql(u8, &cross, "SMSM") or
        std.mem.eql(u8, &cross, "MSSM") or
        std.mem.eql(u8, &cross, "SMMS"))
    {
        return true;
    }

    return false;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    var arena = std.heap.ArenaAllocator.init(gpa.allocator());
    defer arena.deinit();

    const allocator = arena.allocator();

    const file = try std.fs.cwd().openFile("input", .{});
    defer file.close();

    var lines = std.ArrayList([]const u8).init(allocator);

    const reader = file.reader();
    while (try reader.readUntilDelimiterOrEofAlloc(allocator, '\n', 1024)) |line| {
        try lines.append(line);
    }

    const word = "XMAS";

    const word_search = try lines.toOwnedSlice();
    const width: isize = @intCast(word_search[0].len);
    const height: isize = @intCast(word_search.len);

    var part_one: usize = 0;
    var part_two: usize = 0;

    var col: isize = 0;
    while (col < width) : (col += 1) {
        var row: isize = 0;
        while (row < height) : (row += 1) {
            if (word_search[@intCast(row)][@intCast(col)] == word[0]) {
                part_one += findWord(word, word_search, width, height, col, row);
            }
            if (word_search[@intCast(row)][@intCast(col)] == 'A' and findXmasCross(word_search, width, height, col, row)) {
                part_two += 1;
            }
        }
    }

    std.debug.print("part one: {}\n", .{part_one});
    std.debug.print("part two: {}\n", .{part_two});
}
