use hulk_compiler::ast_nodes::expression::Expression;
use hulk_compiler::ast_nodes::program::Statement;
use hulk_compiler::parser::ProgramParser;
use hulk_compiler::tokens::OperatorToken;

#[test]
fn test_code_block_with_multiple_prints() {
    let input = r#"
    {
        print(42);
        print(sin(PI/2));
        print("Hello World");
    }
    "#;

    let program = ProgramParser::new().parse(input).unwrap();
    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::CodeBlock(block) = &**expr {
            let expressions = &block.expression_list.expressions;
            assert_eq!(expressions.len(), 3);

            // First print statement: print(42)
            if let Expression::FunctionCall(print1) = &expressions[0] {
                assert_eq!(print1.function_name, "print");
                assert!(matches!(&print1.arguments[0], Expression::Number(n) if n.value == 42.0));
            } else {
                panic!("Expected first print statement");
            }

            // Second print statement: print(sin(PI/2))
            if let Expression::FunctionCall(print2) = &expressions[1] {
                assert_eq!(print2.function_name, "print");
                if let Expression::FunctionCall(sin_call) = &print2.arguments[0] {
                    assert_eq!(sin_call.function_name, "sin");
                    if let Expression::BinaryOp(div_op) = &sin_call.arguments[0] {
                        assert_eq!(div_op.operator, OperatorToken::DIV);
                        assert!(
                            matches!(&*div_op.left, Expression::Identifier(id) if id.value == "PI")
                        );
                        assert!(matches!(&*div_op.right, Expression::Number(n) if n.value == 2.0));
                    }
                } else {
                    panic!("Expected sin function call");
                }
            } else {
                panic!("Expected second print statement");
            }

            // Third print statement: print("Hello World")
            if let Expression::FunctionCall(print3) = &expressions[2] {
                assert_eq!(print3.function_name, "print");
                if let Expression::Str(str_lit) = &print3.arguments[0] {
                    assert_eq!(str_lit.value, "Hello World");
                } else {
                    panic!("Expected string literal in third print");
                }
            } else {
                panic!("Expected third print statement");
            }
        } else {
            panic!("Expected code block expression");
        }
    } else {
        panic!("Expected StatementExpression");
    }
}
