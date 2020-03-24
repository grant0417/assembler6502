use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;

use clap::{Arg, App};

/// Array of all opcodes in alphabetical order
const OPS: [&str; 56] = [
    "ADC", "AND", "ASL", "BCC", "BCS", "BEQ", "BIT", "BMI", "BNE", "BPL", "BRK", "BVC", "BVS",
    "CLC", "CLD", "CLI", "CLV", "CMP", "CPX", "CPY", "DEC", "DEX", "DEY", "EOR", "INC", "INX",
    "INY", "JMP", "JSR", "LDA", "LDX", "LDY", "LSR", "NOP", "ORA", "PHA", "PHP", "PLA", "PLP",
    "ROL", "ROR", "RTI", "RTS", "SBC", "SEC", "SED", "SEI", "STA", "STX", "STY", "TAX", "TAY",
    "TSX", "TXA", "TXS", "TYA",
];

/// Array of hex values for each mode for each opcode
const OPS_HEX: [[i32; 13]; 56] = [
    //  0    1    2    3    4    5    6    7    8    9   10   11   12
    //imp  acc  imm  abs  abx  aby  zpg  zpx  zpy  ind  inx  iny  rel
    [  -1,  -1,0x69,0x6d,0x7d,0x79,0x65,0x75,  -1,  -1,0x61,0x71,  -1], //ADC
    [  -1,  -1,0x29,0x2d,0x3d,0x39,0x25,0x35,  -1,  -1,0x21,0x31,  -1], //AND
    [  -1,0x0a,  -1,0x0e,0x1e,  -1,0x06,0x16,  -1,  -1,  -1,  -1,  -1], //ASL
    [  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,0x90], //BCC
    [  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,0xb0], //BCS
    [  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,0xf0], //BEQ
    [  -1,  -1,  -1,0x2c,  -1,  -1,0x24,  -1,  -1,  -1,  -1,  -1,  -1], //BIT
    [  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,0x30], //BMI
    [  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,0xd0], //BNE
    [  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,0x10], //BPL
    [0x00,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //BRK
    [  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,0x50], //BVC
    [  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,0x70], //BVS
    [0x18,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //CLC
    [0xd8,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //CLD
    [0x58,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //CLI
    [0xb8,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //CLV
    [  -1,  -1,0xc9,0xcd,0xdd,0xd9,0xc5,0xd5,  -1,  -1,0xc1,0xd1,  -1], //CMP
    [  -1,  -1,0xe0,0xec,  -1,  -1,0xe4,  -1,  -1,  -1,  -1,  -1,  -1], //CPX
    [  -1,  -1,0xc0,0xcc,  -1,  -1,0xc4,  -1,  -1,  -1,  -1,  -1,  -1], //CPY
    [  -1,  -1,  -1,0xce,0xde,  -1,0xc6,0xd6,  -1,  -1,  -1,  -1,  -1], //DEC
    [0xca,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //DEX
    [0x88,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //DEY
    [  -1,  -1,0x49,0x4d,0x5d,0x59,0x45,0x55,  -1,  -1,0x41,0x51,  -1], //EOR
    [  -1,  -1,  -1,0xee,0xfe,  -1,0xe6,0xf6,  -1,  -1,  -1,  -1,  -1], //INC
    [0xe8,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //INX
    [0xc8,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //INY
    [  -1,  -1,  -1,0x4c,  -1,  -1,  -1,  -1,  -1,0x6c,  -1,  -1,  -1], //JMP
    [  -1,  -1,  -1,0x20,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //JSR
    [  -1,  -1,0xa9,0xad,0xbd,0xb9,0xa5,0xb5,  -1,  -1,0xa1,0xb1,  -1], //LDA
    [  -1,  -1,0xa2,0xae,  -1,0xbe,0xa6,  -1,0xb6,  -1,  -1,  -1,  -1], //LDX
    [  -1,  -1,0xa0,0xac,0xbc,  -1,0xa4,0xb4,  -1,  -1,  -1,  -1,  -1], //LDY
    [  -1,0x4a,  -1,0x4e,0x5e,  -1,0x46,0x56,  -1,  -1,  -1,  -1,  -1], //LSR
    [0xea,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //NOP
    [  -1,  -1,0x09,0x0d,0x1d,0x19,0x05,0x15,  -1,  -1,0x01,0x11,  -1], //ORA
    [0x48,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //PHA
    [0x08,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //PHP
    [0x68,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //PLA
    [0x28,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //PLP
    [  -1,0x2a,  -1,0x2e,0x3e,  -1,0x26,0x36,  -1,  -1,  -1,  -1,  -1], //ROL
    [  -1,0x6a,  -1,0x6e,0x7e,  -1,0x66,0x76,  -1,  -1,  -1,  -1,  -1], //ROR
    [0x40,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //RTI
    [0x60,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //RTS
    [  -1,  -1,0xe9,0xed,0xfd,0xf9,0xe5,0xf5,  -1,  -1,0xe1,0xf1,  -1], //SBC
    [0x38,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //SEC
    [0xf8,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //SED
    [0x78,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //SEI
    [  -1,  -1,  -1,0x8d,0x9d,0x99,0x85,0x95,  -1,  -1,0x81,0x91,  -1], //STA
    [  -1,  -1,  -1,0x8e,  -1,  -1,0x86,  -1,0x96,  -1,  -1,  -1,  -1], //STX
    [  -1,  -1,  -1,0x8c,  -1,  -1,0x84,0x94,  -1,  -1,  -1,  -1,  -1], //STY
    [0xaa,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //TAX
    [0xa8,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //TAY
    [0xba,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //TSX
    [0x8a,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //TXA
    [0x9a,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //TXS
    [0x98,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1], //TYA
];

/// For indicating the size of the address
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum AddressSize {
    /// Represents a standard `u8` value (1 bytes)
    U8,
    /// Represents a standard `u16` value (2 bytes)
    U16,
    /// Represents a unknown sized value
    Unknown,
}

/// Storage for defines to be processed in later pass
struct Define {
    size: AddressSize,
    value: HexPair,
}

/// Storage for hex addresses
struct HexPair {
    /// The least-significant byte in an ordered byte pair
    lower: u8,
    /// The most-significant byte in an ordered byte pair
    upper: u8,
}

/// Storage for labels to be processed in later pass.
#[derive(Clone)]
struct Label {
    /// The string representation of a label
    name: String,
    /// Value of location in memory of the first byte on the line with a label
    address: u16,
}

/// Storage for final bytecode to be formatted, with exception of labels
/// which are transformed in the final pass.
struct MachineCode {
    /// Debug info to go at beginning of line in debug mode, split into lines in `Vec`
    debug_info: Vec<String>,
    /// Binary data split into `Vec`s for each line then each byte
    binary_data: Vec<Vec<String>>,
    defines: HashMap<String, Define>
}

impl MachineCode {
    fn new(_size: usize) -> MachineCode {
        MachineCode {
            debug_info: vec![],
            binary_data: vec![],
            defines: HashMap::new()
        }
    }

    fn insert_debug_info(&mut self, line: usize, info: String) {
        self.debug_info.insert(line, info);
    }

    fn insert_byte(&mut self, line: usize, byte: String) {
        match self.binary_data.get_mut(line) {
            Some(l) => l.push(byte),
            None => {
                self.binary_data.insert(line, vec![]);
                self.binary_data.get_mut(line).unwrap().push(byte);
            }
        }

    }
}

/// Currently unused, to be used to add more syntactic meaning to function calls
#[allow(dead_code)]
enum Addressing {
    Implied,
    Accumulator,
    Immediate,
    Absolute,
    AbsoluteXIndexed,
    AbsoluteYIndexed,
    Zeropage,
    ZeropageXIndexed,
    ZeropageYIndexed,
    IndirectYIndexed,
    Indirect,
    Relative,
}

/// Removes comments and tokenizes the imputed program as well as creating a
/// map for tracking symbols.
fn create_symbols_and_tokenize(
    code: &str,
) -> (
    Vec<Vec<String>>,
    HashMap<String, usize>,
    HashMap<String, Define>,
) {
    let mut labels = HashMap::new();
    let mut defines = HashMap::new();
    let mut tokens = Vec::new();

    let mut solo_label = Vec::new();

    let mut line_num = 0;


    for line in code.lines() {
        // Removes ; and splits into tokens
        let split_comments: Vec<&str> = line.split(';').collect();
        let split_tokens: Vec<&str> = split_comments[0].split_whitespace().collect();

        if !split_tokens.is_empty() {
            if split_tokens.iter().any(|s| s.contains('=')) {
                let split_eq: Vec<&str> = split_comments[0].split('=').collect();
                let address = address_size(split_eq[1].trim());
                let value = match address {
                    AddressSize::U8 => u8_decode(split_eq[1].trim()).unwrap(),
                    AddressSize::U16 => u16_decode(split_eq[1].trim()).unwrap(),
                    _ => panic!(),
                };
                if split_eq[0].contains("*") {
                    tokens.insert(line_num, vec!["*".to_string(), format!("{:02X} {:02X}", value.lower, value.upper).to_string()]);
                    line_num += 1;
                } else {
                    defines.insert(split_eq[0].trim().to_string(), Define { size: address, value });
                }
            } else if split_tokens.len() == 1 && !OPS.contains(&split_tokens[0]) && split_tokens[0].ends_with(':') {
                solo_label.push(split_tokens[0]);
            } else {
                tokens.insert(line_num, vec![]);

                for _ in 0..solo_label.len() {
                    labels.insert(solo_label.pop().unwrap().trim_end_matches(':').to_string(), line_num);
                }

                let remove_label = if !OPS.contains(&split_tokens[0]) {
                    labels.insert(split_tokens[0]
                                      .trim_end_matches(':').to_string(), line_num);
                    1
                } else {
                    0
                };

                for token in split_tokens.iter().skip(remove_label) {
                    tokens.get_mut(line_num).unwrap().push((*token).to_string())
                }
                line_num += 1;
            }
        }
    }

    (tokens, labels, defines)
}

/// Takes tokens and symbols and outputs the hex machine code. Has an option for
/// a debug mode which prints a verbose that shows all information needed for
/// human legibility.
fn tokens_to_machine_code(
    tokens: &[Vec<String>],
    labels: &HashMap<String, usize>,
    defines: &HashMap<String, Define>,
) -> Result<(MachineCode, HashMap<String, Label>), Box<dyn Error>> {
    let mut machine_code = MachineCode::new(0);
    let mut label_locations = HashMap::new();
    let mut byte_num = 0;

    for (line_num, line) in tokens.iter().enumerate() {

        if line[0].as_str() == "*" || line[0].as_str() == "ORG" {
            // Set Location
            machine_code.insert_byte(line_num, "*".parse().unwrap());
            machine_code.insert_byte(line_num, (&line[1]).parse().unwrap());
            let val = u16_decode(&line[1]).unwrap();
            byte_num = (val.lower as u16) + (val.upper as u16) * 0x100;
        } else {
            let op_name = line[0].clone();

            let op = OPS.iter().position(|&s| s == op_name).unwrap_or_else(|| panic!(format!("Unknow opcode: {}", op_name)));

            let mut sym = "".to_string();

            for (label, label_line) in labels {
                if line_num == *label_line {
                    sym = label.clone();
                    let l = Label {
                        name: label.clone(),
                        address: byte_num,
                    };
                    label_locations.insert(label.clone(), l);
                }
            }

            machine_code.insert_debug_info(line_num,
                                           format!("{:<04X} {:<06} {:<03} {:<012} ",
                                                   &byte_num, &sym, &op_name, line.get(1).unwrap_or(&"".to_string())).to_string()
            );

            if line.len() == 1 {
                machine_code.insert_byte(line_num, format_opcode(op, 0));
            } else if line.len() == 2 {
                if line[1].as_str() == "A" {
                    // Accumulator Mode
                    machine_code.insert_byte(line_num, format_opcode(op, 1));
                } else {
                    let address = line[1]
                        .trim_end_matches(",X")
                        .trim_end_matches(",Y")
                        .trim_end_matches(')')
                        .trim_end_matches(",X")
                        .trim_start_matches('(')
                        .trim_start_matches('#');

                    let address_str = address_to_string(address, &labels, &defines);
                    let mut num_bit = address_size(address);

                    if labels.contains_key(address) {
                        num_bit = if &op_name == "JMP" || &op_name == "JSR" {
                            AddressSize::U16
                        } else {
                            AddressSize::U8
                        };
                    } else if defines.contains_key(address.trim_start_matches('<').trim_start_matches('>')) {
                        num_bit = if address.starts_with('<') || address.starts_with('>') {
                            AddressSize::U8
                        } else {
                            defines.get(address).unwrap().size
                        }
                    }

                    match num_bit {
                        AddressSize::U8 => byte_num += 1,
                        AddressSize::U16 => byte_num += 2,
                        _ => {}
                    }

                    if line[1].starts_with('#') {
                        // Immediate Mode
                        machine_code.insert_byte(line_num, format_opcode(op, 2));
                    } else if line[1].starts_with('(') {
                        //Indirects
                        if line[1].ends_with(",X)") {
                            // Indexed Indirect
                            machine_code.insert_byte(line_num, format_opcode(op, 10));
                        } else if line[1].ends_with("),Y") {
                            // Indirect Indexed
                            machine_code.insert_byte(line_num, format_opcode(op, 11));
                        } else if line[1].ends_with(')') {
                            // Indirect
                            machine_code.insert_byte(line_num, format_opcode(op, 9));
                        } else {
                            panic!("Unknown pattern: Line starts with '(' but does not end");
                        }
                    } else if line[1].ends_with(",X") {
                        // X-Indexed
                        match num_bit {
                            AddressSize::U8 => {
                                // Zero-page
                                machine_code.insert_byte(line_num, format_opcode(op, 7));
                            }
                            AddressSize::U16 => {
                                // Absolute
                                machine_code.insert_byte(line_num, format_opcode(op, 4));
                            }
                            _ => panic!(),
                        }
                    } else if line[1].ends_with(",Y") {
                        // Y-Indexed
                        match num_bit {
                            AddressSize::U8 => {
                                // Zero-page
                                machine_code.insert_byte(line_num, format_opcode(op, 8));
                            }
                            AddressSize::U16 => {
                                // Absolute
                                machine_code.insert_byte(line_num, format_opcode(op, 5));
                            }
                            _ => panic!(),
                        }
                    } else if num_bit == AddressSize::U8 {
                        if OPS_HEX[op][12] != -1 {
                            //Relative
                            machine_code.insert_byte(line_num, format_opcode(op, 12));
                        } else if OPS_HEX[op][6] != -1 {
                            //Zeropage
                            machine_code.insert_byte(line_num, format_opcode(op, 6));
                        } else if OPS_HEX[op][3] != -1 {
                            //Absolute with no high bytes
                            machine_code.insert_byte(line_num, format_opcode(op, 3));
                            byte_num += 1;
                            machine_code.insert_byte(line_num, (&address_str).parse().unwrap());
                            machine_code.insert_byte(line_num, "00".parse().unwrap());
                        }
                    } else if num_bit == AddressSize::U16 {
                        //Absolute
                        machine_code.insert_byte(line_num, format_opcode(op, 3));
                    } else {
                        panic!(format!("Op: {} Addr: {}, Size: {:?}", &op_name, &address, &num_bit))
                    }
                    match machine_code.binary_data.get(line_num) {
                        Some(s) => {
                            if s.len() <= 2 {
                                machine_code.insert_byte(line_num, address_str);
                            }
                        },
                        None => machine_code.insert_byte(line_num, address_str)
                    }
                }
            } else if line[1].len() > 2 {
                panic!("Too many tokens on line")
            }
            byte_num += 1;
        }
    }

    Ok((machine_code, label_locations))
}

/// Takes all the bytes and formats them properly for a binary file or human readability, also
/// transforms labels to correct values for jumps and branches.
fn machine_code_to_str(code: &MachineCode, labels: &HashMap<String, Label>, debug: bool) -> String {
    let mut s = "".to_string();
    let mut byte_pc = 0u16;

    if debug {
        for (name, define) in &code.defines {
            s.push_str(&format!("     {:<06} =   ${:<02X}{:<02X}\n",
                                name, define.value.upper, define.value.lower))
        }
    }

    for (index, line) in code.binary_data.iter().enumerate() {

        if line[0].as_str() == "*" {
            s.push_str("* = ")
        } else {
            let default = "".to_string();
            if debug {
                s.push_str(code.debug_info.get(index).unwrap_or(&default));
            }

            let x = line.first().unwrap_or(&default);
            let jmp_flag = x == "4C" || x == "6C" || x == "20";

            for byte in line {
                //TODO: Labels on lines above are not displayed
                let label = byte.trim_end_matches("label");
                if labels.contains_key(label) {
                    let pc: u16 = byte_pc;
                    let dest = labels.get(label).unwrap().address;
                    if !jmp_flag {
                        let dist = match dest.cmp(&pc) {
                            Ordering::Less => { (dest as i8).wrapping_sub(pc as i8 - 1) - 2i8 },
                            Ordering::Equal => { 0i8 },
                            Ordering::Greater => { (dest as i8).wrapping_sub(pc as i8) - 1i8 },
                        };
                        s.push_str(&format!("{:02X} ", dist));
                    } else {
                        let addr = labels.get(label).unwrap().address;
                        s.push_str(&format!("{:02X} {:02X}", addr % 0xff, addr / 0xff));
                        byte_pc += 1;
                    }
                } else {
                    s.push_str(&byte);
                    s.push_str(" ");
                }
                byte_pc += 1;
            }

            if !s.ends_with(" ") {
                s.push_str(" ");
            }

            if debug {
                s.push_str("\n");
            }
        }
    }
    s
}

/// Takes any form of address and returns the binary equivalent, with exception of labels
/// which are marked and later transformed in the last pass.
fn address_to_string(
    num: &str,
    labels: &HashMap<String, usize>,
    defines: &HashMap<String, Define>,
) -> String {
    if defines.contains_key(num.trim_start_matches('<').trim_start_matches('>')) {
        let define = defines.get(
            num.trim_start_matches('<').trim_start_matches('>')).unwrap();
        if num.starts_with('<') {
            format!("{:02X}", define.value.lower)
        } else if num.starts_with('>') {
            format!("{:02X}", define.value.upper)
        } else {
            match define.size {
                AddressSize::U8 => format!("{:02X}", define.value.lower),
                AddressSize::U16 => format!("{:02X} {:02X}", define.value.lower, define.value.upper),
                _ => panic!(),
            }
        }
    } else if labels.contains_key(num) {
        let mut num = num.to_string();
        num.push_str("label");
        num
    } else {
        let width = address_size(&num);
        match width {
            AddressSize::U8 => format!("{:02X}", u8_decode(&num).unwrap().lower),
            AddressSize::U16 => {
                let value = u16_decode(&num).unwrap();
                format!("{:02X} {:02X}", value.lower, value.upper)
            }
            _ => {
                panic!(format!("Unknown value: {}\n\
                                Perhaps you meant to define a value or are using the wrong prefix.",
                                num))
            }
        }
    }
}

/// Decodes asm formatted 1 byte numbers and returns in the lower byte of a HexPair
fn u8_decode(num: &str) -> Option<HexPair> {
    if num.starts_with('$') && num.len() == 3 {
        Some(HexPair {
            upper: 0,
            lower: u8::from_str_radix(num.trim_start_matches('$'), 16).unwrap(),
        })
    } else if num.starts_with('%') && num.len() == 5 {
        Some(HexPair {
            upper: 0,
            lower: u8::from_str_radix(num.trim_start_matches('%'), 2).unwrap(),
        })
    } else if num.starts_with('0') && num.len() == 4 {
        Some(HexPair {
            upper: 0,
            lower: u8::from_str_radix(num.trim_start_matches('0'), 8).unwrap(),
        })
    } else {
        let num = num.parse::<u8>();
        match num {
            Ok(n) => Some(HexPair { upper: 0, lower: n }),
            Err(_) => None,
        }
    }
}

/// Decodes asm formatted 2 byte numbers and returns as HexPair
fn u16_decode(num: &str) -> Option<HexPair> {
    if num.starts_with('$') && num.len() == 5 {
        Some(HexPair {
            upper: u8::from_str_radix(&num[1..=2], 16).unwrap(),
            lower: u8::from_str_radix(&num[3..=4], 16).unwrap(),
        })
    } else if num.starts_with('%') && num.len() == 9 {
        Some(HexPair {
            upper: u8::from_str_radix(&num[1..=4], 2).unwrap(),
            lower: u8::from_str_radix(&num[5..=8], 2).unwrap(),
        })
    } else if num.starts_with('0') && num.len() == 7 {
        Some(HexPair {
            upper: u8::from_str_radix(&num[1..=3], 2).unwrap(),
            lower: u8::from_str_radix(&num[4..=6], 2).unwrap(),
        })
    } else {
        let num = num.parse::<i32>().unwrap_or(0);
        let upper = num / 16;
        let lower = num % 16;
        Some(HexPair {
            upper: upper as u8,
            lower: lower as u8,
        })
    }
}

/// Returns if the number is 8 bit or 16 bit
fn address_size(num: &str) -> AddressSize {
    if num.starts_with('$') {
        if num.len() == 3 {
            AddressSize::U8
        } else if num.len() == 5 {
            AddressSize::U16
        } else {
            AddressSize::Unknown
        }
    } else if num.starts_with('%') {
        if num.len() == 5 {
            AddressSize::U8
        } else if num.len() == 9 {
            AddressSize::U16
        } else {
            AddressSize::Unknown
        }
    } else if num.starts_with('0') {
        if num.len() == 4 {
            AddressSize::U8
        } else if num.len() == 7 {
            AddressSize::U16
        } else {
            AddressSize::Unknown
        }
    } else if num.chars().all(char::is_numeric) {
        let parsed_num: i32 = num.parse().unwrap();
        if parsed_num <= 0xff {
            AddressSize::U8
        } else if parsed_num > 0xff {
            AddressSize::U16
        } else {
            AddressSize::Unknown
        }
    } else {
        AddressSize::Unknown
    }
}

// TODO: Use Addressing enum instead of mode
/// Formats opcodes in hex
fn format_opcode(op: usize, mode: usize) -> String {
    format!("{:02X}", &OPS_HEX[op][mode])
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Mode {
    Hex,
    Debug,
    Binary,
}

/// Main call for the binary, processes arguments and calls functions to do processing
fn main() {

    let matches = App::new("6502 Assembler")
        .version("0.1")
        .author("Grant G.")
        .about("Assembles 6502 Assembly into machine code")
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .conflicts_with("binary")
            .help("Outputs machine code formatted with helpful information for debugging"))
        .arg(Arg::with_name("binary")
            .short("b")
            .long("binary")
            .help("Outputs the machine code in a binary format"))
        .arg(Arg::with_name("INPUT")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("A file to output the machine code to"))
        .get_matches();

    let mode = {
        if matches.is_present("debug") {
            Mode::Debug
        } else if matches.is_present("binary") {
            Mode::Binary
        } else {
            Mode::Hex
        }
    };

    let file= matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT");

    let code = match fs::read_to_string(&file) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Unable to read file: {}", &file);
            return;
        }
    };

    // Transforms the code to uppercase since 6502 asm is case insensitive
    let code = code.to_uppercase();

    let (tokens, labels, defines) = create_symbols_and_tokenize(&code);

    let (mut machine_code_labeled, labels) =
        tokens_to_machine_code(&tokens, &labels, &defines).unwrap();

    machine_code_labeled.defines = defines;

    let machine_code = machine_code_to_str(&machine_code_labeled, &labels, matches.is_present("debug"));

    if mode == Mode::Binary {
        match output {
            Some(output_file) => {
                let mut file = File::create(output_file).unwrap();
                let header = "6502ROM...".as_bytes();
                file.write(header).expect("Unable to write to file");
                for byte in machine_code.split_whitespace() {
                    file.write(&[u8::from_str_radix(byte, 16).unwrap()]).expect("Unable to write to file");
                }
            }
            None => {
                print!("{}", "6502ROM...");
                for byte in machine_code.split_whitespace() {
                    print!("{}", u8::from_str_radix(byte, 16).unwrap() as char);
                }
            }
        }
    } else {
        match output {
            Some(output_file) => {
                let mut file = File::create(output_file).unwrap();
                file.write_all(machine_code.as_ref()).expect("Unable to write to file");
            }
            None => {
                println!("{}", machine_code);
            }
        }
    }
}
