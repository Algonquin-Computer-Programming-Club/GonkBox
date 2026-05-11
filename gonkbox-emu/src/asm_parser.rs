#![allow(unused)]

use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

use wasm_bindgen::prelude::*;

use strum_macros::{AsRefStr, EnumIter, FromRepr};

use web_sys::console;

use phf::phf_map;

use crate::util;

use crate::tokenizer::{GonkASMToken, GonkASMTokenType};

/*
 * PROGRAM MAPPING
 */
#[derive(Clone, Debug)]
struct ArgumentDescriptor {
    accept_word: bool,
    accept_register: bool,
    accept_identifier: bool,
    accept_immediate: bool,
    accept_string: bool,
    accept_ram: bool,
}

const ARGDESC_IMONLY: &'static ArgumentDescriptor = &ArgumentDescriptor {
    accept_word: true,
    accept_register: false,
    accept_identifier: false,
    accept_immediate: true,
    accept_string: false,
    accept_ram: false,
};

const ARGDESC_IMONLY_BYTE: &'static ArgumentDescriptor = &ArgumentDescriptor {
    accept_word: false,
    accept_register: false,
    accept_identifier: false,
    accept_immediate: true,
    accept_string: false,
    accept_ram: false,
};

const ARGDESC_STRINGONLY: &'static ArgumentDescriptor = &ArgumentDescriptor {
    accept_word: false,
    accept_register: false,
    accept_identifier: false,
    accept_immediate: false,
    accept_string: true,
    accept_ram: false,
};

const ARGDESC_GENERAL: &'static ArgumentDescriptor = &ArgumentDescriptor {
    accept_word: true,
    accept_register: true,
    accept_identifier: true,
    accept_immediate: true,
    accept_string: false,
    accept_ram: true,
};

const ARGDESC_REGONLY: &'static ArgumentDescriptor = &ArgumentDescriptor {
    accept_word: false,
    accept_register: true,
    accept_identifier: false,
    accept_immediate: false,
    accept_string: false,
    accept_ram: false,
};

#[derive(Clone)]
struct Descriptor<T> {
    key_type: T,
    argument_descriptors: &'static [&'static ArgumentDescriptor],
}

#[derive(Debug)]
pub struct LayoutObject {
    size: u16,
    defaults: Vec<u8>,
}

#[repr(u8)]
#[derive(Clone, Debug, EnumIter, FromRepr)]
pub enum RegisterName {
    // general purpose registers
    Bill,
    BillLow,
    BillHigh,
    Charlie,
    CharlieLow,
    CharlieHigh,
    Tim,
    TimLow,
    TimHigh,

    // instruction pointer
    Paul,
    // jump pointer
    Microwave,

    // comparison flag register
    Canada,
}

fn is_register_byte(register_name: &RegisterName) -> bool {
    matches!(register_name, &RegisterName::BillLow)
        | matches!(register_name, &RegisterName::BillHigh)
        | matches!(register_name, &RegisterName::CharlieLow)
        | matches!(register_name, &RegisterName::CharlieHigh)
        | matches!(register_name, &RegisterName::TimLow)
        | matches!(register_name, &RegisterName::TimHigh)
}

const REGISTER_NAMES: phf::Map<&'static str, RegisterName> = phf_map! {
    // gp registers support acronyms and high/low differentiation
    "bill" => RegisterName::Bill,
    "b" => RegisterName::Bill,
    "bill_h" => RegisterName::BillHigh,
    "b_h" => RegisterName::BillHigh,
    "bill_l" => RegisterName::BillLow,
    "b_l" => RegisterName::BillLow,

    "charlie" => RegisterName::Charlie,
    "c" => RegisterName::Charlie,
    "charlie_h" => RegisterName::CharlieHigh,
    "c_h" => RegisterName::CharlieHigh,
    "charlie_l" => RegisterName::CharlieLow,
    "c_l" => RegisterName::CharlieLow,

    "tim" => RegisterName::Tim,
    "t" => RegisterName::Tim,
    "tim_h" => RegisterName::TimHigh,
    "t_h" => RegisterName::TimHigh,
    "tim_l" => RegisterName::TimLow,
    "t_l" => RegisterName::TimLow,

    // jump address register supports acronyms, no differentiation (should take words only)
    "microwave" => RegisterName::Microwave,
    "m" => RegisterName::Microwave,

    // instruction pointer register only supports full name (should be rarely written anyway)
    "paul" => RegisterName::Paul,

    // comparison register only supports full name (should be rarely written anyway)
    "canada" => RegisterName::Canada,
};

#[repr(u8)]
#[derive(Clone, Debug, EnumIter, FromRepr)]
pub enum InstructionType {
    // data transfer
    Move,

    // math
    Add,
    Sub,
    Mul,
    Div,
    Inc,
    Dec,

    // logic
    Comp,
    Or,
    And,
    Nand,
    Not,

    // control
    Jump,
    JumpE,
    JumpNE,
    JumpL,
    JumpG,

    Stop,

    // debug - should be obvious these aren't normally accessible on a bare-metal style machine
    DebugLogNumber,
    DebugLogCharacter,
    DebugLogString,
}

const INSTRUCTION_TYPES: phf::Map<&'static str, Descriptor<InstructionType>> = phf_map! {
    "move" => Descriptor {
        key_type: InstructionType::Move,
        argument_descriptors: &[ARGDESC_GENERAL, ARGDESC_GENERAL],
    },

    "add" => Descriptor {
        key_type: InstructionType::Add,
        argument_descriptors: &[ARGDESC_GENERAL, ARGDESC_GENERAL],
    },
    "sub" => Descriptor {
        key_type: InstructionType::Sub,
        argument_descriptors: &[ARGDESC_GENERAL, ARGDESC_GENERAL],
    },
    "mul" => Descriptor {
        key_type: InstructionType::Mul,
        argument_descriptors: &[ARGDESC_GENERAL, ARGDESC_GENERAL],
    },
    "div" => Descriptor {
        key_type: InstructionType::Div,
        argument_descriptors: &[ARGDESC_GENERAL, ARGDESC_GENERAL],
    },
    "inc" => Descriptor {
        key_type: InstructionType::Inc,
        argument_descriptors: &[ARGDESC_GENERAL],
    },
    "dec" => Descriptor {
        key_type: InstructionType::Dec,
        argument_descriptors: &[ARGDESC_GENERAL],
    },

    "comp" => Descriptor {
        key_type: InstructionType::Comp,
        argument_descriptors: &[ARGDESC_GENERAL, ARGDESC_GENERAL],
    },
    "and" => Descriptor {
        key_type: InstructionType::And,
        argument_descriptors: &[ARGDESC_GENERAL, ARGDESC_GENERAL],
    },
    "or" => Descriptor {
        key_type: InstructionType::Or,
        argument_descriptors: &[ARGDESC_GENERAL, ARGDESC_GENERAL],
    },
    "nand" => Descriptor {
        key_type: InstructionType::Nand,
        argument_descriptors: &[ARGDESC_GENERAL, ARGDESC_GENERAL],
    },
    "not" => Descriptor {
        key_type: InstructionType::Not,
        argument_descriptors: &[ARGDESC_REGONLY],
    },

    "jump" => Descriptor {
        key_type: InstructionType::Jump,
        argument_descriptors: &[],
    },
    "jumpe" => Descriptor {
        key_type: InstructionType::JumpE,
        argument_descriptors: &[],
    },
    "jumpne" => Descriptor {
        key_type: InstructionType::JumpNE,
        argument_descriptors: &[],
    },
    "jumpl" => Descriptor {
        key_type: InstructionType::JumpL,
        argument_descriptors: &[],
    },
    "jumpg" => Descriptor {
        key_type: InstructionType::JumpG,
        argument_descriptors: &[],
    },

    "stop" => Descriptor {
        key_type: InstructionType::Stop,
        argument_descriptors: &[],
    },

    "dlogn" => Descriptor {
        key_type: InstructionType::DebugLogNumber,
        argument_descriptors: &[ARGDESC_GENERAL]
    },
    "dlogc" => Descriptor {
        key_type: InstructionType::DebugLogCharacter,
        argument_descriptors: &[ARGDESC_GENERAL]
    },
    "dlogs" => Descriptor {
        key_type: InstructionType::DebugLogString,
        argument_descriptors: &[ARGDESC_GENERAL]
    },
};

