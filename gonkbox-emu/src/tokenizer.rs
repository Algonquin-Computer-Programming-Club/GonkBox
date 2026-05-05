use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use crate::asm_parser::{GonkASMToken, GonkASMTokenType};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TokenizerError {
    line: usize,
    error: String,
}

#[wasm_bindgen]
impl TokenizerError {
    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_error(&self) -> String {
        self.error.clone()
    }
}

#[wasm_bindgen]
pub struct Tokenizer {
    source: String,
    line: usize,
    start: usize,
    cursor: usize,
    tokens: Vec<GonkASMToken>,
    errors: Vec<TokenizerError>,
    has_errors: bool,
    identifier_type_map: HashMap<String, GonkASMTokenType>,
}

#[wasm_bindgen]
impl Tokenizer {
    #[wasm_bindgen(constructor)]
    pub fn new(source: String) -> Tokenizer {
        Tokenizer {
            source,
            line: 0,
            start: 0,
            cursor: 0,
            tokens: Vec::new(),
            errors: Vec::new(),
            has_errors: false,
            identifier_type_map: HashMap::from([
                ("bill".into(), GonkASMTokenType::Register),
                ("bill_h".into(), GonkASMTokenType::Register),
                ("bill_l".into(), GonkASMTokenType::Register),
                ("b".into(), GonkASMTokenType::Register),
                ("b_l".into(), GonkASMTokenType::Register),
                ("b_h".into(), GonkASMTokenType::Register),
                ("charlie".into(), GonkASMTokenType::Register),
                ("charlie_l".into(), GonkASMTokenType::Register),
                ("charlie_h".into(), GonkASMTokenType::Register),
                ("c".into(), GonkASMTokenType::Register),
                ("c_l".into(), GonkASMTokenType::Register),
                ("c_h".into(), GonkASMTokenType::Register),
                ("tim".into(), GonkASMTokenType::Register),
                ("tim_l".into(), GonkASMTokenType::Register),
                ("tim_h".into(), GonkASMTokenType::Register),
                ("t".into(), GonkASMTokenType::Register),
                ("t_l".into(), GonkASMTokenType::Register),
                ("t_h".into(), GonkASMTokenType::Register),
                ("paul".into(), GonkASMTokenType::Register),
                ("microwave".into(), GonkASMTokenType::Register),
                ("m".into(), GonkASMTokenType::Register),
                ("canada".into(), GonkASMTokenType::Register),
                ("move".into(), GonkASMTokenType::Instruction),
                ("add".into(), GonkASMTokenType::Instruction),
                ("sub".into(), GonkASMTokenType::Instruction),
                ("mul".into(), GonkASMTokenType::Instruction),
                ("div".into(), GonkASMTokenType::Instruction),
                ("inc".into(), GonkASMTokenType::Instruction),
                ("dec".into(), GonkASMTokenType::Instruction),
                ("comp".into(), GonkASMTokenType::Instruction),
                ("jump".into(), GonkASMTokenType::Instruction),
                ("jumpe".into(), GonkASMTokenType::Instruction),
                ("jumpne".into(), GonkASMTokenType::Instruction),
                ("jumpl".into(), GonkASMTokenType::Instruction),
                ("jumpg".into(), GonkASMTokenType::Instruction),
                ("stop".into(), GonkASMTokenType::Instruction),
                ("dlogn".into(), GonkASMTokenType::Instruction),
                ("dlogc".into(), GonkASMTokenType::Instruction),
                ("dlogs".into(), GonkASMTokenType::Instruction),
                ("dbyte".into(), GonkASMTokenType::Command),
                ("dbytes".into(), GonkASMTokenType::Command),
                ("ibyte".into(), GonkASMTokenType::Command),
                ("ibytes".into(), GonkASMTokenType::Command),
                ("istr".into(), GonkASMTokenType::Command),
                ("istrn".into(), GonkASMTokenType::Command),
                ("dword".into(), GonkASMTokenType::Command),
                ("dwords".into(), GonkASMTokenType::Command),
                ("iword".into(), GonkASMTokenType::Command),
                ("iwords".into(), GonkASMTokenType::Command),
                ("label".into(), GonkASMTokenType::Label),
            ]),
        }
    }

    fn error(&mut self, line: usize, error: String) {
        self.errors.push(TokenizerError { line, error });
        self.has_errors = true;
    }

