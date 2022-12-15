/** base level units to check for */
use regex::Regex;
use std::collections::HashMap;

pub fn Lexemes() -> HashMap<String, Regex>{
    let mut lexemes = HashMap::new(); // this hash map is what is returned from the lexemes file.

    lexemes.insert(String::from("digits"), Regex::new(r"[0-9]").unwrap());
    lexemes.insert(String::from("alpha"), Regex::new(r"[[:aplha:]]").unwrap());
    lexemes.insert(String::from("dot"), Regex::new(r"\.").unwrap());
    lexemes.insert(String::from("doubleQuotes"), Regex::new(r#"\""#).unwrap());
    lexemes.insert(String::from("plus"), Regex::new(r"[\+]").unwrap());
    lexemes.insert(String::from("hyphen"), Regex::new(r"[\-]").unwrap());
    lexemes.insert(String::from("asterisk"), Regex::new(r"[\*]").unwrap());
    lexemes.insert(String::from("modulo"), Regex::new(r"[%]").unwrap());
    lexemes.insert(String::from("forwardSlash"), Regex::new(r"\/").unwrap());
    lexemes.insert(String::from("backSlash"), Regex::new(r"\\").unwrap());
    lexemes.insert(String::from("caret"), Regex::new(r"\^").unwrap());
    lexemes.insert(String::from("leftParen"), Regex::new(r"\(").unwrap());
    lexemes.insert(String::from("rightParen"), Regex::new(r"\)").unwrap());
    lexemes.insert(String::from("leftSquare"), Regex::new(r"\[").unwrap());
    lexemes.insert(String::from("rightSquare"), Regex::new(r"\]").unwrap());
    lexemes.insert(String::from("leftCurly"), Regex::new(r"\{").unwrap());
    lexemes.insert(String::from("rightCurly"), Regex::new(r"\}").unwrap());
    lexemes.insert(String::from("col"), Regex::new(r":").unwrap());
    lexemes.insert(String::from("comma"), Regex::new(r",").unwrap());
    lexemes.insert(String::from("ampersand"), Regex::new(r"&").unwrap());
    lexemes.insert(String::from("pipe"), Regex::new(r"\|").unwrap());
    lexemes.insert(String::from("exclamation"), Regex::new(r"!").unwrap());
    lexemes.insert(String::from("equals"), Regex::new(r"=").unwrap());
    lexemes.insert(String::from("leftArrow"), Regex::new(r"<").unwrap());
    lexemes.insert(String::from("semi"), Regex::new(r";/g").unwrap()); //find out what the g anchor does
    lexemes.insert(String::from("line"), Regex::new(r"\r?\n/g").unwrap()); //find out what the g anchor does
    lexemes.insert(String::from("lineEndings"), Regex::new(r"@").unwrap()); //as far as the lexer is concerned
    lexemes.insert(String::from("spacesAndTabs"), Regex::new(r"\s|\t").unwrap());

    return lexemes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_a_hashmap_of_length_27(){
        let res = Lexemes();
        assert_eq!(res.len(), 27)
    }
}
