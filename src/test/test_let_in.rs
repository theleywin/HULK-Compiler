use crate::ast_nodes::block::ExpressionList;
use crate::ast_nodes::expression::Expression;
use crate::ast_nodes::let_in::Assignment;
use crate::ast_nodes::program::Program;
use crate::ast_nodes::program::Statement;
use crate::semantic_analyzer::semantic_analyzer::SemanticAnalyzer;
use crate::tokens::OperatorToken;

//input: let number = 42, text = "The meaning of life is" in (text @ number)
#[test]
fn test_let_in_multiple_assignments() {
    let mut analyzer = SemanticAnalyzer::new();

    let assignments = vec![
        Assignment::new(
            "number".to_string(),
            Expression::new_number("42".to_string()),
        ),
        Assignment::new(
            "text".to_string(),
            Expression::new_string("The meaning of life is".to_string()),
        ),
    ];

    let body = Expression::new_binary_op(
        Expression::new_identifier("text".to_string()),
        OperatorToken::CONCAT,
        Expression::new_identifier("number".to_string()),
    );

    let let_in = Expression::new_let_in(assignments, body);

    let mut program = Program {
        statements: vec![Statement::new_expression(let_in)],
    };

    let result = analyzer.analyze(&mut program);
    assert!(
        result.is_ok(),
        "Semantic analysis should pass but got errors: {:?}",
        result.err()
    );
}

/*
input: let number = 42 in
         let text = "The meaning of life is" in
             (text @ number)
*/
#[test]
fn test_let_in_nested_flat() {
    let mut analyzer = SemanticAnalyzer::new();

    let inner_assignments = vec![Assignment::new(
        "text".to_string(),
        Expression::new_string("The meaning of life is".to_string()),
    )];

    let inner_body = Expression::new_binary_op(
        Expression::new_identifier("text".to_string()),
        OperatorToken::CONCAT,
        Expression::new_identifier("number".to_string()),
    );

    let inner_let_in = Expression::new_let_in(inner_assignments, inner_body);

    let outer_assignments = vec![Assignment::new(
        "number".to_string(),
        Expression::new_number("42".to_string()),
    )];

    let outer_let_in = Expression::new_let_in(outer_assignments, inner_let_in);

    let mut program = Program {
        statements: vec![Statement::new_expression(outer_let_in)],
    };

    let result = analyzer.analyze(&mut program);
    assert!(
        result.is_ok(),
        "Semantic analysis should pass but got errors: {:?}",
        result.err()
    );
}

/*
input: let number = 42 in (
        let text = "The meaning of life is" in (
                 (text @ number)
))
*/
#[test]
fn test_let_in_nested_parenthesized() {
    let mut analyzer = SemanticAnalyzer::new();

    let inner_assignments = vec![Assignment::new(
        "text".to_string(),
        Expression::new_string("The meaning of life is".to_string()),
    )];

    let inner_body = Expression::new_binary_op(
        Expression::new_identifier("text".to_string()),
        OperatorToken::CONCAT,
        Expression::new_identifier("number".to_string()),
    );

    let inner_let_in = Expression::new_let_in(inner_assignments, inner_body);

    let outer_assignments = vec![Assignment::new(
        "number".to_string(),
        Expression::new_number("42".to_string()),
    )];

    let outer_let_in = Expression::new_let_in(outer_assignments, inner_let_in);

    let mut program = Program {
        statements: vec![Statement::new_expression(outer_let_in)],
    };

    let result = analyzer.analyze(&mut program);
    assert!(
        result.is_ok(),
        "Semantic analysis should pass but got errors: {:?}",
        result.err()
    );
}

//input: let a = 20 in { let a = 42 in (a); a; }
#[test]
fn test_shadowing_in_block() {
    let mut analyzer = SemanticAnalyzer::new();

    let inner_assignments = vec![Assignment::new(
        "a".to_string(),
        Expression::new_number("42".to_string()),
    )];

    let inner_body = Expression::new_identifier("a".to_string());

    let inner_let_in = Expression::new_let_in(inner_assignments, inner_body);

    let outer_assignments = vec![Assignment::new(
        "a".to_string(),
        Expression::new_number("20".to_string()),
    )];

    let block_body = ExpressionList::new(vec![
        inner_let_in,
        Expression::new_identifier("a".to_string()),
    ]);

    let outer_body = Expression::new_code_block(block_body);

    let let_in = Expression::new_let_in(outer_assignments, outer_body);

    let mut program = Program {
        statements: vec![Statement::new_expression(let_in)],
    };

    let result = analyzer.analyze(&mut program);
    assert!(
        result.is_ok(),
        "Semantic analysis should pass but got errors: {:?}",
        result.err()
    );
}