#[derive(Clone, Debug)]
pub enum CommandType {
    // byte pools
    DByte,
    DBytes,
    IByte,
    IBytes,

    // strings
    IStr,
    IStrN,

    // word pools
    DWord,
    DWords,
    IWord,
    IWords,
}

const COMMAND_TYPES: phf::Map<&'static str, Descriptor<CommandType>> = phf_map! {
    "dbyte" => Descriptor {
        key_type: CommandType::DByte,
        argument_descriptors: &[],
    },
    "dbytes" => Descriptor {
        key_type: CommandType::DBytes,
        argument_descriptors: &[ARGDESC_IMONLY],
    },
    "ibyte" => Descriptor {
        key_type: CommandType::IByte,
        argument_descriptors: &[ARGDESC_IMONLY_BYTE],
    },
    "ibytes" => Descriptor {
        key_type: CommandType::IBytes,
        argument_descriptors: &[ARGDESC_IMONLY, ARGDESC_IMONLY_BYTE],
    },

    "istr" => Descriptor {
        key_type: CommandType::IStr,
        argument_descriptors: &[ARGDESC_STRINGONLY],
    },
    "istrn" => Descriptor {
        key_type: CommandType::IStrN,
        argument_descriptors: &[ARGDESC_IMONLY, ARGDESC_STRINGONLY],
    },

    "dword" => Descriptor {
        key_type: CommandType::DWord,
        argument_descriptors: &[],
    },
    "dwords" => Descriptor {
        key_type: CommandType::DWords,
        argument_descriptors: &[ARGDESC_IMONLY],
    },
    "iword" => Descriptor {
        key_type: CommandType::IWord,
        argument_descriptors: &[ARGDESC_IMONLY],
    },
    "iwords" => Descriptor {
        key_type: CommandType::IWords,
        argument_descriptors: &[ARGDESC_IMONLY, ARGDESC_IMONLY],
    },
};

// e.g. %bill or [%charlie]
#[derive(Clone, Debug)]
pub struct RegisterArgument {
    register_name: RegisterName,
    treat_as_address: bool,
}

impl RegisterArgument {
    pub fn get_register_name(&self) -> RegisterName {
        self.register_name.clone()
    }

    pub fn get_is_address(&self) -> bool {
        self.treat_as_address
    }
}

// e.g. 4 or [0x10]
#[derive(Clone, Debug)]
pub struct ImmediateArgument {
    value: u16,
    treat_as_address: bool,
}

impl ImmediateArgument {
    pub fn get_value(&self) -> u16 {
        self.value
    }

    pub fn get_is_address(&self) -> bool {
        self.treat_as_address
    }
}

// e.g. print or [start]
#[derive(Clone, Debug)]
pub struct LabelArgument {
    identifier: String,
    treat_as_address: bool,
}

#[derive(Clone, Debug)]
pub struct StringArgument {
    value: String,
}

#[derive(Clone, Debug)]
pub enum Argument {
    Register(RegisterArgument),
    Immediate(ImmediateArgument),
    Label(LabelArgument),
    StringLiteral(StringArgument),
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Instruction {
    instruction_type: InstructionType,
    arguments: Vec<Argument>,
}

impl Instruction {
    pub fn get_instruction_type(&self) -> InstructionType {
        self.instruction_type.clone()
    }

    pub fn get_arguments(&self) -> Vec<Argument> {
        self.arguments.to_vec()
    }
}

#[derive(Debug, PartialEq)]
pub struct LabelDefinition {
    identifier: String,
}

#[derive(AsRefStr, Debug)]
pub enum ParseErrorType {
    NoRegisters,
    NoIdentifiers,
    NoImmediates,
    NoStrings,
    NoWords,
    NoRamAddresses,

    FailedImmediateParse,
    FailedRamAddress,

    BadTokenType,
    BadArgumentType,

    ArgOverreach,

    UnterminatedString,

    ReusedLabel,
    UnnamedLabel,
    BadStartLabel,
    UnknownLabel,

    UnknownMacro,

    ByteRamAddress,

    RegisterNeeded,
    SizeMismatch,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct ParseError {
    error_type: ParseErrorType,
    description: &'static str,
    token: Option<GonkASMToken>,
}

#[wasm_bindgen]
impl ParseError {
    pub fn get_description(&self) -> String {
        self.description.to_owned()
    }

