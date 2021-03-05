/**
 * smallest recognizable component of the programming language
 */
mod position;


struct Token<T>{
    type: String,
    value: T,
    posStart: position::Position,
    posEnd: position::Position
}

impl<T> Token<T>{
    fn new(type: String, value: Option<T> = None, posStart: Option<Position>, posEnd: Option<Position> = None ) -> Token<T>{
        let mut new_token = Token{
            type: type,
            value: value,
            posStart: posStart,
            posEnd: posEnd
        }

        if(posStart){
            new_token.posStart = posStart;
            new_token.posEnd = posStart.copy();
            new_token.posEnd.advance();
        }

        if(posEnd) {
            new_token.posEnd = posEnd;
        }
        return new_token;
    }

    fn matches(&self, type: String, value: Option<T>) -> bool{
        return self.type == type && self.value == value
    }

    fn to_string(){ //TBD

    }
}