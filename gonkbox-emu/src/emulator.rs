#![allow(unused)]

use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::ArrayBuffer;

use crate::asm_parser::{
    Argument, ByteArgument, ByteArgumentType, Instruction, InstructionType, ParseError,
    ProgramBinary, RegisterName, build_gonkbox_program, bytes_to_instruction,
};

use crate::tokenizer::Tokenizer;
use crate::util;

#[wasm_bindgen]
#[derive(Debug)]
pub enum EmuErrorType {
    OutOfBoundsException,
    InstructionReadFailed,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct EmuError {
    error_type: EmuErrorType,
    description: &'static str,
}

struct CompFlags(u16);

impl CompFlags {
    const FLAG_EQUAL: CompFlags = CompFlags(0b0001);
    const FLAG_LESS: CompFlags = CompFlags(0b0010);
    const FLAG_GREATER: CompFlags = CompFlags(0b0100);

    pub fn empty() -> CompFlags {
        CompFlags(0)
    }

    pub fn is_equal(&self) -> bool {
        (self.0 & Self::FLAG_EQUAL.0) == Self::FLAG_EQUAL.0
    }

    pub fn set_equal(&mut self) {
        self.0 = self.0 | Self::FLAG_EQUAL.0;
    }

    pub fn is_less(&self) -> bool {
        (self.0 & Self::FLAG_LESS.0) == Self::FLAG_LESS.0
    }

    pub fn set_less(&mut self) {
        self.0 = self.0 | Self::FLAG_LESS.0;
    }

    pub fn is_greater(&self) -> bool {
        (self.0 & Self::FLAG_GREATER.0) == Self::FLAG_GREATER.0
    }

    pub fn set_greater(&mut self) {
        self.0 = self.0 | Self::FLAG_GREATER.0;
    }
}

#[wasm_bindgen]
pub struct GonkBoxEmu {
    memory: [u8; 0x1000],
    executing: bool,

    // gp registers
    bill: u16,
    charlie: u16,
    tim: u16,

    // special registers
    paul: u16,
    microwave: u16,
    canada: CompFlags,
}

#[wasm_bindgen]
impl GonkBoxEmu {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GonkBoxEmu {
        GonkBoxEmu {
            memory: [0; 0x1000],
            executing: false,
            bill: 0,
            charlie: 0,
            tim: 0,
            paul: 0,
            microwave: 0,
            canada: CompFlags::empty(),
        }
    }

    pub fn upload_program(&mut self, binary: &ProgramBinary) {
        self.memory = binary.get_binary().clone();
        self.executing = true;

        self.bill = 0;
        self.charlie = 0;
        self.tim = 0;
        self.canada = CompFlags::empty();

        self.paul = binary.get_start_byte();
        self.microwave = binary.get_start_byte();
    }

    pub fn is_executing(&self) -> bool {
        self.executing
    }

    pub fn get_memory(&self) -> Box<[u8]> {
        Box::new(self.memory)
    }

