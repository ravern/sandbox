# Kato

Toy virtual machine written in Rust.

_Abandoning this project for [zuko](https://github.com/ravernkoh/zuko)._

## Specification

Kato executes binaries specified in this section.

### Stack

Kato uses an operand stack and a call stack when executing. The operand stack is used to perform operations while the call stack is used to keep track of the current executing function.

### Instructions

Kato instructions are made up of an operation and its operands. They usually also manipulate the stacks to produce useful output. The address of the current instruction is stored in an instruction pointer, which can be manipulated to call functions.

Operations occupy the first byte of an instruction. They are split into two sections, the code and the operand type, which occupy 5 and 3 bits respectively. The code represents which operation (e.g. `push`, `add`) the instruction performs, while the type represents the type of each operand (e.g. `i16`, `f32`).

The following table shows all the possible types of operands.

| Type | Name  |
|------|-------|
| `1`  | `u8`  |
| `2`  | `u16` |
| `3`  | `u32` |
| `4`  | `i8`  |
| `5`  | `i16` |
| `6`  | `i32` |
| `7`  | `f32` |

The following table shows all the possible operations.

| Name   | Code* | Arity | Types | Stack  | Description                                                                        |
|--------|-------|-------|-------|--------|------------------------------------------------------------------------------------|
| `nope` | `1`   | 0     | -     | -      | Performs no operation                                                              |
| `push` | `2`   | 1     | All   | a - ab | Pushes the value to the operand stack                                              |
| `pop`  | `3`   | 0     | All   | ab - a | Pops a value from the operand stack                                                |
| `add`  | `4`   | 0     | All   | ab - a | Adds the top two values of the operand stack                                       |
| `sub`  | `5`   | 0     | All   | ab - a | Subtracts the top value from the second value of the operand stack                 |
| `jump` | `6`   | 1     | All   | -      | Jumps to the specified instruction address if the top of the operand stack is zero |
| `dup`  | `7`   | 0     | All   | a - aa | Duplicates the top value of the operand stack                                      |
| `eq`   | `8`   | 0     | All   | ab - a | Compares the equality of the top two values of the operand stack                   |
| `neq`  | `9`   | 0     | All   | ab - a | Compares the inequality of the top two values of the operand stack                 |

_* Codes represent the value stored within the 5 bits._

### Structure

A Kato binary is simply a list of instructions stored continuously. The virtual machine can determine the size and number of operands based on the operation, and will thus read the appropriate number of bytes to extract the operands.

#### Constants

Constants (e.g. `0xf0f1`) should be stored in little-endian format. The internal stacks in Kato also use the little-endian format.
