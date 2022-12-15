/** map of all recognized token types. */

use std::collections::HashMap;

pub fn TokenTypes() -> HashMap<String, String>{

    let mut tokenTypes = HashMap::new();

    /** Integer */
    tokenTypes.insert(String::from("INT"), String::from("INT"));

    /** Float */
    tokenTypes.insert(String::from("FLOAT"), String::from("FLOAT"));

    /*String*/
    tokenTypes.insert(String::from("STRING"), String::from("STRING"));

    /** Identifier */
    tokenTypes.insert(String::from("IDENTIFIER"), String::from("IDENTIFIER"));

    /** Keyword */
    tokenTypes.insert(String::from("KEYWORD"), String::from("KEYWORD"));

    /** Addition Operator */
    tokenTypes.insert(String::from("PLUS"), String::from("PLUS"));

    /** Subtraction Operator */
    tokenTypes.insert(String::from("MINUS"), String::from("MINUS"));

    /** Multiplication Operator */
    tokenTypes.insert(String::from("MUL"), String::from("MUL"));

    /** Division Operator */
    tokenTypes.insert(String::from("DIV"), String::from("DIV"));

    /** Power Operator */
    tokenTypes.insert(String::from("POW"), String::from("POW"));

    /** Modulo Operator */
    tokenTypes.insert(String::from("MOD"), String::from("MOD"));

    /** Assignment Operator */
    tokenTypes.insert(String::from("EQ"), String::from("EQ"));

    /** Left Parentheses */
    tokenTypes.insert(String::from("LPAREN"), String::from("LPAREN"));

    /** Right Parentheses */
    tokenTypes.insert(String::from("RPAREN"), String::from("RPAREN"));

    /** Left Square Bracket */
    tokenTypes.insert(String::from("RSQUARE"), String::from("LSQUARE"));

    /** Right Square Bracket */
    tokenTypes.insert(String::from("RSQUARE"), String::from("RSQUARE"));

    /** Left Curly Bracket */
    tokenTypes.insert(String::from("RCURL"), String::from("LCURL"));

    /** Right CUrly Bracket */
    tokenTypes.insert(String::from("RCURL"), String::from("RCURL"));

    /** Double Equal Comparison */
    tokenTypes.insert(String::from("EE"), String::from("EE"));

    /** Not Equal Comparison */
    tokenTypes.insert(String::from("NE"), String::from("NE"));

    /** Less Than Comparison */
    tokenTypes.insert(String::from("LT"), String::from("LT"));

    /** Greater Than Comparison */
    tokenTypes.insert(String::from("GT"), String::from("GT"));

    /** Less Than or equal comparison */
    tokenTypes.insert(String::from("LTE"), String::from("LTE"));

    /** Greater Than or Equal comparison */
    tokenTypes.insert(String::from("GTE"), String::from("GTE"));

    /** Dot */
    tokenTypes.insert(String::from("DOT"), String::from("DOT"));

    /** Colon */
    tokenTypes.insert(String::from("COL"), String::from("COL"));

    /** Comma */
    tokenTypes.insert(String::from("COMMA"), String::from("COMMA"));

    /** AND Symbol */
    tokenTypes.insert(String::from("AND"), String::from("AND"));

    /** OR symbol */
    tokenTypes.insert(String::from("OR"), String::from("OR"));

    /** NOT Symbol */
    tokenTypes.insert(String::from("NOT"), String::from("NOT"));

    /** End of File */
    tokenTypes.insert(String::from("EOF"), String::from("EOF"));

    /** New Line */
    tokenTypes.insert(String::from("NEWLINE"), String::from("NEWLINE"));

    return tokenTypes;
}
