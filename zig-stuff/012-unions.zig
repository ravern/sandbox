const std = @import("std");

// We could use an enum to represent a shape, but we can't store any
// shape data within it.
const ShapeEnum = enum {
    circle,
    rectangle,
    square,
};

// We can use a union instead. A union is like a struct, but it can only
// store one of its members at a time, rather than all at once.
const ShapeUnion = union {
    circle: struct { radius: f32 },
    rectangle: struct { width: f32, height: f32 },
    square: struct { size: f32 },
};

const TaggedShapeUnion = union(ShapeEnum) {
    circle: struct { radius: f32 },
    rectangle: struct { width: f32, height: f32 },
    square: struct { size: f32 },
};

// We can also use an automatic enum to tag the union.
const TaggedShapeUnionAutomatic = union(enum) {
    circle: struct { radius: f32 },
    rectangle: struct { width: f32, height: f32 },
    square: struct { size: f32 },
};

pub fn main() !void {
    // We can access members of a union directly.
    const some_rectangle = ShapeUnion{ .rectangle = .{ .width = 3.14, .height = 2.71 } };
    std.debug.print("some_rectangle has width {} and height {}\n", .{ some_rectangle.rectangle.width, some_rectangle.rectangle.height });

    // But what if we do the following???
    //                ------------ try to uncomment the code below -------------
    // std.debug.print("some_rectangle has radius {}\n", .{some_rectangle.circle.radius});

    // Notice how given a shape of type ShapeUnion, we don't know which kind
    // shape it is? We can't switch on it...
    //                ------------ try to uncomment the code below -------------
    // const some_shape = ShapeUnion{ .circle = .{ .radius = 3.14 } };
    // switch (some_shape) {
    //     .circle => std.debug.print("some_shape is a circle of radius {}\n", .{some_shape.circle.radius}),
    //     .rectangle => std.debug.print("some_shape is a rectangle of width {} and height {}\n", .{ some_shape.rectangle.width, some_shape.rectangle.height }),
    //     .square => std.debug.print("some_shape is a square of size {}\n", .{some_shape.square.size}),
    // }

    // We must "tag" the union with an enum to know which kind of shape it is.
    const some_shape = TaggedShapeUnion{ .circle = .{ .radius = 3.14 } };
    switch (some_shape) {
        .circle => std.debug.print("some_shape is a circle of radius {}\n", .{some_shape.circle.radius}),
        .rectangle => std.debug.print("some_shape is a rectangle of width {} and height {}\n", .{ some_shape.rectangle.width, some_shape.rectangle.height }),
        .square => std.debug.print("some_shape is a square of size {}\n", .{some_shape.square.size}),
    }
}
