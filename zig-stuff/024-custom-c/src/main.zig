const c = @cImport({
    @cInclude("greeter.c");
});

pub fn main() !void {
    c.greet("Alice");
    c.greet("Bob");
    c.greet(null);
}