    pub fn get_token(&self) -> Option<GonkASMToken> {
        self.token.clone()
    }
}

fn parse_string(input: &Vec<GonkASMToken>, start_i: usize) -> Result<(String, usize), ParseError> {
    let mut pos = start_i;
    let mut string: String = String::from("");
    loop {
        string.push_str(&input[pos].value);
        if pos >= input.len() {
            return Err(ParseError {
                error_type: ParseErrorType::UnterminatedString,
                description: "String has to be started and ended by quotation marks.",
                token: Some(input[start_i].clone()),
            });
        } else if input[pos].value.ends_with("\"") {
            break;
        }
        pos += 1;
    }
    string = match string.strip_prefix("\"") {
        Some(r) => r.to_owned(),
        None => String::from(""),
    };
    string = match string.strip_suffix("\"") {
        Some(r) => r.to_owned(),
        None => String::from(""),
    };
    string = string.replace("\\n", "\n");
    string = string.replace("\\\"", "\"");
    string = string.replace("\\\\", "\\");
    Ok((string, pos - start_i))
}

fn match_arguments<T>(
    input: &Vec<GonkASMToken>,
    start_i: usize,
    descriptor: &Descriptor<T>,
) -> Result<(Vec<Argument>, usize), ParseError> {
    let mut arguments: Vec<Argument> = Vec::new();

    let argument_descriptors = descriptor.argument_descriptors;

    let mut pos = start_i;
    let mut argn = 0;
    let mut in_rambracket = false;

    let mut completed = argn == argument_descriptors.len();

    while !completed {
        let token = input[pos].clone();

        if argn >= argument_descriptors.len() {
            return Err(ParseError {
                error_type: ParseErrorType::ArgOverreach,
                description: "Collected more arguments than accepted by the argument descriptor.",
                token: Some(token.clone()),
            });
        }

        let arg_desc = argument_descriptors[argn];

        match token.token_type {
            GonkASMTokenType::Register => {
                if !arg_desc.accept_register {
                    return Err(ParseError {
                        error_type: ParseErrorType::NoRegisters,
                        description: "Argument descriptor doesn't accept registers here.",
                        token: Some(token.clone()),
                    });
                }

                arguments.push(Argument::Register(RegisterArgument {
                    register_name: REGISTER_NAMES
                        .get(&token.value)
                        .cloned()
                        .expect("Couldn't find register"),
                    treat_as_address: in_rambracket,
                }));
                argn += 1;
                in_rambracket = false;
            }

            GonkASMTokenType::Identifier => {
                if !arg_desc.accept_identifier {
                    return Err(ParseError {
                        error_type: ParseErrorType::NoIdentifiers,
                        description: "Argument descriptor doesn't accept identifiers here.",
                        token: Some(token.clone()),
                    });
                }

                arguments.push(Argument::Label(LabelArgument {
                    identifier: token.value.clone(),
                    treat_as_address: in_rambracket,
                }));
                argn += 1;
                in_rambracket = false;
            }

            GonkASMTokenType::ImmediateLiteral => {
                if !arg_desc.accept_immediate {
                    return Err(ParseError {
                        error_type: ParseErrorType::NoImmediates,
                        description: "Argument descriptor doesn't accept immediates here.",
                        token: Some(token.clone()),
                    });
                }

                let value = &token.value;
                let immediate = match value.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(ParseError {
                            error_type: ParseErrorType::FailedImmediateParse,
                            description: "Failed to parse immediate value.",
                            token: Some(token.clone()),
                        });
                    }
                };

                if immediate > (u8::MAX as u16) && !arg_desc.accept_word {
                    return Err(ParseError {
                        error_type: ParseErrorType::NoWords,
                        description: "Argument descriptor doesn't accept words (immediates over 255).",
                        token: Some(token.clone()),
                    });
                }

                arguments.push(Argument::Immediate(ImmediateArgument {
                    value: immediate,
                    treat_as_address: in_rambracket,
                }));
                argn += 1;
                in_rambracket = false;
            }

            GonkASMTokenType::StringLiteral => {
                if !arg_desc.accept_string {
                    return Err(ParseError {
                        error_type: ParseErrorType::NoStrings,
                        description: "Argument descriptor doesn't accept strings.",
                        token: Some(token.clone()),
                    });
                }

                if in_rambracket {
                    return Err(ParseError {
                        error_type: ParseErrorType::FailedRamAddress,
                        description: "Strings can't be used as RAM addresses.",
                        token: Some(token.clone()),
                    });
                }

                let string: String = match parse_string(input, pos) {
                    Ok((s, i)) => {
                        pos += i;
                        s
                    }
                    Err(err) => {
                        return Err(err);
                    }
                };

                arguments.push(Argument::StringLiteral(StringArgument { value: string }));
                argn += 1;
            }

            GonkASMTokenType::RamBracket => {
                if !arg_desc.accept_ram {
                    return Err(ParseError {
                        error_type: ParseErrorType::NoRamAddresses,
                        description: "Argument descriptor doesn't accept RAM addresses.",
                        token: Some(token.clone()),
                    });
                }

                if !in_rambracket {
                    in_rambracket = true;
                } else {
                    return Err(ParseError {
                        error_type: ParseErrorType::FailedRamAddress,
                        description: "Chained RAM addresses aren't allowed.",
                        token: Some(token.clone()),
                    });
                }
            }

            _ => {
                return Err(ParseError {
                    error_type: ParseErrorType::BadTokenType,
                    description: "Bad input token type for arguments.",
                    token: Some(token.clone()),
                });
            }
        }

        pos += 1;

        completed = argn == argument_descriptors.len() && !in_rambracket;
    }

    Ok((arguments, pos))
}

macro_rules! argtype {
    ($val: expr, $arm: ident) => {
        match $val {
            Argument::$arm(r) => r,
            _ => {
                let a = $val;
                return Err(ParseError {
                    error_type: ParseErrorType::BadArgumentType,
                    description: "Incorrectly assumed argument type",
                    token: None,
                });
            }
        }
    };
}

fn parse_command(input: &Vec<GonkASMToken>, i: usize) -> Result<(LayoutObject, usize), ParseError> {
    let descriptor = COMMAND_TYPES
        .get(&input[i].value)
        .expect("nonexistant command name");

    let (arguments, i) = match match_arguments(input, i + 1, descriptor) {
        Ok(result) => result,
        Err(error) => {
            return Err(error);
        }
    };

    let layout_object: LayoutObject = match descriptor.key_type {
        CommandType::DByte => LayoutObject {
            size: 1,
            defaults: vec![0],
        },
        CommandType::DBytes => LayoutObject {
            size: argtype!(&arguments[0], Immediate).value,
            defaults: vec![0; argtype!(&arguments[0], Immediate).value as usize],
        },
        CommandType::IByte => LayoutObject {
            size: 1,
            defaults: vec![argtype!(&arguments[0], Immediate).value as u8],
        },
        CommandType::IBytes => LayoutObject {
            size: argtype!(&arguments[0], Immediate).value,
            defaults: vec![
                argtype!(&arguments[1], Immediate).value as u8;
                argtype!(&arguments[0], Immediate).value as usize
            ],
        },

        CommandType::IStr => {
            let mut string = argtype!(&arguments[0], StringLiteral).value.to_owned();
            string.push('\0');
            LayoutObject {
                size: string.len() as u16,
                defaults: string.as_bytes().to_vec(),
            }
        }
        CommandType::IStrN => {
            let len = argtype!(&arguments[0], Immediate).value;
            let mut string = argtype!(&arguments[1], StringLiteral).value.to_owned();
            string.push('\0');
            LayoutObject {
                size: len,
                defaults: string.as_bytes().to_vec(),
            }
        }

        CommandType::DWord => LayoutObject {
            size: 2,
            defaults: vec![0],
        },
        CommandType::DWords => LayoutObject {
            size: argtype!(&arguments[0], Immediate).value,
            defaults: vec![0; argtype!(&arguments[0], Immediate).value as usize],
        },
        CommandType::IWord => {
            let arg = argtype!(&arguments[0], Immediate);
            let val = vec![arg.value as u8, (arg.value >> 8) as u8];
            LayoutObject {
                size: 2,
                defaults: val,
            }
        }
        CommandType::IWords => {
            let len = argtype!(&arguments[0], Immediate).value;
            let arg = argtype!(&arguments[0], Immediate);
            let mut val = Vec::new();
            for i in (0..len) {
                val.append(&mut vec![(arg.value % 256) as u8, (arg.value >> 8) as u8]);
            }
            LayoutObject {
                size: len * 2,
                defaults: val,
            }
        }
    };

    Ok((layout_object, i))
}

fn parse_instruction(
    input: &Vec<GonkASMToken>,
    i: usize,
) -> Result<(Instruction, usize), ParseError> {
    let descriptor = INSTRUCTION_TYPES
        .get(&input[i].value)
        .expect("nonexistant instruction name");

    let (arguments, i) = match match_arguments(input, i + 1, descriptor) {
        Ok(result) => result,
        Err(error) => {
            return Err(error);
        }
    };

    Ok((
        Instruction {
            instruction_type: descriptor.key_type.clone(),
            arguments,
        },
        i,
    ))
}

