const std = @import("std");
const Allocator = std.mem.Allocator;

pub fn LinkedList(comptime T: type) type {
    return struct {
        const Self = @This();

        const Node = struct {
            next: ?*Node = null,
            value: T,

            pub fn append(self: *Node, node: ?*Node) !void {
                if (self.next) |next| {
                    try next.append(node);
                } else {
                    self.next = node;
                }
            }
        };

        head: ?*Node,
        allocator: Allocator,

        pub fn init(allocator: Allocator) Self {
            return LinkedList(T){
                .head = null,
                .allocator = allocator,
            };
        }

        pub fn deinit(self: *Self) void {
            _ = self;
        }

        pub fn prepend(self: *Self, value: T) !void {
            const node = try self.allocator.create(Node);
            node.* = .{
                .value = value,
                .next = self.head,
            };
            self.head = node;
        }

        pub fn append(self: *Self, value: T) !void {
            const node = try self.allocator.create(Node);
            node.* = .{
                .value = value,
            };

            if (self.head) |head| {
                try head.append(node);
            } else {
                self.head = node;
            }
        }

        pub fn pop(self: *Self) ?T {
            if (self.head) |head| {
                self.head = head.next;
                const value = head.value;
                self.allocator.destroy(head);
                return value;
            } else {
                return null;
            }
        }
    };
}