    fn advance(&mut self) -> char {
        self.cursor += 1;
        self.source.as_bytes()[self.cursor - 1].into()
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.source.as_bytes()[self.cursor].into()
        }
    }

    fn at_end(&self) -> bool {
        self.cursor >= self.source.len()
    }

    fn get_current_string(&self) -> String {
        self.source[self.start..self.cursor].to_owned()
    }

    fn capture_string(&mut self) {
        self.advance();
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.error(self.line, "Unterminated string.".into());
            }
            self.advance();
        }

        if self.at_end() {
            self.error(self.line, "Unterminated string.".into());
        }

        self.advance();

        self.add_token(GonkASMTokenType::StringLiteral);
    }

    fn capture_char(&mut self) {
        if self.peek() == '\'' {
            self.error(self.line, "Prematurely ended character.".into());
        }
        self.advance();
        if self.peek() != '\'' {
            self.error(self.line, "Badly sized character.".into());
        }

        self.advance();

        let string = self.get_current_string();
        if string.len() == 3 {
            let byte = string.as_bytes()[1];
            let num_string = byte.to_string();
            self.add_token_ex(GonkASMTokenType::ImmediateLiteral, num_string);
        } else {
            self.error(self.line, "Badly sized character.".into());
        }
    }

    fn capture_escape_char(&mut self) {
        self.advance();
        if self.peek() != '\\' && self.peek() != 'n' && self.peek() != '\'' {
            self.error(self.line, "Invalid escape character.".into());
        }

        self.advance();
        self.advance();

        let mut string = self.get_current_string();
        if string.len() == 4 {
            string = string.replace("\\\\", "\\");
            string = string.replace("\\n", "\n");
            string = string.replace("\\\'", "\'");
            let byte = string.as_bytes()[1];
            let num_string = byte.to_string();
            self.add_token_ex(GonkASMTokenType::ImmediateLiteral, num_string);
        } else {
            self.error(self.line, "Badly sized escape character.".into());
        }
    }

    fn capture_num(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        self.add_token(GonkASMTokenType::ImmediateLiteral);
    }

    fn capture_macro(&mut self) {
        self.advance();
        while self.peek().is_ascii_alphabetic() || self.peek() == '_' {
            self.advance();
        }

        self.add_token(GonkASMTokenType::Macro)
    }

    fn capture_identifier(&mut self) {
        while self.peek().is_ascii_alphabetic() || self.peek() == '_' {
            self.advance();
        }

        let string = self.get_current_string();
        let reserved = self.identifier_type_map.get(&string);
        match reserved {
            Some(result) => self.add_token(*result),
            None => self.add_token(GonkASMTokenType::Identifier),
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '*' => self.add_token(GonkASMTokenType::RamBracket),
            ';' => {
                while self.peek() != '\n' && !self.at_end() {
                    self.advance();
                }
                self.line += 1;
            }
            ' ' => {}
            '\t' => {}
            '\r' => {}
            '\n' => self.line += 1,
            '"' => self.capture_string(),
            '\'' => {
                if self.peek() == '\\' {
                    self.capture_escape_char();
                } else {
                    self.capture_char();
                }
            }
            '$' => self.capture_macro(),
            _ => {
                if c.is_digit(10) {
                    self.capture_num();
                } else if c.is_alphabetic() {
                    self.capture_identifier();
                } else {
                    self.error(self.line, "Unrecognized character.".into())
                }
            }
        };
    }

    fn add_token(&mut self, token_type: GonkASMTokenType) {
        self.add_token_ex(token_type, self.get_current_string());
    }

    fn add_token_ex(&mut self, token_type: GonkASMTokenType, value: String) {
        self.tokens.push(GonkASMToken::new(
            value,
            token_type,
            self.line,
            self.start,
            self.cursor,
        ));
    }

    pub fn build(&mut self) -> Result<Vec<GonkASMToken>, Vec<TokenizerError>> {
        while !self.at_end() {
            self.start = self.cursor;
            self.scan_token();
        }
        if self.has_errors {
            Err(self.errors.clone())
        } else {
            Ok(self.tokens.clone())
        }
    }

    pub fn get_tokens(&self) -> Vec<GonkASMToken> {
        self.tokens.clone()
    }
}