struct TemplateGonkASMToken {
    value: &'static str,
    token_type: GonkASMTokenType,
}

struct MacroDefinition {
    placeholder: &'static str,
    placeholder_type: GonkASMTokenType,
    result: &'static [&'static TemplateGonkASMToken],
}

const MACRO_DEFINITIONS: phf::Map<&'static str, MacroDefinition> = phf_map! {
    "$PRINT" => MacroDefinition {
        placeholder: "V",
        placeholder_type: GonkASMTokenType::Identifier,
        result: &[
            // prepare cursor in string
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction
            },
            &TemplateGonkASMToken {
                value: "V",
                token_type: GonkASMTokenType::Identifier
            },
            &TemplateGonkASMToken {
                value: "bill",
                token_type: GonkASMTokenType::Register
            },
            // define top of loop
            &TemplateGonkASMToken {
                value: "label",
                token_type: GonkASMTokenType::Label
            },
            &TemplateGonkASMToken {
                value: "__print_+T",
                token_type: GonkASMTokenType::Identifier
            },
            // check if at end of string
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction
            },
            &TemplateGonkASMToken {
                value: "*",
                token_type: GonkASMTokenType::RamBracket
            },
            &TemplateGonkASMToken {
                value: "bill",
                token_type: GonkASMTokenType::Register
            },
            &TemplateGonkASMToken {
                value: "charlie_l",
                token_type: GonkASMTokenType::Register
            },
            &TemplateGonkASMToken {
                value: "comp",
                token_type: GonkASMTokenType::Instruction
            },
            &TemplateGonkASMToken {
                value: "charlie_l",
                token_type: GonkASMTokenType::Register
            },
            &TemplateGonkASMToken {
                value: "0",
                token_type: GonkASMTokenType::ImmediateLiteral
            },
            // escape if at end of string
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction
            },
            &TemplateGonkASMToken {
                value: "__print_exit_+T",
                token_type: GonkASMTokenType::Identifier
            },
            &TemplateGonkASMToken {
                value: "microwave",
                token_type: GonkASMTokenType::Register
            },
            &TemplateGonkASMToken {
                value: "jumpe",
                token_type: GonkASMTokenType::Instruction
            },
            // write current byte to output
            &TemplateGonkASMToken {
                value: "$WRITE",
                token_type: GonkASMTokenType::Macro
            },
            &TemplateGonkASMToken {
                value: "bill",
                token_type: GonkASMTokenType::Register
            },
            // increment cursor
            &TemplateGonkASMToken {
                value: "inc",
                token_type: GonkASMTokenType::Instruction
            },
            &TemplateGonkASMToken {
                value: "bill",
                token_type: GonkASMTokenType::Register
            },
            // next iteration of loop
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction
            },
            &TemplateGonkASMToken {
                value: "__print_+T",
                token_type: GonkASMTokenType::Identifier
            },
            &TemplateGonkASMToken {
                value: "microwave",
                token_type: GonkASMTokenType::Register
            },
            &TemplateGonkASMToken {
                value: "jump",
                token_type: GonkASMTokenType::Instruction
            },
            // define endpoint
            &TemplateGonkASMToken {
                value: "label",
                token_type: GonkASMTokenType::Label
            },
            &TemplateGonkASMToken {
                value: "__print_exit_+T",
                token_type: GonkASMTokenType::Identifier
            },
        ]
    },
    "$WRITE" => MacroDefinition {
        placeholder: "V",
        placeholder_type: GonkASMTokenType::Register,
        result: &[
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction,
            },
            &TemplateGonkASMToken {
                value: "0",
                token_type: GonkASMTokenType::ImmediateLiteral,
            },
            &TemplateGonkASMToken {
                value: "charlie",
                token_type: GonkASMTokenType::Register,
            },
            // set top of loop
            &TemplateGonkASMToken {
                value: "label",
                token_type: GonkASMTokenType::Label,
            },
            &TemplateGonkASMToken {
                value: "__write_+T",
                token_type: GonkASMTokenType::Identifier,
            },
            // put value at output mapping into charlie
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction,
            },
            &TemplateGonkASMToken {
                value: "*",
                token_type: GonkASMTokenType::RamBracket,
            },
            &TemplateGonkASMToken {
                value: "2",
                token_type: GonkASMTokenType::ImmediateLiteral,
            },
            &TemplateGonkASMToken {
                value: "charlie",
                token_type: GonkASMTokenType::Register,
            },
            // check if lower byte is 0
            &TemplateGonkASMToken {
                value: "comp",
                token_type: GonkASMTokenType::Instruction,
            },
            &TemplateGonkASMToken {
                value: "charlie_l",
                token_type: GonkASMTokenType::Register,
            },
            &TemplateGonkASMToken {
                value: "0",
                token_type: GonkASMTokenType::ImmediateLiteral,
            },
            // jump if not 0
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction,
            },
            &TemplateGonkASMToken {
                value: "__write_+T",
                token_type: GonkASMTokenType::Identifier,
            },
            &TemplateGonkASMToken {
                value: "microwave",
                token_type: GonkASMTokenType::Register,
            },
            &TemplateGonkASMToken {
                value: "jumpne",
                token_type: GonkASMTokenType::Instruction,
            },
            // finally, write the character
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction,
            },
            &TemplateGonkASMToken {
                value: "1",
                token_type: GonkASMTokenType::ImmediateLiteral,
            },
            &TemplateGonkASMToken {
                value: "charlie_l",
                token_type: GonkASMTokenType::Register,
            },
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction,
            },
            &TemplateGonkASMToken {
                value: "*",
                token_type: GonkASMTokenType::RamBracket,
            },
            &TemplateGonkASMToken {
                value: "V",
                token_type: GonkASMTokenType::Register,
            },
            &TemplateGonkASMToken {
                value: "charlie_h",
                token_type: GonkASMTokenType::Register,
            },
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction,
            },
            &TemplateGonkASMToken {
                value: "charlie",
                token_type: GonkASMTokenType::Register,
            },
            &TemplateGonkASMToken {
                value: "*",
                token_type: GonkASMTokenType::RamBracket,
            },
            &TemplateGonkASMToken {
                value: "2",
                token_type: GonkASMTokenType::ImmediateLiteral,
            },
        ],
    },
    "$READ" => MacroDefinition {
        placeholder: "V",
        placeholder_type: GonkASMTokenType::Register,
        result: &[
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction,
            },
            &TemplateGonkASMToken {
                value: "0",
                token_type: GonkASMTokenType::ImmediateLiteral,
            },
            &TemplateGonkASMToken {
                value: "V",
                token_type: GonkASMTokenType::Register,
            },
            // set top of loop
            &TemplateGonkASMToken {
                value: "label",
                token_type: GonkASMTokenType::Label,
            },
            &TemplateGonkASMToken {
                value: "__read_+T",
                token_type: GonkASMTokenType::Identifier,
            },
            // put value at output mapping into charlie
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction,
            },
            &TemplateGonkASMToken {
                value: "*",
                token_type: GonkASMTokenType::RamBracket,
            },
            &TemplateGonkASMToken {
                value: "4",
                token_type: GonkASMTokenType::ImmediateLiteral,
            },
            &TemplateGonkASMToken {
                value: "V",
                token_type: GonkASMTokenType::Register,
            },
            // check if lower byte is 0
            &TemplateGonkASMToken {
                value: "comp",
                token_type: GonkASMTokenType::Instruction,
            },
            &TemplateGonkASMToken {
                value: "V_l",
                token_type: GonkASMTokenType::Register,
            },
            &TemplateGonkASMToken {
                value: "0",
                token_type: GonkASMTokenType::ImmediateLiteral,
            },
            // jump if 0
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction,
            },
            &TemplateGonkASMToken {
                value: "__read_+T",
                token_type: GonkASMTokenType::Identifier,
            },
            &TemplateGonkASMToken {
                value: "microwave",
                token_type: GonkASMTokenType::Register,
            },
            &TemplateGonkASMToken {
                value: "jumpe",
                token_type: GonkASMTokenType::Instruction,
            },
        ],
    }
};

