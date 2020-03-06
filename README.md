# 6502 Assembler

This assembler takes 6502 ASM and converts it into a plain text file
or binary file that is readable by the 6502 CPU emulator. 

Supported 6502 ASM features:
 * All addressing modes (including relative) and standard Opcodes
 * Hex, Binary, Octal, and Decimal number representation
 * Labels and defines
 * Comments
  * `<` and `>`
 
Features to be added:
 * Pragmas (`.BYTE`, `.WORD`, `.TEXT`, ect.)
 * PC address setting (`* = $0000`, `ORG`)
 
Things not in the scope of the project:
 * A full macro engine
 * Non standard opcodes
 
The assembler takes the file with the assembly as an argument and two optional 
arguments of `--pretty-print`, which instead of outputting a binary format will 
print in a human readable format, and `--output file_name`, which will print the 
output to a file as an alternative to the standard output.

The program is quite buggy in its current state but will improve as the 6502
emulator matures.
 
## 6502 ASM basics 

Each [opcode](https://www.masswerk.at/6502/6502_instruction_set.html) 
is either implied or takes some type of address. The address modes for each opcode
are given in the above linked page. When a numeric value is needed you can
use the prefixes `$` (hex), `0` (octal), and `%` (binary). Decimal can also be used 
un-prefixed. To get the lower byte of a 16 bit use < and > for the upper byte. 
Defines, similar to constant variables, can be used to assign values to a name using
the syntax `name = value`.

Here is a simple program to loop through the Fibonacci sequence under 255:
```asm
        VAL = $01
        OLD = $FD
        NEW = $FE
        JMP RESET   ; Sets up the inital conditions
LOOP:   ADC NEW     ; Adds the last value
        LDX OLD     ; Moves the
        STX NEW
        STA OLD     ; Stores the new number
        BVS RESET   ; Resets if overflow flags is set
        JMP LOOP    ; Otherwise loops
RESET:  CLC
        LDA #VAL    ; Resets values to inital loop conditions
        STA NEW
        LDA #$00
        STA OLD
        JMP LOOP    ; Returns to loop
```

The program under debug mode would render the result as: 
```asm
     NEW    =   $00FE
     VAL    =   $0001
     OLD    =   $00FD
0000        JMP RESET        4C 10 00 
0003 LOOP   ADC NEW          65 FE 
0005        LDX OLD          A6 FD 
0007        STX NEW          86 FE 
0009        STA OLD          85 FD 
000B        BVS RESET        70 03 
000D        JMP LOOP         4C 03 00 
0010 RESET  CLC              18 
0011        LDA #VAL         A9 01 
0013        STA NEW          85 FE 
0015        LDA #$00         A9 00 
0017        STA OLD          85 FD 
0019        JMP LOOP         4C 03 00 
```

The left column represents the position of the first byte in memory. The next
column represents the defines and labels. The next two columns are opcodes and
addresses. The last columns are the resulting machine code from the line.

## How to run the program

Simply clone the repository using Git:

```shell script
git clone https://github.com/grant0417/assembler6502
```

The using [Cargo](https://www.rust-lang.org/learn/get-started) 
run the command on a file with the 6502 code in it:

```shell script
cargo run -- file_name
```
