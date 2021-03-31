mod keyword_lexemes;
mod lexeme;
mod tokenTypes;
mod position;
mod token;

/** for command line entries */
use std::env;
use std::convert::TryInto;

use std::collections::HashMap;

pub fn Lexer() {
    let lexemes = lexeme::Lexemes();
    let keyword_lexemes = keyword_lexemes::Keyword_lexemes();

    println!("Hello Lexer");
}

struct Lexer {
    fileName: String,
    position: position::Position,
    currentChar: Option<char>,
    text: String,
}

impl Lexer {
    fn new(&self, fileName: String, text: String) -> Lexer {
        let mut text_clone = text.clone();
        let fileName_clone = fileName.clone();
        let lexemes = lexeme::Lexemes();
        let semi = lexemes.get("semi").unwrap();
        let line = lexemes.get("line").unwrap();

        //lex.line.replace_all(str, new_str)
        if fileName == "<stdin>" {
            text_clone = semi.replace_all(&text_clone, "\n").to_string();
        }
        text_clone = line.replace_all(&text_clone, "@").to_string();

        let lexer = Lexer {
            fileName: fileName_clone,
            position: position::Position::new(-1, 0, -1, fileName, text),
            currentChar: None,
            text: text_clone,
        };

        self.advance();

        return *self;
    }
    //get cli colors lib from crates

    fn advance(&self) {
        self.position.advance(self.currentChar);
        if self.position.idx < self.text.len().try_into().unwrap() {
            self.currentChar = Some(self.text.chars().nth(self.position.idx.try_into().unwrap()).unwrap());
        } else {
            self.currentChar = None;
        }
    }

    fn makeNumber(&self) -> token::Token::<T> {
        let mut numStr = String::new();
        let mut dotCount: usize = 0;
        let posStart = self.position.copy();
        let lexeme = lexeme::Lexemes();
        let token_type = tokenTypes::TokenTypes();
        let digits = lexeme.get("digits").unwrap();
        let dot = lexeme.get("dot").unwrap();
        let int = token_type.get("INT");
        let float = token_type.get("FLOAT");

        /** Keep going while character is a digit or a dot, and we haven't seen a dot yet */
        while self.currentChar != None
            && (digits.is_match(&self.currentChar.unwrap().to_string()) || dot.is_match(&self.currentChar.unwrap().to_string()))
        {
            if dot.is_match(&self.currentChar.unwrap().to_string()) {
                if dotCount == 1 {
                    break;
                }
                dotCount += 1;
            }

            numStr.push(self.currentChar.unwrap());
            self.advance();
        }

        //Check if INT or FLOAT
        if dotCount == 0 {
            return token::<i64>Token::<i64>new(int, numStr.parse::<i64>().unwrap(), posStart, self.position);
        } else {
            return token::<f64>Token::<f64>new(float, numStr.parse::<f64>().unwrap(), posStart, self.position);
        }
    }

    fn makeString(&self){
        let mut string = String::new();
        let posStart = self.position.copy();
        let mut escapeCharacter: bool = false;
        let lexeme = lexeme::Lexemes();
        let double_quotes = lexeme.get("doubleQuotes").unwrap();
        let backslash = lexeme.get("backSlash").unwrap();
        let token_type = tokenTypes::TokenTypes();
        let string_token = token_type.get("STRING");
        self.advance();

        while self.currentChar != None 
            && (!(double_quotes.is_match(&self.currentChar.unwrap().to_string())) || escapeCharacter) {
                if escapeCharacter{
                    if double_quotes.is_match(&self.currentChar.unwrap().to_string()){
                        string.push(self.currentChar.unwrap());
                    } else {
                        string.push_str("\\");
                        string.push(self.currentChar.unwrap());
                    }
                    escapeCharacter = false;
                } else {
                    if backslash.is_match(&self.currentChar.unwrap().to_string()){
                        escapeCharacter = true;
                    } else {
                        string.push(self.currentChar.unwrap());
                    }
                }
                self.advance();
            }
        self.advance();
        return token::<String>Token::<String>new(string_token, string, posStart, self.position);
    }

    fn makeIdentifier(self){
        let idStr = String::new();
        let posStart = self.position.copy();
        let lexeme = lexeme::Lexemes();
        let token_type = tokenTypes::TokenTypes();
        let digits = lexeme.get("digits").unwrap();
        let alpha = lexeme.get("alpha").unwrap();

        while self.currentChar != None &&
            (digits.is_match(&self.currentChar.unwrap().to_string()) || 
              alpha.is_match(&self.currentChar.unwrap().to_string())
        ) {
            idStr.push(self.currentChar.unwrap());
            self.advance();
        }

        let keywords = keyword_lexemes::Keyword_lexemes();
        let tokType = match keywords.get(&idStr){
            Some(keyword) => token_type.get("KEYWORD"),
            None => token_type.get("IDENTIFIER")
        };

        return token::<String>Token::<String>new(tokType, idStr, posStart, self.position);
    }

}