fn expand_macro(tokens: &Vec<GonkASMToken>, index: usize) -> Result<Vec<GonkASMToken>, ParseError> {
    let macro_definition = match MACRO_DEFINITIONS.get(&tokens[index].value) {
        Some(result) => result,
        None => {
            return Err(ParseError {
                error_type: ParseErrorType::UnknownMacro,
                description: "Unknown macro.",
                token: Some(tokens[index].clone()),
            });
        }
    };

    let arg_token = tokens[index + 1].clone();
    let mut new_tokens: Vec<GonkASMToken> = Vec::new();
    for template_token in macro_definition.result {
        let mut token = GonkASMToken {
            value: String::from(template_token.value),
            token_type: template_token.token_type,
            line: tokens[index].line,
            range_start: tokens[index].range_start,
            range_end: tokens[index + 1].range_end,
        };
        if template_token.value.contains(macro_definition.placeholder) {
            if (token.token_type == template_token.token_type) {
                token.value = template_token
                    .value
                    .replace(macro_definition.placeholder, &arg_token.value);
                token.token_type = macro_definition.placeholder_type;
            } else {
                return Err(ParseError {
                    error_type: ParseErrorType::BadTokenType,
                    description: "Incorrect argument type passed to macro.",
                    token: Some(arg_token),
                });
            }
        }
        token.value = token.value.replace("+T", &format!("TEMP_ON_{index}"));
        new_tokens.push(token);
    }

    Ok(new_tokens)
}

pub fn expand_macros(tokens: Vec<GonkASMToken>) -> Result<Vec<GonkASMToken>, ParseError> {
    let mut expanded_tokens: Vec<GonkASMToken> = Vec::new();
    let mut changed = false;
    let mut token_index = 0;
    while token_index < tokens.len() {
        let token = &tokens[token_index];
        if matches!(token.token_type, GonkASMTokenType::Macro) {
            let mut new_tokens = match expand_macro(&tokens, token_index) {
                Ok(result) => result,
                Err(err) => {
                    return Err(err);
                }
            };
            expanded_tokens.append(&mut new_tokens);
            changed = true;
            token_index += 1;
        } else {
            expanded_tokens.push(token.clone());
        }
        token_index += 1;
    }
    if changed {
        expanded_tokens = match expand_macros(expanded_tokens) {
            Ok(result) => result,
            Err(err) => {
                return Err(err);
            }
        };
    }
    Ok(expanded_tokens)
}

#[derive(Debug)]
pub struct ProgramMap {
    layout_objects: Vec<(LayoutObject, usize)>,
    instructions: Vec<(Instruction, usize)>,
    label_definitions: Vec<(LabelDefinition, usize)>,
}

impl ProgramMap {
    pub fn build(input: &Vec<GonkASMToken>) -> Result<ProgramMap, ParseError> {
        let mut program_map = ProgramMap {
            layout_objects: Vec::new(),
            instructions: Vec::new(),
            label_definitions: Vec::new(),
        };

        let mut i = 0;
        while i < input.len() {
            match &input[i].token_type {
                GonkASMTokenType::Command => {
                    let (layout_object, last_consumed_i) = match parse_command(&input, i) {
                        Ok(result) => result,
                        Err(err) => {
                            return Err(err);
                        }
                    };
                    program_map.layout_objects.push((layout_object, i));
                    i = last_consumed_i;
                }

                GonkASMTokenType::Instruction => {
                    let (instruction, last_consumed_i) = match parse_instruction(&input, i) {
                        Ok(result) => result,
                        Err(err) => {
                            return Err(err);
                        }
                    };
                    program_map.instructions.push((instruction, i));
                    i = last_consumed_i;
                }

                GonkASMTokenType::Label => {
                    if i > input.len() - 2 {
                        return Err(ParseError {
                            error_type: ParseErrorType::UnnamedLabel,
                            description: "Label has no identifier following it.",
                            token: Some(input[i].clone()),
                        });
                    }

                    match input[i + 1].token_type {
                        GonkASMTokenType::Identifier => {}
                        _ => {
                            return Err(ParseError {
                                error_type: ParseErrorType::BadTokenType,
                                description: "Label is followed by a non-identifier.",
                                token: Some(input[i + 1].clone()),
                            });
                        }
                    };

                    program_map.label_definitions.push((
                        LabelDefinition {
                            identifier: input[i + 1].value.clone(),
                        },
                        i + 2,
                    ));
                    i += 2;
                }

                _ => {
                    return Err(ParseError {
                        error_type: ParseErrorType::BadTokenType,
                        description: "Bad input token type.",
                        token: Some(input[i].clone()),
                    });
                }
            }
        }

        Ok(program_map)
    }
}

