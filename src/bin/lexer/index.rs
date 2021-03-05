mod keyword_lexemes;
mod lexemes;
mod tokenTypes;

mod position;
mod tokens;

use std::env; /** for command line entries */

//need to get a module for interpreter errors.

let lexemes = lexemes::Lexemes();
let keyword_lexemes = keyword_lexemes::Keyword_lexemes();

struct Lexer {
    fileName: String,
    position: position::Position,
    currentChar: Option<char>,
    text: String
}

impl Lexer {
    fn new(&self, fileName: String, text: String) -> Lexer{
        let mut text_clone = text.clone();
        let fileName_clone = fileName.clone();
        let semi = lexeme::Lexemes.get("semi").unwrap();
        let line = lexeme::Lexemes.get("line").unwrap();

//lex.line.replace_all(str, new_str)
        if (fileName == "<stdin>"){
            text_clone = semi.replace_all(text_clone, "\n" );
        }
        
        text_clone = line.replace(text_clone, "@");

        let lexer = Lexer{
            fileName: fileName_clone,
            position: position::Position::new(-1,0,-1, fileName, text),
            currentChar: None,
            text: text_clone
        }

        self.advnace();
    }
    //get cli colors lib from crates

    fn advance(&self){
        self.position.advance(self.currentChar);
        self.currentChar = if (self.position.idx < self.text.len()){
            &self.text[self.position.idx];
        } else{
            None;
        }
    }

    fn makeNumber(&self) -> token::Token<T> {
        let mut numStr = String::new();
        let mut dotCount: usize = 0;
        let posStart = self.position.copy();
        let digits = lexeme::Lexemes.get("digits").unwrap();
        let dot = lexeme::Lexemes.get("dot").unwarp();
        let int = tokenTypes::TokenTypes.get("INT");
        let float = tokenTypes::TokenTypes.get("FLOAT");

        /** Keep going while character is a digit or a dot, and we haven't seen a dot yet */

        while(self.currentChar != None && (digits.is_match(self.currentChar) || dot.is_match(self.currentChar) )){
            if (dot.is_match(self.currentChar)){
                if (dotCount == 1){
                    break;
                }
                dotCount += 1;
            }

            numStr.push(self.currentChar);
            self.advance();
        }

        //Check if INT or FLOAT
        if (dotCount == 0){
            return token::Token::new(int, numStr.parse::<i64>().unwrap(), posStart, self.position)
        } else{
            return token::Token::new(int, numStr.parse::<f64>().unwrap(), posStart, self.position)
        }
    }


}
