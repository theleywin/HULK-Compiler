use crate::ast_nodes::block::ExpressionList;
use crate::ast_nodes::binary_op::BinaryOpNode;
use crate::ast_nodes::expression::Expression;
use crate::ast_nodes::let_in::Assignment;
use crate::ast_nodes::let_in::LetInNode;
use crate::ast_nodes::literals::IdentifierNode;
use crate::ast_nodes::program::Program;
use crate::ast_nodes::program::Statement;
use crate::semantic_analyzer::semantic_analyzer::SemanticAnalyzer;
use crate::semantic_analyzer::semantic_errors::SemanticError;
use crate::tokens::OperatorToken;

//input: variable;
#[test]
fn test_undefined_variable_identifier() {
    let mut analyzer = SemanticAnalyzer::new();
    let mut program = Program {
        statements: vec![Statement::new_expression(Expression::Identifier(
            IdentifierNode::new("variable"),
        ))],
    };

    let result = analyzer.analyze(&mut program);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0],
        SemanticError::UndefinedIdentifier("variable".to_string())
    );
}

//input: a; let a = 5 in (a+a);
#[test]
fn test_undefined_variable_in_let_in() {
    let mut analyzer = SemanticAnalyzer::new();
    let mut program = Program {
        statements: vec![
            Statement::new_expression(Expression::Identifier(IdentifierNode::new("a"))),
            Statement::new_expression(Expression::LetIn(LetInNode::new(
                vec![Assignment::new(
                    "a".to_string(),
                    Expression::new_number("5".to_string()),
                )],
                Expression::BinaryOp(BinaryOpNode::new(
                    Expression::Identifier(IdentifierNode::new("a")),
                    OperatorToken::PLUS,
                    Expression::Identifier(IdentifierNode::new("a")),
                )),
            ))),
        ],
    };

    let result = analyzer.analyze(&mut program);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0],
        SemanticError::UndefinedIdentifier("a".to_string())
    );
}

//input: (((1 + 2) ^ 3) * 4) / 5
#[test]
fn test_nested_binary_operations() {
    let mut analyzer = SemanticAnalyzer::new();
    let mut program = Program {
        statements: vec![Statement::new_expression(Expression::BinaryOp(
            BinaryOpNode::new(
                Expression::BinaryOp(BinaryOpNode::new(
                    Expression::BinaryOp(BinaryOpNode::new(
                        Expression::BinaryOp(BinaryOpNode::new(
                            Expression::new_number("1".to_string()),
                            OperatorToken::PLUS,
                            Expression::new_number("2".to_string()),
                        )),
                        OperatorToken::POW,
                        Expression::new_number("3".to_string()),
                    )),
                    OperatorToken::MUL,
                    Expression::new_number("4".to_string()),
                )),
                OperatorToken::DIV,
                Expression::new_number("5".to_string()),
            ),
        ))],
    };

    let result = analyzer.analyze(&mut program);

    assert!(result.is_ok());
}

//input: "The message is \"Hello World\""
#[test]
fn test_string_literal() {
    let mut analyzer = SemanticAnalyzer::new();
    let mut program = Program {
        statements: vec![Statement::new_expression(Expression::new_string(
            "The message is \"Hello World\"".to_string(),
        ))],
    };

    let result = analyzer.analyze(&mut program);

    assert!(result.is_ok());
}

//input: "The meaning of life is " @ 42
#[test]
fn test_string_concatenation_with_number() {
    let mut analyzer = SemanticAnalyzer::new();
    let mut program = Program {
        statements: vec![Statement::new_expression(Expression::BinaryOp(
            BinaryOpNode::new(
                Expression::new_string("The meaning of life is ".to_string()),
                OperatorToken::CONCAT,
                Expression::new_string("42".to_string()),
            ),
        ))],
    };

    let result = analyzer.analyze(&mut program);

    assert!(result.is_ok());
}

//input: "{42;sin(PI/2);\"Hello World\";}"
#[test]
fn test_block_with_function_call_errors() {
    let mut analyzer = SemanticAnalyzer::new();
    let mut program = Program {
        statements: vec![Statement::new_expression(
            Expression::new_code_block(ExpressionList::new(vec![
                Expression::new_number("42".to_string()),
                Expression::new_function_call(
                    "sin".to_string(),
                    vec![Expression::new_binary_op(
                        Expression::new_identifier("PI".to_string()),
                        OperatorToken::DIV,
                        Expression::new_number("2".to_string()),
                    )],
                ),
                Expression::new_string("Hello World".to_string()),
            ]))
        )],
    };

    let result = analyzer.analyze(&mut program);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    
    assert_eq!(
        errors.iter().find(|e| matches!(e, SemanticError::UndeclaredFunction(_))),
        Some(&SemanticError::UndeclaredFunction("sin".to_string()))
    );
}