#[derive(Clone, Debug)]
enum LabelAttachment {
    Instruction(usize),
    LayoutObject(usize),
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct LinkedProgramMap {
    labels: HashMap<String, LabelAttachment>,
    instructions: Vec<(Instruction, usize)>,
    layout_objects: Vec<(LayoutObject, usize)>,
}

impl LinkedProgramMap {
    pub fn build(
        program_map: ProgramMap,
        tokens: &Vec<GonkASMToken>,
    ) -> Result<LinkedProgramMap, ParseError> {
        let labels = &program_map.label_definitions;
        let duplicated =
            (1..labels.len()).position(|i| labels[i..].iter().any(|l| l.0 == labels[i - 1].0));
        if duplicated.is_some() {
            return Err(ParseError {
                error_type: ParseErrorType::ReusedLabel,
                description: "The same label is created multiple times.",
                token: Some(tokens[duplicated.unwrap()].clone()),
            });
        }

        let mut label_map: HashMap<String, LabelAttachment> = HashMap::new();

        for label in labels {
            let identifier = &label.0.identifier;
            let token_index = label.1;

            let layout_object = program_map
                .layout_objects
                .iter()
                .position(|ins| ins.1 == token_index);

            let instruction = program_map
                .instructions
                .iter()
                .position(|ins| ins.1 == token_index);

            if layout_object.is_some() {
                label_map.insert(
                    identifier.to_owned(),
                    LabelAttachment::LayoutObject(layout_object.unwrap()),
                );
            } else if instruction.is_some() {
                label_map.insert(
                    identifier.to_owned(),
                    LabelAttachment::Instruction(instruction.unwrap()),
                );
            }
        }

        Ok(LinkedProgramMap {
            labels: label_map,
            instructions: program_map.instructions,
            layout_objects: program_map.layout_objects,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArgFormatFlags(u8);

impl ArgFormatFlags {
    const FLAG_RAM_ADDRESS: ArgFormatFlags = ArgFormatFlags(0b0001);
    const FLAG_BYTE: ArgFormatFlags = ArgFormatFlags(0b0010);
    const FLAG_REGISTER: ArgFormatFlags = ArgFormatFlags(0b0100);
    const FLAG_EXISTS: ArgFormatFlags = ArgFormatFlags(0b1000);

    fn empty() -> Self {
        ArgFormatFlags(0)
    }

    fn mask() -> Self {
        ArgFormatFlags(0b00001111)
    }

    fn bits(&self) -> u8 {
        self.0
    }

    fn is_ram_address(self) -> bool {
        self & Self::FLAG_RAM_ADDRESS == Self::FLAG_RAM_ADDRESS
    }

    fn set_ram_address(&mut self) {
        self.0 = self.0 | Self::FLAG_RAM_ADDRESS.0;
    }

    fn is_byte(self) -> bool {
        self & Self::FLAG_BYTE == Self::FLAG_BYTE
    }

    fn set_byte(&mut self) {
        self.0 = self.0 | Self::FLAG_BYTE.0;
    }

    fn is_register(self) -> bool {
        self & Self::FLAG_REGISTER == Self::FLAG_REGISTER
    }

    fn set_register(&mut self) {
        self.0 = self.0 | Self::FLAG_REGISTER.0;
    }

    fn exists(self) -> bool {
        self & Self::FLAG_EXISTS == Self::FLAG_EXISTS
    }

    fn set_exists(&mut self) {
        self.0 = self.0 | Self::FLAG_EXISTS.0;
    }
}

impl std::ops::BitAnd for ArgFormatFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        ArgFormatFlags(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for ArgFormatFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        ArgFormatFlags(self.0 | rhs.0)
    }
}

impl std::ops::Not for ArgFormatFlags {
    type Output = Self;
    fn not(self) -> Self::Output {
        ArgFormatFlags(!self.0 & 0b00001111)
    }
}

pub fn instruction_to_bytes(
    instruction: &Instruction,
    tokens: &Vec<GonkASMToken>,
    token_index: usize,
) -> Result<(Vec<u8>, Vec<(String, u16)>), ParseError> {
    let instruction_byte: u8 = instruction.instruction_type.clone() as u8;

    let mut argument_format_byte: u8 = 0;
    let mut argument_bytes: Vec<u8> = Vec::new();
    let mut label_bytes: Vec<(String, u16)> = Vec::new();

    if (instruction.arguments.len() == 1) {
        let arg = &instruction.arguments[0];
        let reg = match arg {
            Argument::Register(reg) if !reg.treat_as_address => reg,
            _ => {
                return Err(ParseError {
                    error_type: ParseErrorType::RegisterNeeded,
                    description: "Single argument must be a register.",
                    token: Some(tokens[token_index].clone()),
                });
            }
        };

        let mut arg_format = ArgFormatFlags::empty();
        arg_format.set_exists();
        arg_format.set_register();
        if reg.treat_as_address {
            arg_format.set_ram_address();
        }

        argument_format_byte = arg_format.0;

        argument_bytes.push(reg.register_name.clone() as u8);
    } else if (instruction.arguments.len() == 2) {
        let token = Some(tokens[token_index].clone());
        let arg1 = &instruction.arguments[0];
        let arg2 = &instruction.arguments[1];

        let (src_pointer, src_immediate, src_byte, src_negotiable) = match arg1 {
            Argument::Register(reg) => (
                reg.treat_as_address,
                false,
                is_register_byte(&reg.register_name),
                false,
            ),
            Argument::Immediate(imm) => (imm.treat_as_address, true, false, imm.get_value() < 256),
            Argument::Label(label) => (label.treat_as_address, true, false, false),
            _ => panic!("String literals can't be used as arguments."),
        };
        let src_register = !src_immediate;

        let (dest_pointer, dest_immediate, dest_byte, dest_negotiable) = match arg2 {
            Argument::Register(reg) => (
                reg.treat_as_address,
                false,
                is_register_byte(&reg.register_name),
                false,
            ),
            Argument::Immediate(imm) => (imm.treat_as_address, true, false, imm.get_value() < 256),
            Argument::Label(label) => (label.treat_as_address, true, false, false),
            _ => panic!("String literals can't be used as arguments."),
        };
        let dest_register = !dest_immediate;

        let mut dest_decides = true;
        if (!matches!(instruction.instruction_type, InstructionType::Comp)) {
            // possible argument formats
            // imm imm - cant use imm as dest
            // reg imm - cant use imm as dest
            // ram imm - cant use imm as dest
            //
            // imm reg - use dest as size, failure if dest is byte and imm > 255
            // reg reg - use dest as size, failure if reg mismatch
            // ram reg - use dest as size, no failure
            //
            // imm ram - cant put imm in ram
            // reg ram - use src as size, no failure
            // ram ram - cant put ram in ram

            if (dest_immediate && !dest_pointer) {
                return Err(ParseError {
                    error_type: ParseErrorType::NoImmediates,
                    description: "Immediates can't be used as a destination.",
                    token,
                });
            }

            if (src_pointer && dest_pointer) {
                return Err(ParseError {
                    error_type: ParseErrorType::RegisterNeeded,
                    description: "RAM can't be used as a source if the destination is RAM (sizes are ambiguous).",
                    token,
                });
            }

            if (src_immediate && dest_pointer) {
                return Err(ParseError {
                    error_type: ParseErrorType::RegisterNeeded,
                    description: "Immediates can't be used as a source if the destination is RAM (sizes are ambiguous).",
                    token,
                });
            }

            dest_decides = !dest_pointer;

            // dest is byte and imm > 255
            if ((dest_decides && dest_byte) && (src_immediate && !src_pointer && !src_negotiable)) {
                return Err(ParseError {
                    error_type: ParseErrorType::SizeMismatch,
                    description: "Sizes of arguments must be equal.",
                    token,
                });
            }

            // register mismatch
            if (dest_decides && (src_register && !src_pointer) && src_byte != dest_byte) {
                return Err(ParseError {
                    error_type: ParseErrorType::SizeMismatch,
                    description: "Sizes of arguments must be equal.",
                    token,
                });
            }
        } else {
            if (dest_pointer || src_pointer) {
                return Err(ParseError {
                    error_type: ParseErrorType::NoRamAddresses,
                    description: "Pointers aren't allowed in Comp instructions.",
                    token,
                });
            }

            if (dest_immediate && src_immediate) {
                return Err(ParseError {
                    error_type: ParseErrorType::RegisterNeeded,
                    description: "At least one argument must be a register in a Comp instruction.",
                    token,
                });
            }

            if ((dest_register && src_register) && (dest_byte != src_byte)) {
                return Err(ParseError {
                    error_type: ParseErrorType::SizeMismatch,
                    description: "Sizes of arguments must be equal.",
                    token,
                });
            }

            if ((dest_register && src_immediate) && (dest_byte && !src_byte && !src_negotiable)) {
                return Err(ParseError {
                    error_type: ParseErrorType::SizeMismatch,
                    description: "Sizes of arguments must be equal.",
                    token,
                });
            }

            if ((src_register && dest_immediate) && (src_byte && !dest_byte && !dest_negotiable)) {
                return Err(ParseError {
                    error_type: ParseErrorType::SizeMismatch,
                    description: "Sizes of arguments must be equal.",
                    token,
                });
            }
        }

        let byte = if dest_decides { dest_byte } else { src_byte };
        let mut src_arg_format = ArgFormatFlags::empty();
        let mut dest_arg_format = ArgFormatFlags::empty();
        src_arg_format.set_exists();
        dest_arg_format.set_exists();

        let mut offset = 2;
        if src_register {
            src_arg_format.set_register();
            argument_bytes.push(argtype!(arg1, Register).register_name.clone() as u8);
            offset += 1;
            if byte {
                src_arg_format.set_byte();
            }
            if src_pointer {
                src_arg_format.set_ram_address();
            }
        } else {
            if let Argument::Immediate(imm) = arg1 {
                if src_pointer {
                    let bytes = imm.value.to_le_bytes();
                    argument_bytes.push(bytes[0]);
                    argument_bytes.push(bytes[1]);
                    src_arg_format.set_ram_address();
                    if byte {
                        src_arg_format.set_byte();
                    }
                    offset += 2;
                } else {
                    if byte {
                        argument_bytes.push(imm.value as u8);
                        src_arg_format.set_byte();
                        offset += 1;
                    } else {
                        let bytes = imm.value.to_le_bytes();
                        argument_bytes.push(bytes[0]);
                        argument_bytes.push(bytes[1]);
                        offset += 2;
                    }
                }
            } else if let Argument::Label(label) = arg1 {
                argument_bytes.push(0);
                argument_bytes.push(0);
                label_bytes.push((label.identifier.clone(), offset));
                if src_pointer {
                    src_arg_format.set_ram_address();
                    if byte {
                        src_arg_format.set_byte();
                    }
                }
                offset += 2;
            }
        }

        if dest_register {
            dest_arg_format.set_register();
            argument_bytes.push(argtype!(arg2, Register).register_name.clone() as u8);
            offset += 1;
            if byte {
                dest_arg_format.set_byte();
            }
            if dest_pointer {
                dest_arg_format.set_ram_address();
            }
        } else {
            if let Argument::Immediate(imm) = arg2 {
                if dest_pointer {
                    let bytes = imm.value.to_le_bytes();
                    argument_bytes.push(bytes[0]);
                    argument_bytes.push(bytes[1]);
                    dest_arg_format.set_ram_address();
                    if byte {
                        dest_arg_format.set_byte();
                    }
                    offset += 2;
                } else {
                    if byte {
                        argument_bytes.push(imm.value as u8);
                        dest_arg_format.set_byte();
                        offset += 1;
                    } else {
                        let bytes = imm.value.to_le_bytes();
                        argument_bytes.push(bytes[0]);
                        argument_bytes.push(bytes[1]);
                        offset += 2;
                    }
                }
            } else if let Argument::Label(label) = arg2 {
                argument_bytes.push(0);
                argument_bytes.push(0);
                label_bytes.push((label.identifier.clone(), offset));
                if dest_pointer {
                    dest_arg_format.set_ram_address();
                    if byte {
                        dest_arg_format.set_byte();
                    }
                }
                offset += 2;
            }
        }

        argument_format_byte = (src_arg_format.0) + (dest_arg_format.0 << 4);
    }

    let mut bytes = vec![];
    bytes.push(instruction_byte);
    bytes.push(argument_format_byte);
    bytes.append(&mut argument_bytes);

    Ok((bytes, label_bytes))
}

#[derive(Debug)]
pub enum ByteArgumentType {
    Register(RegisterName),
    Immediate(u16),
}

#[derive(Debug)]
pub struct ByteArgument {
    argument_type: ByteArgumentType,
    byte: bool,
    address: bool,
}

impl ByteArgument {
    pub fn get_argument_type(&self) -> &ByteArgumentType {
        &self.argument_type
    }

    pub fn is_byte(&self) -> bool {
        self.byte
    }

    pub fn is_address(&self) -> bool {
        self.address
    }
}

#[derive(Debug)]
pub struct ByteInstruction {
    instruction_type: InstructionType,
    arguments: Vec<ByteArgument>,
}

impl ByteInstruction {
    pub fn get_instruction_type(&self) -> &InstructionType {
        &self.instruction_type
    }

    pub fn get_arguments(&self) -> &Vec<ByteArgument> {
        &self.arguments
    }
}

pub fn bytes_to_instruction(
    bytes: &[u8; 0x1000],
    index: u16,
) -> Result<ByteInstruction, ParseError> {
    let instruction_byte = bytes[index as usize];
    let instruction_type = InstructionType::from_repr(instruction_byte)
        .expect("it's too late for me to handle errors. instruction type doesn't exist.");

    let mut arguments: Vec<ByteArgument> = Vec::new();
    let arg_format_byte: u8 = bytes[index as usize + 1];
    let arg_format1 = ArgFormatFlags(arg_format_byte) & ArgFormatFlags::mask();
    let arg_format2 = ArgFormatFlags(arg_format_byte >> 4) & ArgFormatFlags::mask();
    let mut offset = 2;

    if arg_format1.exists() {
        if arg_format1.is_register() {
            let name = RegisterName::from_repr(bytes[index as usize + offset]);
            arguments.push(ByteArgument {
                argument_type: ByteArgumentType::Register(
                    name.expect("Tried to parse unknown register name"),
                ),
                byte: arg_format1.is_byte(),
                address: arg_format1.is_ram_address(),
            });
            offset += 1;
        } else {
            if arg_format1.is_ram_address() {
                let value = u16::from_le_bytes([
                    bytes[index as usize + offset],
                    bytes[index as usize + offset + 1],
                ]);
                arguments.push(ByteArgument {
                    argument_type: ByteArgumentType::Immediate(value),
                    byte: arg_format1.is_byte(),
                    address: true,
                });
                offset += 2;
            } else {
                if arg_format1.is_byte() {
                    let value = bytes[index as usize + offset] as u16;
                    arguments.push(ByteArgument {
                        argument_type: ByteArgumentType::Immediate(value),
                        byte: true,
                        address: false,
                    });
                    offset += 1;
                } else {
                    let value = u16::from_le_bytes([
                        bytes[index as usize + offset],
                        bytes[index as usize + offset + 1],
                    ]);
                    arguments.push(ByteArgument {
                        argument_type: ByteArgumentType::Immediate(value),
                        byte: false,
                        address: false,
                    });
                    offset += 2;
                }
            }
        }
    }

    if arg_format2.exists() {
        if arg_format2.is_register() {
            let name = RegisterName::from_repr(bytes[index as usize + offset]);
            arguments.push(ByteArgument {
                argument_type: ByteArgumentType::Register(
                    name.expect("Tried to parse unknown register name"),
                ),
                byte: arg_format2.is_byte(),
                address: arg_format2.is_ram_address(),
            });
            offset += 1;
        } else {
            if arg_format2.is_ram_address() {
                let value = u16::from_le_bytes([
                    bytes[index as usize + offset],
                    bytes[index as usize + offset + 1],
                ]);
                arguments.push(ByteArgument {
                    argument_type: ByteArgumentType::Immediate(value),
                    byte: arg_format2.is_byte(),
                    address: true,
                });
                offset += 2;
            } else {
                if arg_format2.is_byte() {
                    let value = bytes[index as usize + offset] as u16;
                    arguments.push(ByteArgument {
                        argument_type: ByteArgumentType::Immediate(value),
                        byte: true,
                        address: false,
                    });
                    offset += 1;
                } else {
                    let value = u16::from_le_bytes([
                        bytes[index as usize + offset],
                        bytes[index as usize + offset + 1],
                    ]);
                    arguments.push(ByteArgument {
                        argument_type: ByteArgumentType::Immediate(value),
                        byte: false,
                        address: false,
                    });
                    offset += 2;
                }
            }
        }
    }

    Ok(ByteInstruction {
        instruction_type,
        arguments,
    })
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct BinaryTokenMapping {
    start: u16,
    end: u16,
    tokens: Vec<GonkASMToken>,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct ProgramBinary {
    binary: [u8; 0x1000],
    binary_token_map: Vec<BinaryTokenMapping>,
}

impl ProgramBinary {
    pub fn new(binary: [u8; 0x1000], binary_token_map: Vec<BinaryTokenMapping>) -> ProgramBinary {
        ProgramBinary {
            binary,
            binary_token_map,
        }
    }

    pub fn get_binary(&self) -> &[u8; 0x1000] {
        &self.binary
    }
}

#[wasm_bindgen]
impl ProgramBinary {
    pub fn get_binary_token_map(&self) -> Vec<BinaryTokenMapping> {
        self.binary_token_map.to_vec()
    }

    pub fn get_start_byte(&self) -> u16 {
        u16::from_le_bytes([self.binary[0], self.binary[1]])
    }

    pub fn get_binary_blob(&self) -> Box<[u8]> {
        Box::new(self.binary)
    }
}

impl ProgramBinary {
    pub fn build(
        linked_program_map: LinkedProgramMap,
        tokens: &Vec<GonkASMToken>,
    ) -> Result<ProgramBinary, ParseError> {
        let mut binary: [u8; 0x1000] = [0; 0x1000];
        let mut binary_token_map: Vec<BinaryTokenMapping> = Vec::new();

        let mut label_map: HashMap<String, u16> = HashMap::new();

        let mut label_replacement_queue: Vec<(String, u16)> = Vec::new();

        let mut layout_pos: u16 = 6;
        for i in (0..linked_program_map.layout_objects.len()) {
            let (layout_object, token_index) = &linked_program_map.layout_objects[i];
            binary_token_map.push(BinaryTokenMapping {
                start: layout_pos,
                end: layout_pos + layout_object.size,
                tokens: vec![tokens[*token_index].clone()],
            });
            for label in &linked_program_map.labels {
                let label_index = match label.1 {
                    LabelAttachment::Instruction(_) => {
                        continue;
                    }
                    LabelAttachment::LayoutObject(result) => result,
                };
                if i == *label_index {
                    label_map.insert(label.0.clone(), layout_pos);
                }
            }
            let defaults = &layout_object.defaults;
            for i in (0..layout_object.size as usize) {
                if i < defaults.len() {
                    binary[i + layout_pos as usize] = defaults[i];
                } else {
                    binary[i + layout_pos as usize] = 0;
                }
            }
            layout_pos += layout_object.size;
        }

        for i in (0..linked_program_map.instructions.len()) {
            let (instruction, token_index) = &linked_program_map.instructions[i];
            for label in &linked_program_map.labels {
                let label_index = match label.1 {
                    LabelAttachment::LayoutObject(_) => {
                        continue;
                    }
                    LabelAttachment::Instruction(result) => result,
                };
                if i == *label_index {
                    label_map.insert(label.0.clone(), layout_pos);
                }
            }
            let (bytes, needed_labels) =
                match instruction_to_bytes(&instruction, tokens, *token_index) {
                    Ok(result) => result,
                    Err(err) => {
                        return Err(err);
                    }
                };
            for i in (0..bytes.len()) {
                binary[layout_pos as usize + i] = bytes[i];
            }
            for i in needed_labels {
                label_replacement_queue.push((i.0, i.1 + layout_pos));
            }
            let mut mapped_tokens = vec![tokens[*token_index].clone()];
            for j in (0..instruction.arguments.len()) {
                mapped_tokens.push(tokens[*token_index + j].clone());
            }
            binary_token_map.push(BinaryTokenMapping {
                start: layout_pos,
                end: layout_pos + bytes.len() as u16,
                tokens: mapped_tokens,
            });
            layout_pos += bytes.len() as u16;
        }

        for (label_name, byte_index) in label_replacement_queue {
            let label_address = match label_map.get(&label_name) {
                Some(result) => result,
                None => {
                    return Err(ParseError {
                        error_type: ParseErrorType::UnknownLabel,
                        description: "Unknown label in use.",
                        token: Some(
                            binary_token_map
                                .iter()
                                .find(|x| x.tokens.iter().any(|y| y.value == label_name))
                                .unwrap()
                                .tokens[0]
                                .clone(),
                        ),
                    });
                }
            };
            let label_address_slice = label_address.to_le_bytes();
            binary[byte_index as usize] = label_address_slice[0];
            binary[(byte_index + 1) as usize] = label_address_slice[1];
        }

        let start_byte = label_map.get("start");
        let start_byte: u16 = match start_byte {
            Some(result) => *result,
            None => {
                return Err(ParseError {
                    error_type: ParseErrorType::BadStartLabel,
                    description: "Missing start label.",
                    token: None,
                });
            }
        };
        let start_bytes: [u8; 2] = u16::to_le_bytes(start_byte);
        binary[0] = start_bytes[0];
        binary[1] = start_bytes[1];

        Ok(ProgramBinary {
            binary,
            binary_token_map,
        })
    }
}

/*
 * PROGRAM BUILDING
 */
#[wasm_bindgen(js_name = "buildGonkASMProgram")]
pub fn build_gonkbox_program(tokens: Vec<GonkASMToken>) -> Result<ProgramBinary, ParseError> {
    let string = fmt::format(format_args!("Token list: {tokens:#?}"));

    let tokens = match expand_macros(tokens) {
        Ok(result) => result,
        Err(err) => {
            util::log!("{err:#?}");
            return Err(err);
        }
    };

    // multi pass compilation:
    // step 1 - create instructions, layout objects, and loose identifiers
    let program_map: ProgramMap = match ProgramMap::build(&tokens) {
        Ok(result) => result,
        Err(err) => {
            util::log!("{err:#?}");
            return Err(err);
        }
    };

    // step 2 - harden identifiers with pointer to intended layout object or instruction
    let linked_program_map: LinkedProgramMap = match LinkedProgramMap::build(program_map, &tokens) {
        Ok(result) => result,
        Err(err) => {
            util::log!("{err:#?}");
            return Err(err);
        }
    };

    // step 3 - generate binary
    let program_binary = match ProgramBinary::build(linked_program_map, &tokens) {
        Ok(result) => result,
        Err(err) => {
            util::log!("{err:#?}");
            return Err(err);
        }
    };

    Ok(program_binary)
}
