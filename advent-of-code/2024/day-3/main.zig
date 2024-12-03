const std = @import("std");

const PartOne = struct {
    result: usize = 0,
    state: State = .m,
    left: usize = 0,
    right: usize = 0,

    const State = enum {
        m,
        u,
        l,
        open_paren,
        left_number_and_comma,
        right_number_and_close_paren,
    };

    fn process(self: *PartOne, c: u8) !void {
        switch (self.state) {
            .m => self.state = if (c == 'm') .u else .m,
            .u => self.state = if (c == 'u') .l else .m,
            .l => self.state = if (c == 'l') .open_paren else .m,
            .open_paren => self.state = if (c == '(') .left_number_and_comma else .m,
            .left_number_and_comma => {
                if (c == ',' and self.left != 0) {
                    self.state = .right_number_and_close_paren;
                } else if (c >= '0' and c <= '9') {
                    self.left = self.left * 10 + (c - '0');
                } else {
                    self.reset();
                }
            },
            .right_number_and_close_paren => {
                if (c == ')' and self.right != 0) {
                    self.result += self.left * self.right;
                    self.reset();
                } else if (c >= '0' and c <= '9') {
                    self.right = self.right * 10 + (c - '0');
                } else {
                    self.reset();
                }
            },
        }
    }

    fn reset(self: *PartOne) void {
        self.left = 0;
        self.right = 0;
        self.state = .m;
    }
};

const PartTwo = struct {
    result: usize = 0,
    state: State = .mul_m_or_dont_d,
    left: usize = 0,
    right: usize = 0,

    const State = enum {
        mul_m_or_dont_d,

        mul_u,
        mul_l,
        mul_open_paren,
        mul_left_number_and_comma,
        mul_right_number_and_close_paren,

        dont_o,
        dont_n,
        dont_apostrophe,
        dont_t,
        dont_open_paren,
        dont_close_paren,

        do_d,
        do_o,
        do_open_paren,
        do_close_paren,
    };

    fn process(self: *PartTwo, c: u8) !void {
        switch (self.state) {
            .mul_m_or_dont_d => self.state = if (c == 'm') .mul_u else if (c == 'd') .dont_o else .mul_m_or_dont_d,
            .mul_u => self.state = if (c == 'u') .mul_l else .mul_m_or_dont_d,
            .mul_l => self.state = if (c == 'l') .mul_open_paren else .mul_m_or_dont_d,
            .mul_open_paren => self.state = if (c == '(') .mul_left_number_and_comma else .mul_m_or_dont_d,
            .mul_left_number_and_comma => {
                if (c == ',' and self.left != 0) {
                    self.state = .mul_right_number_and_close_paren;
                } else if (c >= '0' and c <= '9') {
                    self.left = self.left * 10 + (c - '0');
                } else {
                    self.reset();
                }
            },
            .mul_right_number_and_close_paren => {
                if (c == ')' and self.right != 0) {
                    self.result += self.left * self.right;
                    self.reset();
                } else if (c >= '0' and c <= '9') {
                    self.right = self.right * 10 + (c - '0');
                } else {
                    self.reset();
                }
            },
            .dont_o => self.state = if (c == 'o') .dont_n else .mul_m_or_dont_d,
            .dont_n => self.state = if (c == 'n') .dont_apostrophe else .mul_m_or_dont_d,
            .dont_apostrophe => self.state = if (c == '\'') .dont_t else .mul_m_or_dont_d,
            .dont_t => self.state = if (c == 't') .dont_open_paren else .mul_m_or_dont_d,
            .dont_open_paren => self.state = if (c == '(') .dont_close_paren else .mul_m_or_dont_d,
            .dont_close_paren => self.state = if (c == ')') .do_d else .mul_m_or_dont_d,
            .do_d => self.state = if (c == 'd') .do_o else .do_d,
            .do_o => self.state = if (c == 'o') .do_open_paren else .do_d,
            .do_open_paren => self.state = if (c == '(') .do_close_paren else .do_d,
            .do_close_paren => self.state = if (c == ')') .mul_m_or_dont_d else .do_d,
        }
    }

    fn reset(self: *PartTwo) void {
        self.left = 0;
        self.right = 0;
        self.state = .mul_m_or_dont_d;
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    var arena = std.heap.ArenaAllocator.init(gpa.allocator());
    defer arena.deinit();

    const allocator = arena.allocator();
    _ = allocator;

    const file = try std.fs.cwd().openFile("input", .{});
    defer file.close();

    var part_one = PartOne{};
    var part_two = PartTwo{};

    const reader = file.reader();
    while (reader.readByte()) |c| {
        try part_one.process(c);
        try part_two.process(c);
    } else |_| {}

    std.debug.print("part one: {}\n", .{part_one.result});
    std.debug.print("part two: {}\n", .{part_two.result});
}
