# 6502 Assembler

This assembler takes 6502 ASM and converts it into a binary file
that is readable by the 6502 CPU emulator. 

Supported 6502 ASM features:
 * All addressing modes (including relative) and standard Opcodes
 * Hex, Binary, Octal, and Decimal number representation
 * Labels and defines
 * Comments
  * `<` and `>`
 
Features to be added:
 * Pragmas (`.BYTE1`, `.WORD`, `.TEXT`, ect.)
 * PC address setting (`* = $0000`, `ORG`)
 
Things not in the scope of the project:
 * A full macro engine
 * Non standard opcodes
 
The binary takes the file with the assembly as an argument and two optional 
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
        JMP RESET   ; Sets up the inital conditions
LOOP:   ADC $00     ; Adds the last value
        LDX $01     ; Moves the 
        STX $00
        STA $01     ; Stores the new number
        BVS RESET   ; Resets if overflow flags is set
        JMP LOOP    ; Otherwise loops
RESET:  CLV
        LDA VAL     ; Resets values to inital loop conditions
        STA $00
        LDA #$00
        STA $01
        JMP START   ; Returns to loop
```

The program under debug mode would render the result as: 
```asm
     VAL    =   $0001
0000        JMP RESET        4C 12 00
0003 LOOP   ADC $00          6D 00 00 
0006        LDX $01          A6 01 
0008        STX $00          86 00 
000A        STA $01          8D 01 00 
000D        BVS RESET        70 03 
000F        JMP LOOP         4C 03 00
0012 RESET  CLV              B8 
0013        LDA VAL          AD 01 00 
0016        STA $00          8D 00 00 
0019        LDA #$00         A9 00 
001B        STA $01          8D 01 00 
001E        JMP LOOP         4C 03 00
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
