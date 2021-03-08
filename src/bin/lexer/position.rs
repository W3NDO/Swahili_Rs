/**
 * represents the exact line/colNumber/File position for the lexer, parser and interpreter
 */
#[path = "./lexeme.rs"]
mod lexeme;

pub struct Position {
    pub idx: i64,     //index(position) in the text content
    lineNumber: i64,  //line number
    colNumber: i64,   //column number
    fileName: String, //current file name
    fileText: String, //current file text content
}

impl Position {
    //static method
    pub fn new(
        idx: i64,
        lineNumber: i64,
        colNumber: i64,
        fileName: String,
        fileText: String,
    ) -> Position {
        Position {
            idx: idx,
            lineNumber: lineNumber,
            colNumber: colNumber,
            fileName: fileName,
            fileText: fileText,
        }
    }

    pub fn advance(&self, currentChar: Option<char>) -> &Position {
        self.idx += 1;
        self.colNumber += 1;
        let lexemes = lexeme::Lexemes();
        let lineEnding = lexemes.get("lineEndings").unwrap();
        if (lineEnding.is_match(currentChar.unwrap_or(None))) {
            self.lineNumber += 1;
            self.colNumber = 0;
        }
        return self;
    }

    pub fn copy(&self) -> Position {
        return Position {
            idx: self.idx,
            lineNumber: self.lineNumber,
            colNumber: self.colNumber,
            fileName: self.fileName,
            fileText: self.fileText,
        };
    }
}
