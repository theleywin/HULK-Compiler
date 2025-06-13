use crate::ast_nodes::expression::Expression;
use crate::ast_nodes::block::ExpressionList;
use crate::ast_nodes::function_def::{FunctionDefNode, FunctionParams};
use crate::ast_nodes::program::Program;
use crate::ast_nodes::program::Statement;
use crate::semantic_analyzer::semantic_analyzer::SemanticAnalyzer;
use crate::tokens::OperatorToken;
use crate::ast_nodes::let_in::Assignment;

//input: function divide ( x : Number , y : Number ) : Number => x / y ;
#[test]
fn test_function_definition_and_call() {
    let mut analyzer = SemanticAnalyzer::new();

    let params = vec![
        FunctionParams::new("x".to_string(), "Number".to_string()),
        FunctionParams::new("y".to_string(), "Number".to_string()),
    ];

    let body = Expression::new_binary_op(
        Expression::new_identifier("x".to_string()),
        OperatorToken::DIV,
        Expression::new_identifier("y".to_string()),
    );

    let func_def =
        FunctionDefNode::new_expr("divide".to_string(), params, "Number".to_string(), body);

    let func_call = Expression::new_function_call(
        "divide".to_string(),
        vec![
            Expression::new_number("10".to_string()),
            Expression::new_number("2".to_string()),
        ],
    );

    let mut program = Program {
        statements: vec![
            Statement::StatementFunctionDef(Box::new(func_def)),
            Statement::new_expression(func_call),
        ],
    };

    let result = analyzer.analyze(&mut program);
    assert!(
        result.is_ok(),
        "Semantic analysis should pass but got errors: {:?}",
        result.err()
    );
}

/*Input:
function operate(x:Number, y:Number):Number {
    x + y;
    x - y;
    x * y;
    x / y;
}
 */
#[test]
fn test_function_with_block_body() {
    let mut analyzer = SemanticAnalyzer::new();

    let params = vec![
        FunctionParams::new("x".to_string(), "Number".to_string()),
        FunctionParams::new("y".to_string(), "Number".to_string()),
    ];

    let expressions = vec![
        Expression::new_binary_op(
            Expression::new_identifier("x".to_string()),
            OperatorToken::PLUS,
            Expression::new_identifier("y".to_string()),
        ),
        Expression::new_binary_op(
            Expression::new_identifier("x".to_string()),
            OperatorToken::MINUS,
            Expression::new_identifier("y".to_string()),
        ),
        Expression::new_binary_op(
            Expression::new_identifier("x".to_string()),
            OperatorToken::MUL,
            Expression::new_identifier("y".to_string()),
        ),
        Expression::new_binary_op(
            Expression::new_identifier("x".to_string()),
            OperatorToken::DIV,
            Expression::new_identifier("y".to_string()),
        ),
    ];

    let body = Expression::new_code_block(ExpressionList::new(expressions));

    let func_def =
        FunctionDefNode::new_expr("operate".to_string(), params, "Number".to_string(), body);

    let mut program = Program {
        statements: vec![Statement::StatementFunctionDef(Box::new(func_def))],
    };

    let result = analyzer.analyze(&mut program);
    assert!(
        result.is_ok(),
        "Semantic analysis should pass but got errors: {:?}",
        result.err()
    );
}

/*input:
function gcd(a: Number, b: Number): Number {
    if (b == 0) {
        a;
    } else {
        gcd(b, a % b);
    }
}
 */
#[test]
fn test_recursive_gcd_function() {
    let mut analyzer = SemanticAnalyzer::new();
    
    let params = vec![
        FunctionParams::new("a".to_string(), "Number".to_string()),
        FunctionParams::new("b".to_string(), "Number".to_string()),
    ];
    
    let condition = Expression::new_binary_op(
        Expression::new_identifier("b".to_string()),
        OperatorToken::EQ,
        Expression::new_number("0".to_string()),
    );
    
    let then_block = Expression::new_identifier("a".to_string());
    
    let else_block = Expression::new_function_call(
        "gcd".to_string(),
        vec![
            Expression::new_identifier("b".to_string()),
            Expression::new_binary_op(
                Expression::new_identifier("a".to_string()),
                OperatorToken::MOD,
                Expression::new_identifier("b".to_string()),
            ),
        ],
    );
    
    let body = Expression::new_if_else(
        condition,
        then_block,
        vec![(None, else_block)],
    );
    
    let func_def = FunctionDefNode::new_expr(
        "gcd".to_string(),
        params,
        "Number".to_string(),
        body,
    );
    
    let mut program = Program {
        statements: vec![
            Statement::StatementFunctionDef(Box::new(func_def)),
        ],
    };

    let result = analyzer.analyze(&mut program);
    assert!(result.is_ok(), "Semantic analysis should pass but got errors: {:?}", result.err());
}


/*Input:
function loca(a: Number): String {
    let b = 5 in (b @ "Hello world");
}

loca(3);
 */
#[test]
fn test_function_with_let_in_and_concat() {
    let mut analyzer = SemanticAnalyzer::new();
    
    let params = vec![
        FunctionParams::new("a".to_string(), "Number".to_string()),
    ];
    
    let assignments = vec![
        Assignment::new(
            "b".to_string(),
            Expression::new_number("5".to_string()),
        )
    ];
    
    let body = Expression::new_binary_op(
        Expression::new_identifier("b".to_string()),
        OperatorToken::CONCAT,
        Expression::new_string("Hello world".to_string()),
    );
    
    let let_in = Expression::new_let_in(assignments, body);
    
    let func_def = FunctionDefNode::new_expr(
        "loca".to_string(),
        params,
        "String".to_string(),
        let_in,
    );
    
    let mut program = Program {
        statements: vec![
            Statement::StatementFunctionDef(Box::new(func_def)),
        ],
    };

    let result = analyzer.analyze(&mut program);
    assert!(result.is_ok(), "Semantic analysis should pass but got errors: {:?}", result.err());
}