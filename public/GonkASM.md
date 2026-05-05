# GonkASM
The language used to control the GonkBox.

## Concepts
1. Registers: 1-Word internal variables where all the processing happens
    1. General purpose (GP) registers: any data can be used, read/write allowed
    2. Fixed purpose (FP) registers: registers with a specific internal purpose
2. RAM/Memory: the block of bytes that can be read/written to by programs,
including data and the program's own code
3. State: the combination of all registers and RAM; any processor with the same
state will behave identically until an outside change is made
4. Instructions: processes built into the processor that change state in some way,
available to programmers with a certain 'mnemonic' (name)
5. Commands: hints used to declare blocks of data, written in code
6. Labels: names given to blocks of data or specific instructions
7. Immediates/Literals: values typed out in code

## Syntax
### Registers
Registers can be accessed by writing their name. For ease of use, the first letter
of a register's name can be used instead of its full name.

All registers are listed below:
- `bill`, `charlie`, `tim` *(GP)*: basic registers
- `microwave` *(FP)*: jump instruction pointer
- `paul` *(read-only FP)*: instruction pointer
- `canada` *(read-only FP)*: comparison results

Registers are 1-Word, aka 2-Byte; but sometimes processing needs to happen
per-byte. To use a register's higher or lower byte, suffix them with `_h` or
`_l` respectively (e.g. `bill_h` or `c_l`). This is *not* valid for FP
registers, as they should always be accessed in their intended way if accessed
at all.

### Addressing RAM
As RAM is just a pool of bytes, one after the other, we can address it with an
integer number of any form - the first byte is 1, the second is 2, etc. However,
bytes aren't big enough to address the entirety of the GonkBox's RAM, so RAM
addresses must be words (which have 256x the range).

To get the value of the byte or word in RAM at a given address, put the address
between after a `*` symbol. For example, the value of the word at the fourth byte
in ram is `*4`, and the value of the word charlie points to is `*charlie`.

### Instructions
| Mnemonic | Arguments        | Purpose                                      |
| -------- | ---------------- | ------------------------------------------   |
| `move`   | `source`, `dest` | Copy the value of `source` to `dest`         |
| `add`    | `source`, `dest` | Add the value of `source` onto `dest`        |
| `sub`    | `source`, `dest` | Subtract the value of `source` from `dest`   |
| `flip`   | `arg`            | Set arg to its two's complement (eg 1 to -1) |
| `comp`   | `arg1`, `arg2`   | Compare `arg1` and `arg2`, store in `canada` |
| `and`    | `arg1`, `arg2`   | And all bits of `arg1` with `arg2`           |
| `or`     | `arg1`, `arg2`   | Or all bits of `arg1` with `arg2`            |
| `nand`   | `arg1`, `arg2`   | Nand all bits of `arg1` with `arg2`          |
| `not`    | `arg1`           | Not all bits of `arg1`                       |
| `jump`   |                  | Move `paul` to `microwave`                   |
| `jumpe`  |                  | Jump only if `canada` has EQUAL set          |
| `jumpne` |                  | Jump only if `canada` doesnt have EQUAL set  |
| `jumpl`  |                  | Jump only if `canada` has LESS set           |
| `jumpg`  |                  | Jump only if `canada` has GREATER set        |
| `stop`   |                  | Stop execution of the program                |

### Arguments in Instructions
Several rules apply for what values can be used in these arguments.
- An argument may be an immediate value, a RAM address, or a register name.
- Data can not be processed exclusively in RAM; at least one argument has to be
  a register.
- Value sizes must match the size the instruction excepts (words vs bytes).
    - Immediate values will scale automatically, but will error if too big (i.e.
      numbers over 255 in a byte)

### Defining Data
Programs need data, and 'commands' are our tool for defining it in our program.
Each command creates a pool of data which will exist at the start of RAM, and
the language will automatically align everything so there's no
overlaps. Unlike instructions, commands don't actually run as part of
the program; they're more like hints to the compiler on how to position things.

Commands for declaring byte pools:
| Mnemonic | Arguments  | Purpose                                  |
| -------- | ---------- | ---------------------------------------- |
| `dbyte`  |            | Declare one byte                         |
| `dbytes` | `n`        | Declare `n` bytes                        |
| `ibyte`  | `def`      | Declare one byte defaulted to `def`      |
| `ibytes` | `n`, `def` | Declare `n` bytes all defaulted to `def` |

Commands for declaring word pools:
| Mnemonic | Arguments  | Purpose                                  |
| -------- | ---------- | ---------------------------------------- |
| `dword`  |            | Declare one word                         |
| `dwords` | `n`        | Declare `n` words                        |
| `iword`  | `def`      | Declare one word defaulted to `def`      |
| `iwords` | `n`, `def` | Declare `n` words all defaulted to `def` |

Strings are just pools of bytes, but rather than giving all bytes the same
default value, they take bytes in order from the given `str`.

Commands for declaring strings:
| Mnemonic | Arguments  | Purpose                                              |
| -------- | ---------- | ---------------------------------------------------- |
| `istr`   | `str`      | Declare a string equal to `str` with no extra chars  |
| `istrn`  | `n`, `str` | Declare a string equal to `str` with exactly n chars |

### Labels
Labels are ways to 'name' a part of the program. By writing a label before an
instruction or a command, you can refer to it later by its name. The syntax
for writing a label is `label` `name`. Labels must not be declared more than once,
as that would create ambiguity on which is being referred to.

One label is reserved and must be included in a program: `start`. Add `label
start` before an instruction to set it as the start of the program.

### Immediate and Literal Values
Immediate values are used to write constant values in code. For example,
setting a register to the value `4` can be done simply with `move 4 charlie`.

Strings are similar, and are called literal values. They are defined by two
quotation marks, with the characters between them being the content of the
string, like so: `"Hello world!"`.

Additionally, 'escapes' exist to define special data in the string that aren't
just normal characters. These are marked by a backslash `\` before a character,
and are displayed differently in the editor.
- `\n` is a new line
- `\"` adds a quotation mark in the middle of a string
- `\\` adds a non-escape backslash in a string

Single characters can also be written, surrounded by single quotes `'` instead
of double quotes. These will be interpreted as numbers according to their ASCII
encoding, meaning they can be inserted in places where immediate values are
accepted.

## Example Program
```
label msg               ; refer to the next element as 'msg' in code
istr "hello world!\n"   ; store a string in the binary

label start	            ; first instruction to run
move 1 bill             ; bill = 1
move 2 charlie          ; charlie = 2
add bill charlie        ; charlie += bill (charlie = 3)

comp bill charlie       ; store bill in relation to charlie
move print microwave    ; set our jump address to the print section
jumpne                  ; jump to microwave if bill != charlie
stop

label print
$PRINT msg              ; print the string at the address
stop                    ; programs must always end with stop
```