//input: let a = 7, a = 7 * 6 in a
#[test]
fn test_multiple_assignments_same_scope() {
    let mut analyzer = SemanticAnalyzer::new();

    let assignments = vec![
        Assignment::new("a".to_string(), Expression::new_number("7".to_string())),
        Assignment::new(
            "a".to_string(),
            Expression::new_binary_op(
                Expression::new_number("7".to_string()),
                OperatorToken::MUL,
                Expression::new_number("6".to_string()),
            ),
        ),
    ];

    let body = Expression::new_identifier("a".to_string());

    let let_in = Expression::new_let_in(assignments, body);

    let mut program = Program {
        statements: vec![Statement::new_expression(let_in)],
    };

    let result = analyzer.analyze(&mut program);
    assert!(
        result.is_ok(),
        "Semantic analysis should pass but got errors: {:?}",
        result.err()
    );
}

//input: let a = 7 in let a = 7 * 6 in a
#[test]
fn test_nested_shadowing() {
    let mut analyzer = SemanticAnalyzer::new();

    let inner_assignments = vec![Assignment::new(
        "a".to_string(),
        Expression::new_binary_op(
            Expression::new_number("7".to_string()),
            OperatorToken::MUL,
            Expression::new_number("6".to_string()),
        ),
    )];

    let inner_body = Expression::new_identifier("a".to_string());

    let inner_let_in = Expression::new_let_in(inner_assignments, inner_body);

    let outer_assignments = vec![Assignment::new(
        "a".to_string(),
        Expression::new_number("7".to_string()),
    )];

    let outer_let_in = Expression::new_let_in(outer_assignments, inner_let_in);

    let mut program = Program {
        statements: vec![Statement::new_expression(outer_let_in)],
    };

    let result = analyzer.analyze(&mut program);
    assert!(
        result.is_ok(),
        "Semantic analysis should pass but got errors: {:?}",
        result.err()
    );
}

//input: let a = 0 in { a := "waos"; }
#[test]
fn test_destructive_assignment() {
    let mut analyzer = SemanticAnalyzer::new();

    let assignments = vec![Assignment::new(
        "a".to_string(),
        Expression::new_number("0".to_string()),
    )];

    let destructive_assign = Expression::new_destructive_assign(
        Expression::new_identifier("a".to_string()),
        Expression::new_string("waos".to_string()),
    );

    let block_body = ExpressionList::new(vec![destructive_assign]);
    let body = Expression::new_code_block(block_body);

    let let_in = Expression::new_let_in(assignments, body);

    let mut program = Program {
        statements: vec![Statement::new_expression(let_in)],
    };

    let result = analyzer.analyze(&mut program);
    assert!(
        result.is_ok(),
        "Semantic analysis should pass but got errors: {:?}",
        result.err()
    );
}


//input: let a = 0 in let b = a := 1 in { a; b; }
#[test]
fn test_destructive_assignment_in_nested_scope() {
    let mut analyzer = SemanticAnalyzer::new();

    let inner_assignments = vec![Assignment::new(
        "b".to_string(),
        Expression::new_destructive_assign(
            Expression::new_identifier("a".to_string()),
            Expression::new_number("1".to_string()),
        ),
    )];

    // Changed from print calls to simple identifiers
    let inner_block = ExpressionList::new(vec![
        Expression::new_identifier("a".to_string()),
        Expression::new_identifier("b".to_string()),
    ]);

    let inner_body = Expression::new_code_block(inner_block);

    let inner_let_in = Expression::new_let_in(inner_assignments, inner_body);

    let outer_assignments = vec![Assignment::new(
        "a".to_string(),
        Expression::new_number("0".to_string()),
    )];

    let outer_let_in = Expression::new_let_in(outer_assignments, inner_let_in);

    let mut program = Program {
        statements: vec![Statement::new_expression(outer_let_in)],
    };

    let result = analyzer.analyze(&mut program);
    assert!(
        result.is_ok(),
        "Semantic analysis should pass but got errors: {:?}",
        result.err()
    );
}