    pub fn fmt_memory(&self) {
        util::log!("       00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
        util::log!("------------------------------------------------------");
        for i in (0..0x100) {
            let mut str = String::from("");
            let slice = &self.memory[i * 0x10..(i + 1) * 0x10];
            str.push_str(&format!("0x{i:02X} | "));
            for b in slice {
                str.push_str(&format!("{b:02X} "));
            }
            util::log!("{str}");
        }
    }

    fn get_register(&self, register: &RegisterName) -> u16 {
        match register {
            RegisterName::Bill => self.bill,
            RegisterName::Charlie => self.charlie,
            RegisterName::Tim => self.tim,
            RegisterName::BillLow => self.bill.to_le_bytes()[0] as u16,
            RegisterName::CharlieLow => self.charlie.to_le_bytes()[0] as u16,
            RegisterName::TimLow => self.tim.to_le_bytes()[0] as u16,
            RegisterName::BillHigh => self.bill.to_le_bytes()[1] as u16,
            RegisterName::CharlieHigh => self.charlie.to_le_bytes()[1] as u16,
            RegisterName::TimHigh => self.tim.to_le_bytes()[1] as u16,
            RegisterName::Microwave => self.microwave,
            RegisterName::Paul => self.paul,
            RegisterName::Canada => self.canada.0,
        }
    }

    fn is_register_byte(&self, register: &RegisterName) -> bool {
        match register {
            RegisterName::Bill => false,
            RegisterName::Charlie => false,
            RegisterName::Tim => false,
            RegisterName::BillLow => true,
            RegisterName::CharlieLow => true,
            RegisterName::TimLow => true,
            RegisterName::BillHigh => true,
            RegisterName::CharlieHigh => true,
            RegisterName::TimHigh => true,
            RegisterName::Microwave => false,
            RegisterName::Paul => false,
            RegisterName::Canada => false,
        }
    }

    fn get_value_memory(&self, pos: u16, byte: bool) -> u16 {
        u16::from_le_bytes([
            self.memory[pos as usize],
            if byte {
                0
            } else {
                self.memory[pos as usize + 1]
            },
        ])
    }

    fn get_value(&self, arg: &ByteArgument) -> u16 {
        match arg.get_argument_type() {
            ByteArgumentType::Register(reg) => {
                let value = self.get_register(reg);
                if arg.is_address() {
                    self.get_value_memory(value, arg.is_byte())
                } else {
                    value
                }
            }
            ByteArgumentType::Immediate(value) => {
                if arg.is_address() {
                    self.get_value_memory(*value, arg.is_byte())
                } else {
                    *value
                }
            }
        }
    }

    fn put_value_memory(&mut self, value: u16, pos: u16, byte: bool) {
        let bytes = u16::to_le_bytes(value);
        self.memory[pos as usize] = bytes[0];
        if !byte {
            self.memory[pos as usize + 1] = bytes[1];
        }
    }

    fn put_value_register(&mut self, value: u16, register: &RegisterName) {
        match register {
            RegisterName::Bill => self.bill = value,
            RegisterName::Charlie => self.charlie = value,
            RegisterName::Tim => self.tim = value,
            RegisterName::BillLow => {
                let mut result = self.bill.to_le_bytes();
                result[0] = value as u8;
                self.bill = u16::from_le_bytes(result);
            }
            RegisterName::CharlieLow => {
                let mut result = self.charlie.to_le_bytes();
                result[0] = value as u8;
                self.charlie = u16::from_le_bytes(result);
            }
            RegisterName::TimLow => {
                let mut result = self.tim.to_le_bytes();
                result[0] = value as u8;
                self.tim = u16::from_le_bytes(result);
            }
            RegisterName::BillHigh => {
                let mut result = self.bill.to_le_bytes();
                result[1] = value as u8;
                self.bill = u16::from_le_bytes(result);
            }
            RegisterName::CharlieHigh => {
                let mut result = self.charlie.to_le_bytes();
                result[1] = value as u8;
                self.charlie = u16::from_le_bytes(result);
            }
            RegisterName::TimHigh => {
                let mut result = self.tim.to_le_bytes();
                result[1] = value as u8;
                self.tim = u16::from_le_bytes(result);
            }
            RegisterName::Microwave => self.microwave = value,
            RegisterName::Paul => self.paul = value,
            RegisterName::Canada => self.canada = CompFlags(value),
        }
    }

    fn put_value(&mut self, src: u16, dest: &ByteArgument) {
        match dest.get_argument_type() {
            ByteArgumentType::Register(reg) => {
                if dest.is_address() {
                    let value = self.get_register(reg);
                    self.put_value_memory(src, value, dest.is_byte());
                } else {
                    self.put_value_register(src, reg);
                }
            }
            ByteArgumentType::Immediate(value) => {
                if !dest.is_address() {
                    panic!("Fed immediate as destination to put_value.");
                }
                self.put_value_memory(src, *value, dest.is_byte());
            }
        }
    }

    fn get_argsize(&self, args: &Vec<ByteArgument>) -> u16 {
        let mut size = 0;
        for arg in args {
            if matches!(arg.get_argument_type(), ByteArgumentType::Register(_)) {
                size += 1;
            } else if arg.is_address() {
                size += 2;
            } else if arg.is_byte() {
                size += 1;
            } else {
                size += 2;
            }
        }
        size
    }

    pub fn step(&mut self) -> Result<Option<String>, EmuError> {
        if (!self.executing) {
            return Ok(None);
        }

        let ip = self.paul;
        if (ip >= 0x1000) {
            return Err(EmuError {
                error_type: EmuErrorType::OutOfBoundsException,
                description: "Tried to access an instruction outside of memory.",
            });
        }
        let instruction = match bytes_to_instruction(&self.memory, ip) {
            Ok(result) => result,
            Err(err) => {
                return Err(EmuError {
                    error_type: EmuErrorType::InstructionReadFailed,
                    description: "Encountered unparsable instructions.",
                });
            }
        };

        let mut increment_paul = true;

        let arguments = &instruction.get_arguments();

        match instruction.get_instruction_type() {
            InstructionType::Move => {
                let arg1 = &arguments[0];
                let arg2 = &arguments[1];

                let src = self.get_value(arg1);
                self.put_value(src, arg2);
            }
            InstructionType::Add => {
                let arg1 = &arguments[0];
                let arg2 = &arguments[1];

                let src = self.get_value(arg1);
                let dest = self.get_value(arg2);
                self.put_value(src + dest, arg2);
            }
            InstructionType::Sub => {
                let arg1 = &arguments[0];
                let arg2 = &arguments[1];

                let src = self.get_value(arg1);
                let dest = self.get_value(arg2);
                self.put_value(src - dest, arg2);
            }
            InstructionType::Inc => {
                let arg = &arguments[0];
                let value = self.get_value(arg) + 1;
                self.put_value(value, arg);
            }
            InstructionType::Dec => {
                let arg = &arguments[0];
                let value = self.get_value(arg) - 1;
                self.put_value(value, arg);
            }
            InstructionType::Flip => {
                let arg = &arguments[0];
                let value = -(self.get_value(arg) as i16) as u16;
                self.put_value(value, arg);
            }
            InstructionType::Comp => {
                let arg1 = &arguments[0];
                let arg2 = &arguments[1];

                let src = self.get_value(arg1);
                let dest = self.get_value(arg2);

                let mut result = CompFlags::empty();
                if (src == dest) {
                    result.set_equal();
                }
                if (src > dest) {
                    result.set_greater();
                }
                if (src < dest) {
                    result.set_less();
                }

                self.canada = result;
            }
            InstructionType::Or => {
                let arg1 = &arguments[0];
                let arg2 = &arguments[1];

                let src = self.get_value(arg1);
                let dest = self.get_value(arg2);

                self.put_value(src | dest, arg2);
            }
            InstructionType::And => {
                let arg1 = &arguments[0];
                let arg2 = &arguments[1];

                let src = self.get_value(arg1);
                let dest = self.get_value(arg2);

                self.put_value(src & dest, arg2);
            }
            InstructionType::Nand => {
                let arg1 = &arguments[0];
                let arg2 = &arguments[1];

                let src = self.get_value(arg1);
                let dest = self.get_value(arg2);

                self.put_value(!(src & dest), arg2);
            }
            InstructionType::Not => {
                let arg = &arguments[0];
                let src = self.get_value(arg);
                self.put_value(!src, arg);
            }
            InstructionType::Jump => {
                self.paul = self.microwave;
                increment_paul = false;
            }
            InstructionType::JumpE => {
                if (self.canada.is_equal()) {
                    self.paul = self.microwave;
                    increment_paul = false;
                }
            }
            InstructionType::JumpNE => {
                if (!self.canada.is_equal()) {
                    self.paul = self.microwave;
                    increment_paul = false;
                }
            }
            InstructionType::JumpL => {
                if (self.canada.is_less()) {
                    self.paul = self.microwave;
                    increment_paul = false;
                }
            }
            InstructionType::JumpG => {
                if (self.canada.is_greater()) {
                    self.paul = self.microwave;
                    increment_paul = false;
                }
            }
            InstructionType::Stop => {
                self.executing = false;
            }
        }

        if (increment_paul) {
            self.paul += 2;
            self.paul += self.get_argsize(arguments);
        }

        Ok(Some(format!("{instruction:#?}")))
    }

    pub fn get_register_text(&self) -> String {
        let bill = self.bill;
        let charlie = self.charlie;
        let tim = self.tim;
        let microwave = self.microwave;
        let paul = self.paul;
        let canada = self.canada.0;
        format!(
            "Bill: {bill:02X}\nCharlie: {charlie:02X}\nTim: {tim:02X}\nMicrowave: {microwave:02X}\nPaul: {paul:02X}\nCanada: {canada:02X}"
        )
    }

    pub fn get_bill(&self) -> u16 {
        self.bill
    }

    pub fn get_charlie(&self) -> u16 {
        self.charlie
    }

    pub fn get_tim(&self) -> u16 {
        self.tim
    }

    pub fn get_paul(&self) -> u16 {
        self.paul
    }

    pub fn get_microwave(&self) -> u16 {
        self.microwave
    }

    pub fn get_canada(&self) -> u16 {
        self.canada.0
    }

    pub fn try_read(&mut self) -> Option<u8> {
        if self.memory[2] != 0 {
            let byte = self.memory[3];
            self.memory[2] = 0;
            return Some(byte);
        }
        None
    }

    pub fn try_write(&mut self, byte: u8) -> bool {
        if self.memory[4] == 0 {
            self.memory[5] = byte;
            self.memory[4] = 1;
            return true;
        }
        false
    }
}

#[test]
fn test() {
    let source_code = r#"
    label msg               ; comment
    istr "hello world!\n"   ; comment

    label start             ; comment
    move 1 bill             ; comment
    move 2 charlie          ; comment
    add bill charlie        ; comment

    comp bill charlie       ; comment
    move print microwave    ; comment
    jumpne                  ; comment
    stop                    ; comment

    label print             ; comment
    $PRINT msg              ; comment
    stop                    ; comment
    "#;

    let mut tokenizer = Tokenizer::new(source_code.into());
    let tokens = match tokenizer.build() {
        Ok(result) => result,
        Err(err) => {
            for error in err {
                util::log!("Error [{}]: {}", error.get_line(), error.get_error());
            }
            panic!("Tokenizer failed.");
        }
    };

    let program_binary = match build_gonkbox_program(tokens) {
        Ok(result) => result,
        Err(err) => {
            util::log!("Error {err:#?}");
            panic!("Parse failed.");
        }
    };

    let mut emu = GonkBoxEmu::new();
    emu.upload_program(&program_binary);

    while (emu.is_executing()) {
        let result = emu.step();
        match result {
            Err(err) => {
                util::log!("{err:#?}");
                panic!("{err:#?}");
            }
            _ => {}
        }
        let output = emu.try_read();
        match output {
            Some(b) => {
                let c = b as char;
                print!("{c}");
            }
            None => {}
        }
    }
}
