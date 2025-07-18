use crate::ParseError;

use crate::test::{
    calculator_grammar::{grammar, token_type::TokenType},
    helpers,
};

fn parse(input: &str) -> Result<Option<i32>, ParseError<TokenType>> {
    let answ = helpers::parse(grammar::lexer_parser, input);

    answ
}

#[test]
fn sum_test() {
    let answ = parse("2 + 3 + 1 - 5").unwrap().unwrap();

    assert_eq!(answ, 1);
}

#[test]
fn multiplication_test() {
    let answ = parse("2 * 3 / 3").unwrap().unwrap();

    assert_eq!(answ, 2);
}

#[test]
fn composition_test() {
    let answ = parse("3 + 2 * 3").unwrap().unwrap();

    assert_eq!(answ, 9);
}

#[test]
fn parenthesis_test() {
    let answ = parse("(3 + 2) * 3").unwrap().unwrap();

    assert_eq!(answ, 15);
}


// Error handling tests
#[test]
fn syntax_error_test() {
    let result = parse("2 + + 3");
    assert!(result.is_err());
}

#[test]
fn unmatched_parenthesis_test() {
    let result = parse("(2 + 3");
    assert!(result.is_err());
}

#[test]
fn empty_parenthesis_test() {
    let result = parse("2 + ()");
    assert!(result.is_err());
}

#[test]
fn invalid_operator_sequence_test() {
    let result = parse("2 */ 3");
    assert!(result.is_err());
}

// Edge cases tests
#[test]
fn single_number_test() {
    let result = parse("42").unwrap().unwrap();
    assert_eq!(result, 42);
}

#[test]
fn whitespace_handling_test() {
    let result = parse("  2   +   3  ").unwrap().unwrap();
    assert_eq!(result, 5);
}

#[test]
fn operator_precedence_complex_test() {
    let result = parse("2 + 3 * 4 - 5 / 5").unwrap().unwrap();
    assert_eq!(result, 13); // 2 + 12 - 1
}

#[test]
fn division_precedence_test() {
    let result = parse("10 / 2 / 5").unwrap().unwrap();
    assert_eq!(result, 1); // Left associative: (10/2)/5 = 5/5 = 1
}