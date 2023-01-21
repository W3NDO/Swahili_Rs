fn main() {
    println!("Habari, Dunia!");
}

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calc);
pub mod ast;
#[test]
fn calc(){
    let expr = calc::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");

    let expr2 = calc::ExprParser::new()
        .parse("62 % 60 + 1 ")
        .unwrap();
    assert_eq!(&format!("{:?}", expr2), "((62 % 60) + 1)");
}
