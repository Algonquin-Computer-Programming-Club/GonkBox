#![allow(unused)]

use std::fmt::{self, Debug};

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
#[derive(AsRefStr, Clone, Debug)]
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

#[derive(Debug)]
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
    UnterminatedString,

    BadTokenType,
    BadArgumentType,

    ArgOverreach,
    UnnamedLabel,
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
        let arg_desc = argument_descriptors[argn];

        let token = input[pos].clone();
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

                if !in_rambracket && token.value.eq("[") {
                    in_rambracket = true;
                } else if in_rambracket && token.value.eq("]") {
                    in_rambracket = false;
                } else {
                    return Err(ParseError {
                        error_type: ParseErrorType::FailedRamAddress,
                        description: "RAM address brackets are malformed.",
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

        if argn > argument_descriptors.len() {
            return Err(ParseError {
                error_type: ParseErrorType::ArgOverreach,
                description: "Collected more arguments than accepted by the argument descriptor.",
                token: Some(token.clone()),
            });
        }
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

#[derive(Debug)]
pub struct ProgramMap {
    layout_objects: Vec<(LayoutObject, usize)>,
    instructions: Vec<(Instruction, usize)>,
    label_definitions: Vec<(LabelDefinition, usize)>,
}

impl ProgramMap {
    pub fn build(input: Vec<GonkASMToken>) -> Result<ProgramMap, ParseError> {
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
                    i = last_consumed_i;
                    program_map.layout_objects.push((layout_object, i));
                }

                GonkASMTokenType::Instruction => {
                    let (instruction, last_consumed_i) = match parse_instruction(&input, i) {
                        Ok(result) => result,
                        Err(err) => {
                            return Err(err);
                        }
                    };
                    i = last_consumed_i;
                    program_map.instructions.push((instruction, i));
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

                GonkASMTokenType::Macro => {
                    i += 2;
                    // everyone knows macros arent real
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

/*
 * PROGRAM BUILDING
 */
#[wasm_bindgen]
#[derive(Debug)]
pub struct GonkBoxProgram {
    program_map: ProgramMap,
}

#[wasm_bindgen(js_name = "buildGonkASMProgram")]
pub fn build_gonkbox_program(tokens: Vec<GonkASMToken>) -> Result<GonkBoxProgram, ParseError> {
    let string = fmt::format(format_args!("Token list: {tokens:#?}"));
    log1!("Token list: {tokens:#?}");

    // multi pass compilation:
    // step 1 - create instructions, layout objects, and loose identifiers
    let program_map: ProgramMap = match ProgramMap::build(tokens) {
        Ok(result) => result,
        Err(err) => {
            log1!("{err:#?}");
            return Err(err);
        }
    };

    log1!("Program map: {program_map:#?}");

    // step 2 - harden identifiers with pointer to intended layout object or instruction

    // step 3 - generate binary

    Ok(GonkBoxProgram { program_map })
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn no_tokens_input() {
        let tokens = vec![];
        let program = build_gonkbox_program(tokens);
        match program {
            Ok(result) => {
                log1!("{result:#?}");
            }
            Err(err) => {
                log1!("{err:#?}");
                panic!();
            }
        }
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
        match program {
            Ok(result) => {
                log1!("{result:#?}");
            }
            Err(err) => {
                log1!("{err:#?}");
                panic!();
            }
        }
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
        match program {
            Ok(result) => {
                log1!("{result:#?}");
            }
            Err(err) => {
                log1!("{err:#?}");
                panic!();
            }
        }
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
        match program {
            Ok(result) => {
                log1!("{result:#?}");
            }
            Err(err) => {
                log1!("{err:#?}");
                panic!();
            }
        }
    }
}
