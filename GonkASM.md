# GonkASM
The language used to control the GonkBox.

## Basic Syntax
When writing an instruction, the following syntax is used:
`mnemonic` `arg1` `arg2` `...`

### Registers
Registers can be accessed by writing a `%` symbol followed by their name.
For ease of use, the first letter of a register's name can be used instead of
its full name.

All registers are listed below:
- `bill`, `charlie`, `tim`: general purpose registers (16 bits/2 bytes)
  - byte access: gp registers can have their higher or lower byte accessed by
    suffixing them with \_h or \_l respectively (e.g. `bill_h` or `c_l`)
- `microwave`: jump instruction pointer
- `paul` **(read-only)**: instruction pointer

### Immediate Values
An immediate value (a value literally typed out in the code) can be used in place
of the source register in most source/destination instructions. For example, to
store the value `4` in the register `%charlie`, write `move 4, %charlie`.

### Addressing RAM
As RAM is just a pool of bytes, one after the other, we can address it with an
integer number of any form - the first byte is 1, the second is 2, etc. However,
bytes aren't big enough to address the entirety of the GonkBox's RAM, so RAM
addresses must be words (which have 256x the range).

To get the value of the byte or word in RAM at a given address, put the address
between `[` and `]` symbols. For example, the value of the fourth byte in ram is
`[4]`, and the value of the word charlie points to is `[%charlie]`.

### Defining Data
To make writing programs more convenient, pools of data can be declared so the
language will handle their positions/sizes and avoid overlaps automatically.
To make pools bigger than a single element, an argument `n` is needed to specify
how big to make the pool. These aren't actually instructions, as they're not run
by the program, instead just giving the compiler more info; thus, they're called
'commands' here.

Strings are just pools of bytes, but rather than giving all bytes the same
default value, they take consecutive values from the given `"string"`, and take 0
if the length of the string is less than the given `n`.

Commands for declaring byte pools:
| Mnemonic | Arguments    | Purpose                                               |
| -------- | -------------| ----------------------------------------------------- |
| `dbyte`  |              | Declare one byte                                      |
| `dbytes` | `n`          | Declare `n` bytes                                     |
| `ibyte`  | `def`        | Declare one byte defaulted to `def`                   |
| `ibytes` | `n`, `def`   | Declare `n` bytes all defaulted to `def`              |
| `istr`   | `"str"`      | Declare bytes defaulted from `"str"`, fitting `"str"` |
| `istrn`  | `n`, `"str"` | Declare `n` bytes defaulted from `"str"`              |

Commands for declaring word pools:
| `dword`  |                 | Declare one word                                    |
| `dwords` | `n`             | Declare `n` words                                   |
| `iword`  | `def`           | Declare one word defaulted to `def`                 |
| `iwords` | `n`, `def`      | Declare `n` words all defaulted to `def`            |

### Labels
Labels are a tool to give a specific byte address a name for convenience when
writing; when the program runs, rather than using the label, it will use the
value they refer to. The syntax for writing a label is `.label` `name`. Labels
must not be declared more than once, as that would create ambiguity on which is
being referred to.

When a label is declared, its value may be one of two things:
- the byte address of the instruction following it
- the byte address of the data pool following it (see Defining Data)

One label is reserved and must be included in a program: `start`. Add `.label
start` to identify the following instruction as the first instruction to run in
the program.

### Sources and Destinations
Sources/Destination form is a common format for arguments in instructions.
Several rules apply for what values can be used in these arguments.
- A source can be an immediate value, a RAM byte address, or a register name.
- A destination can be a RAM byte address or a register name.
- Data can not be processed exclusively in RAM; at least one argument has to be
  a register.
- The sizes of the source and destination must match the size the instruction
  excepts (words vs bytes).
  - Immediate values will scale automatically (but will create warnings if an
    overflow occurs).

In any instructions with a source and destination argument, the form is `mnemonic`
`source` `destination` `...`

## Instructions
| Mnemonic | Arguments        | Purpose                                    |
| -------- | ---------------- | ------------------------------------------ |
| `move`   | `source`, `dest` | Copy the value of `source` to `dest`       |
| `add`    | `source`, `dest` | Add the value of `source` onto `dest`      |
| `sub`    | `source`, `dest` | Subtract the value of `source` from `dest` |
| `comp`   | `arg1`, `arg2`   | Compare `arg1` and `arg2`, store in `<cr>` |
| `jump`   |                  | Move `%paul` to `%microwave`               |
| `jumpe`  |                  | Jump only if `<cr>` has EQUAL set          |
| `jumpne` |                  | Jump only if `<cr>` doesnt have EQUAL set  |
| `jumpl`  |                  | Jump only if `<cr>` has LESS set           |
| `jumpg`  |                  | Jump only if `<cr>` has GREATER set        |
| `stop`   |                  | Stop execution of the program              |

## Example Program
```
@bring WRITEIO, READIO
.label string
istr "hello world\n"

.label start
; set bill and charlie to 4 and 5 respectively
move 4, %bill
move 5, %charlie
; add bill and charlie, store in charlie (should be 9)
add %bill, %charlie

; set tim to 10
move 10, %tim
; compare the values of tim and charlie
comp %tim, %charlie
; set *hello as the position to jump to
move *hello, %microwave

; jump to *hello if %tim != %charlie
jumpne
; otherwise stop prematurely
stop

.label hello
```

