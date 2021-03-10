/**
 * smallest recognizable component of the programming language
 */
#[path = "./position.rs"]
mod position;

#[derive(PartialEq)]
pub struct Token<T> {
    token_type: String,
    value: T,
    posStart: position::Position,
    posEnd: position::Position,
}

impl<T> Token<T> {
    fn new(
        TT: String,
        value: Option<T>,
        posStart: Option<position::Position>,
        posEnd: Option<position::Position>,
    ) -> Token<T> {
        let mut new_token = Token::<T> {
            token_type: TT,
            value: value,
            posStart: posStart.unwrap(),
            posEnd: posEnd.unwrap(),
        };

        if posStart.is_some(){
           new_token.posStart = posStart.unwrap();
           new_token.posEnd = posStart.unwrap().copy();
           new_token.posEnd.advance(None);
        }

        if posEnd.is_some() {
           new_token.posEnd = posEnd.unwrap();
        }
        return new_token;
    }

    fn matches(&self, TT: String, value: Option<Box<T>>) -> bool {
        return self.token_type == TT && self.value == value;
    }

    fn to_string() { //TBD
    }
}
