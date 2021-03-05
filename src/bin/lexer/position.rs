/**
 * represents the exact line/colNumber/File position for the lexer, parser and interpreter
 */
mod lexeme;

struct Position {
    idx: i64,         //index(position) in the text content
    lineNumber: i64,  //line number
    colNumber: i64,   //column number
    fileName: String, //current file name
    fileText: String, //current file text content
}

impl Position {
    //static method
    fn new(
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

    fn advance(&self, currentChar: Option<char> = None ) -> Position{
        self.idx ++;
        self.colNumber++;
        
        lineEnding = lexeme::lexemes.get("lineEndings").unwrap();
        if (currentChar.is_match(lineEnding)){
            self.lineNumber++;
            self.colNumber = 0;
        }

        return self;
    }

    fn copy(&self) -> Position{
        return Postion{
            idx: self.idx,
            lineNumber: self.lineNumber,
            colNumber: self.colNumber,
            fileName: self.fileName,
            fileText: self.fileText
        };
    }
}
