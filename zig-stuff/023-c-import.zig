const c = @cImport({
    @cInclude("stdio.h");
});

pub fn main() !void {
    _ = c.printf("Hello, world!\n");
}
