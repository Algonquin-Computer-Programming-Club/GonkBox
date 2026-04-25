#![allow(unused)]

use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

use wasm_bindgen::prelude::*;

use strum_macros::AsRefStr;

use web_sys::console;

use phf::phf_map;

/*
 *  UTILITY
 */
macro_rules! log1 {
    ($a:literal) => {
        let string = fmt::format(format_args!($a));
        if cfg!(target_arch = "wasm32") {
            console::log_1(&string.into());
        } else {
            println!("{}", string);
        }
    };
}

/*
 *  TOKENS
 */
#[wasm_bindgen]
#[derive(AsRefStr, Clone, Copy, Debug)]
pub enum GonkASMTokenType {
    Command,
    Instruction,
    RamBracket,
    Separator,
    Label,
    Register,
    Identifier,
    ImmediateLiteral,
    StringLiteral,
    StringLiteralEscape,
    Macro,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct GonkASMToken {
    value: String,
    token_type: GonkASMTokenType,
}

#[wasm_bindgen]
impl GonkASMToken {
    #[wasm_bindgen(constructor)]
    pub fn new(value: String, token_type: GonkASMTokenType) -> GonkASMToken {
        GonkASMToken { value, token_type }
    }
}

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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum InstructionType {
    // data transfer
    Move,

    // math
    Add,
    Sub,
    Inc,
    Dec,
    Flip,

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
    "inc" => Descriptor {
        key_type: InstructionType::Inc,
        argument_descriptors: &[ARGDESC_GENERAL],
    },
    "dec" => Descriptor {
        key_type: InstructionType::Dec,
        argument_descriptors: &[ARGDESC_GENERAL],
    },
    "flip" => Descriptor {
        key_type: InstructionType::Flip,
        argument_descriptors: &[ARGDESC_REGONLY],
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
#[derive(Debug)]
pub struct RegisterArgument {
    register_name: RegisterName,
    treat_as_address: bool,
}

// e.g. 4 or [0x10]
#[derive(Debug)]
pub struct ImmediateArgument {
    value: u16,
    treat_as_address: bool,
}

// e.g. print or [start]
#[derive(Debug)]
pub struct LabelArgument {
    identifier: String,
    treat_as_address: bool,
}

#[derive(Debug)]
pub struct StringArgument {
    value: String,
}

#[derive(Debug)]
pub enum Argument {
    Register(RegisterArgument),
    Immediate(ImmediateArgument),
    Label(LabelArgument),
    StringLiteral(StringArgument),
}

#[derive(Debug)]
pub struct Instruction {
    instruction_type: InstructionType,
    arguments: Vec<Argument>,
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
    UnnamedLabel,
    UnterminatedString,

    ReusedLabel,

    UnknownMacro,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct ParseError {
    error_type: ParseErrorType,
    description: &'static str,
    token: Option<GonkASMToken>,
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
        log1!("[{argn}]:{{{token:#?}, {arg_desc:#?}}}");

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

                if !in_rambracket && token.value.eq("*") {
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
            defaults: vec![0],
        },
        CommandType::IByte => LayoutObject {
            size: 1,
            defaults: vec![argtype!(&arguments[0], Immediate).value as u8],
        },
        CommandType::IBytes => LayoutObject {
            size: argtype!(&arguments[0], Immediate).value,
            defaults: vec![argtype!(&arguments[1], Immediate).value as u8],
        },

        CommandType::IStr => {
            let string = &argtype!(&arguments[0], StringLiteral).value;
            LayoutObject {
                size: string.len() as u16,
                defaults: string.as_bytes().to_vec(),
            }
        }
        CommandType::IStrN => {
            let len = argtype!(&arguments[0], Immediate).value;
            let string = &argtype!(&arguments[1], StringLiteral).value;
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
            defaults: vec![0],
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
            let val = vec![arg.value as u8, (arg.value >> 8) as u8];
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
                value: ".label",
                token_type: GonkASMTokenType::Label
            },
            &TemplateGonkASMToken {
                value: "__print_+T",
                token_type: GonkASMTokenType::Identifier
            },
            // check if at end of string
            &TemplateGonkASMToken {
                value: "comp",
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
                value: "bill_l",
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
                value: ".label",
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
            // set top of loop
            &TemplateGonkASMToken {
                value: ".label",
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
                value: "0",
                token_type: GonkASMTokenType::ImmediateLiteral,
            },
            &TemplateGonkASMToken {
                value: "charlie",
                token_type: GonkASMTokenType::Register,
            },
            // check if higher byte is 0
            &TemplateGonkASMToken {
                value: "comp",
                token_type: GonkASMTokenType::Instruction,
            },
            &TemplateGonkASMToken {
                value: "charlie_h",
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
                value: "__write_+T",
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
            // finally, write the byte
            &TemplateGonkASMToken {
                value: "move",
                token_type: GonkASMTokenType::Instruction,
            },
            &TemplateGonkASMToken {
                value: "V",
                token_type: GonkASMTokenType::Register,
            },
            &TemplateGonkASMToken {
                value: "*",
                token_type: GonkASMTokenType::RamBracket,
            },
            &TemplateGonkASMToken {
                value: "0",
                token_type: GonkASMTokenType::ImmediateLiteral,
            },
        ],
    },
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

    let mut new_tokens: Vec<GonkASMToken> = Vec::new();
    for template_token in macro_definition.result {
        let mut token = GonkASMToken::new(
            String::from(template_token.value),
            template_token.token_type,
        );
        if template_token.value == macro_definition.placeholder {
            token.value = tokens[index + 1].value.clone();
            token.token_type = macro_definition.placeholder_type;
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

#[derive(Debug)]
enum LabelAttachment {
    Instruction(usize),
    LayoutObject(usize),
}

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

/*
 * PROGRAM BUILDING
 */
#[wasm_bindgen]
#[derive(Debug)]
pub struct GonkBoxProgram {
    linked_program_map: LinkedProgramMap,
}

#[wasm_bindgen(js_name = "buildGonkASMProgram")]
pub fn build_gonkbox_program(tokens: Vec<GonkASMToken>) -> Result<GonkBoxProgram, ParseError> {
    let string = fmt::format(format_args!("Token list: {tokens:#?}"));
    log1!("Token list: {tokens:#?}");

    let tokens = match expand_macros(tokens) {
        Ok(result) => result,
        Err(err) => {
            log1!("{err:#?}");
            return Err(err);
        }
    };

    log1!("Expanded token list: {tokens:#?}");

    // multi pass compilation:
    // step 1 - create instructions, layout objects, and loose identifiers
    let program_map: ProgramMap = match ProgramMap::build(&tokens) {
        Ok(result) => result,
        Err(err) => {
            log1!("{err:#?}");
            return Err(err);
        }
    };

    log1!("Program map: {program_map:#?}");

    // step 2 - harden identifiers with pointer to intended layout object or instruction
    let linked_program_map: LinkedProgramMap = match LinkedProgramMap::build(program_map, &tokens) {
        Ok(result) => result,
        Err(err) => {
            log1!("{err:#?}");
            return Err(err);
        }
    };

    log1!("Linked program map: {linked_program_map:#?}");

    // step 3 - generate binary

    Ok(GonkBoxProgram { linked_program_map })
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn no_tokens_input() {
        let tokens = vec![];
        let program = build_gonkbox_program(tokens);
        assert!(matches!(program, Ok(_)), "{program:#?}");
    }

    #[test]
    fn single_instruction() {
        let tokens = vec![
            GonkASMToken {
                value: String::from("move"),
                token_type: GonkASMTokenType::Instruction,
            },
            GonkASMToken {
                value: String::from("charlie"),
                token_type: GonkASMTokenType::Register,
            },
            GonkASMToken {
                value: String::from("bill"),
                token_type: GonkASMTokenType::Register,
            },
        ];
        let program = build_gonkbox_program(tokens);
        assert!(matches!(program, Ok(_)), "{program:#?}");
    }

    #[test]
    fn multi_instruction() {
        let tokens = vec![
            // command 1
            // label start
            GonkASMToken {
                value: String::from(".label"),
                token_type: GonkASMTokenType::Label,
            },
            GonkASMToken {
                value: String::from("start"),
                token_type: GonkASMTokenType::Identifier,
            },
            // instruction 1
            // insert 4 into bill
            GonkASMToken {
                value: String::from("move"),
                token_type: GonkASMTokenType::Instruction,
            },
            GonkASMToken {
                value: String::from("4"),
                token_type: GonkASMTokenType::ImmediateLiteral,
            },
            GonkASMToken {
                value: String::from("bill"),
                token_type: GonkASMTokenType::Register,
            },
            // instruction 2
            // insert 5 into charlie
            GonkASMToken {
                value: String::from("move"),
                token_type: GonkASMTokenType::Instruction,
            },
            GonkASMToken {
                value: String::from("5"),
                token_type: GonkASMTokenType::ImmediateLiteral,
            },
            GonkASMToken {
                value: String::from("charlie"),
                token_type: GonkASMTokenType::Register,
            },
            // instruction 3
            // compare bill and charlie
            GonkASMToken {
                value: String::from("comp"),
                token_type: GonkASMTokenType::Instruction,
            },
            GonkASMToken {
                value: String::from("bill"),
                token_type: GonkASMTokenType::Register,
            },
            GonkASMToken {
                value: String::from("charlie"),
                token_type: GonkASMTokenType::Register,
            },
            // print bill
            GonkASMToken {
                value: String::from("$PRINT"),
                token_type: GonkASMTokenType::Macro,
            },
            GonkASMToken {
                value: String::from("bill"),
                token_type: GonkASMTokenType::Register,
            },
        ];
        let program = build_gonkbox_program(tokens);
        assert!(matches!(program, Ok(_)), "{program:#?}");
    }

    #[test]
    fn single_command() {
        let tokens = vec![
            GonkASMToken {
                value: String::from("istr"),
                token_type: GonkASMTokenType::Command,
            },
            GonkASMToken {
                value: String::from("\"Hello, world!\""),
                token_type: GonkASMTokenType::StringLiteral,
            },
        ];
        let program = build_gonkbox_program(tokens);
        assert!(matches!(program, Ok(_)), "{program:#?}");
    }

    #[test]
    fn multi_label() {
        let tokens = vec![
            GonkASMToken {
                value: String::from(".label"),
                token_type: GonkASMTokenType::Label,
            },
            GonkASMToken {
                value: String::from("first"),
                token_type: GonkASMTokenType::Identifier,
            },
            GonkASMToken {
                value: String::from("stop"),
                token_type: GonkASMTokenType::Instruction,
            },
            GonkASMToken {
                value: String::from(".label"),
                token_type: GonkASMTokenType::Label,
            },
            GonkASMToken {
                value: String::from("second"),
                token_type: GonkASMTokenType::Identifier,
            },
            GonkASMToken {
                value: String::from("stop"),
                token_type: GonkASMTokenType::Instruction,
            },
        ];
        let program = build_gonkbox_program(tokens);
        assert!(matches!(program, Ok(_)), "{program:#?}");
    }

    #[test]
    fn reused_label() {
        let tokens = vec![
            GonkASMToken {
                value: String::from(".label"),
                token_type: GonkASMTokenType::Label,
            },
            GonkASMToken {
                value: String::from("test"),
                token_type: GonkASMTokenType::Identifier,
            },
            GonkASMToken {
                value: String::from("stop"),
                token_type: GonkASMTokenType::Instruction,
            },
            GonkASMToken {
                value: String::from(".label"),
                token_type: GonkASMTokenType::Label,
            },
            GonkASMToken {
                value: String::from("test"),
                token_type: GonkASMTokenType::Identifier,
            },
            GonkASMToken {
                value: String::from("stop"),
                token_type: GonkASMTokenType::Instruction,
            },
        ];
        let program = build_gonkbox_program(tokens);
        assert!(
            matches!(&program, Err(err) if matches!(err.error_type, ParseErrorType::ReusedLabel)),
            "{program:#?}"
        );
    }

    #[test]
    fn detached_label() {
        let tokens = vec![
            GonkASMToken {
                value: String::from(".label"),
                token_type: GonkASMTokenType::Label,
            },
            GonkASMToken {
                value: String::from("test"),
                token_type: GonkASMTokenType::Identifier,
            },
        ];
        let program = build_gonkbox_program(tokens);
        assert!(matches!(&program, Ok(_)), "{program:#?}");
    }

    #[test]
    fn print_macro() {
        let tokens = vec![
            GonkASMToken {
                value: String::from(".label"),
                token_type: GonkASMTokenType::Label,
            },
            GonkASMToken {
                value: String::from("msg"),
                token_type: GonkASMTokenType::Identifier,
            },
            GonkASMToken {
                value: String::from("istr"),
                token_type: GonkASMTokenType::Command,
            },
            GonkASMToken {
                value: String::from("\"Hello, world!\""),
                token_type: GonkASMTokenType::StringLiteral,
            },
            GonkASMToken {
                value: String::from(".label"),
                token_type: GonkASMTokenType::Label,
            },
            GonkASMToken {
                value: String::from("start"),
                token_type: GonkASMTokenType::Identifier,
            },
            GonkASMToken {
                value: String::from("$PRINT"),
                token_type: GonkASMTokenType::Macro,
            },
            GonkASMToken {
                value: String::from("msg"),
                token_type: GonkASMTokenType::Identifier,
            },
        ];
        let program = build_gonkbox_program(tokens);
        assert!(matches!(&program, Ok(_)), "{program:#?}");
    }
}
