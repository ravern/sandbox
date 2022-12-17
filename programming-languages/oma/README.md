# Oma

Practical dynamic programming language.

## Usage

Run the REPL.

```bash
oma repl
```

Run a file.

```bash
oma run index.oma
```

Launch the debugger on a file.

```bash
oma debug index.oma
```

## Guide

### Data types

Oma supports the following basic data types.

```oma
number = 0.0
bool = true
string = "foo"
map = { foo: "bar", baz: "qux" }
array = [ 1, 2, 3 ]
```

### Variables and assignment

Variables in Oma are simply assigned to.

```oma
foo = 12
bar = foo
```

Variables are mutable, meaning they can be modified whenever, whereever. Oma expects users to be disciplined in mutation.

```oma
foo = 12
foo = true
foo = 43.2
```

### Control flow and loops

Oma supports the basic control flows and loops.

```oma
if condition {
  foo
} else if other_condition {
  bar
} else {
  baz
}

for foo in foos {
  foo.bar()
}

while foo {
  bar()
}
```

### Lambda expressions

All functions in Oma are closures.

```oma
foo = (bar) -> {
  bar + 2
}
```

### Pattern matching

A core feature of Oma is pattern matching. It can be used in many areas e.g. lambda parameters.

```oma
{ foo, bar } = { foo: 1, bar: 2 }
```

