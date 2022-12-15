/** base level Keyword units to check for */

use std::collections::HashMap;

pub fn Keyword_lexemes() -> HashMap<String, String> {

let keyword_lexemes = HashMap::new();

    keyword_lexemes.insert(String::from("let"), String::from("wacha"));
    keyword_lexemes.insert(String::from("if"), String::from("kama"));
    keyword_lexemes.insert(String::from("elif"), String::from("au"));
    keyword_lexemes.insert(String::from("else"), String::from("sivyo"));
    keyword_lexemes.insert(String::from("for"), String::from("kwa"));
    keyword_lexemes.insert(String::from("to"), String::from("mpaka"));
    keyword_lexemes.insert(String::from("in"), String::from("katika"));
    keyword_lexemes.insert(String::from("step"), String::from("hatua"));
    keyword_lexemes.insert(String::from("while"), String::from("ambapo"));
    keyword_lexemes.insert(String::from("function"), String::from("shughuli"));
    keyword_lexemes.insert(String::from("try"), String::from("jaribu"));
    keyword_lexemes.insert(String::from("catch"), String::from("iwapo"));
    keyword_lexemes.insert(String::from("finally"), String::from("mwishowe"));
    keyword_lexemes.insert(String::from("return"), String::from("rudisha"));
    keyword_lexemes.insert(String::from("continue"), String::from("endelea"));
    keyword_lexemes.insert(String::from("break"), String::from("ondoka"));
    keyword_lexemes.insert(String::from("throw"), String::from("tupa"));

    return keyword_lexemes;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_returns_a_hashmap_of_keyword_lexemes(){
        let res = Keyword_lexemes();
        assert_eq!(res, 16);
    }
}
