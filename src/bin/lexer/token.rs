/**
 * smallest recognizable component of the programming language
 */
mod position;


struct Token<T>{
    tok_type: String,
    value: T,
    posStart: position::Position,
    posEnd: position::Position
}

impl<T> Token<T>{
    fn new(tok_type: String, value: Option<T>, posStart: Option<Position>, posEnd: Option<Position> ) -> Token<T>{
        let mut new_token = Token{
            tok_type,
            value,
            posStart,
            posEnd
        };

        match posStart {
            Some(posStart) => {
                new_token.posStart = posStart;
                new_token.posEnd = posStart.copy();
                new_token.posEnd.advance();
            },
            _ => println!("no posStart given")
        }

        match posEnd {
            Some(posEnd) => {
                new_token.posEnd = posEnd;
            },
            _ => println!("no posEnd given")
        }

        return new_token;
    }

    fn matches(&self, tok_type: String, value: Option<T>) -> bool{
        return self.tok_type == tok_type && self.value == value
    }

    fn to_string(){ //TODO colors stuff here

    }
}
