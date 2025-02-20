const std = @import("std");

const Point = struct {
    x: i32,
    y: i32,
};

const Rect = struct {
    top_left: Point,
    bottom_right: Point,

    // You can define methods in structs.
    fn area(self: Rect) i32 {
        return (self.bottom_right.x - self.top_left.x) * (self.bottom_right.y - self.top_left.y);
    }
};

pub fn main() !void {
    var point1 = Point{ .x = 32, .y = 32 };
    const point2 = Point{ .x = 99, .y = 44 };
    const rect = Rect{ .top_left = point1, .bottom_right = point2 };
    std.debug.print("point1: {any}\n", .{point1});
    std.debug.print("point2: {any}\n", .{point2});
    std.debug.print("rect: {any}\n", .{rect});

    // You can access struct members using `.`. Works for nesting too.
    std.debug.print("point1.x: {}\n", .{point1.x});
    std.debug.print("rect.bottom_right.y: {}\n", .{rect.bottom_right.y});

    // Methods are accessed in a similar way.
    std.debug.print("rect.area(): {}\n", .{rect.area()});

    // Structs are copied.
    point1.x = 99;
    std.debug.print("point1.x: {}\n", .{point1.x});
    std.debug.print("rect.top_left.x: {}\n", .{rect.top_left.x});
}
