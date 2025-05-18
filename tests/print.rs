use hulk_compiler::ast_nodes::expression::Expression;
use hulk_compiler::ast_nodes::program::Statement;
use hulk_compiler::parser::ProgramParser;
use hulk_compiler::tokens::OperatorToken;

#[test]
fn test_simple_print() {
    // Test input: print(42);
    let input = "print(42);";

    // Parse the input
    let program = hulk_compiler::parser::ProgramParser::new()
        .parse(input)
        .unwrap();

    // Verify we have 1 statement
    assert_eq!(program.statements.len(), 1);

    // Check the statement structure
    match &program.statements[0] {
        Statement::StatementExpression(expr) => {
            // Verify it's a function call
            if let Expression::FunctionCall(func_call) = &**expr {
                assert_eq!(func_call.function_name, "print");

                // Verify the argument
                assert_eq!(func_call.arguments.len(), 1);
                if let Expression::Number(num_node) = &func_call.arguments[0] {
                    assert_eq!(num_node.value, 42.0);
                } else {
                    panic!("Expected number literal in print argument");
                }
            } else {
                panic!("Expected function call in expression");
            }
        }
        _ => panic!("Expected StatementExpression variant"),
    }
}

#[test]
fn test_nested_arithmetic() {
    let input = "print((((1 + 2) ^ 3) * 4) / 5);";
    let program = ProgramParser::new().parse(input).unwrap();

    assert_eq!(program.statements.len(), 1);

    // Unwrap the print statement's argument
    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::FunctionCall(func_call) = &**expr {
            assert_eq!(func_call.function_name, "print");

            // Start checking the nested arithmetic operations
            if let Expression::BinaryOp(div_op) = &func_call.arguments[0] {
                assert_eq!(div_op.operator, OperatorToken::DIV);

                // Check left side of division (multiplication)
                if let Expression::BinaryOp(mul_op) = &*div_op.left {
                    assert_eq!(mul_op.operator, OperatorToken::MUL);

                    // Check left side of multiplication (exponentiation)
                    if let Expression::BinaryOp(pow_op) = &*mul_op.left {
                        assert_eq!(pow_op.operator, OperatorToken::POW);

                        // Check base of exponentiation (addition)
                        if let Expression::BinaryOp(add_op) = &*pow_op.left {
                            assert_eq!(add_op.operator, OperatorToken::PLUS);
                            assert!(
                                matches!(&*add_op.left, Expression::Number(n) if n.value == 1.0)
                            );
                            assert!(
                                matches!(&*add_op.right, Expression::Number(n) if n.value == 2.0)
                            );
                        }

                        // Check exponent
                        assert!(matches!(&*pow_op.right, Expression::Number(n) if n.value == 3.0));
                    }

                    // Check multiplier
                    assert!(matches!(&*mul_op.right, Expression::Number(n) if n.value == 4.0));
                }

                // Check divisor
                assert!(matches!(&*div_op.right, Expression::Number(n) if n.value == 5.0));
            } else {
                panic!("Expected division operation at root");
            }
        } else {
            panic!("Expected function call in print statement");
        }
    } else {
        panic!("Expected StatementExpression");
    }
}

#[test]
fn test_string_literal() {
    let input = "print(\"Hello World\");";
    let program = hulk_compiler::parser::ProgramParser::new()
        .parse(input)
        .unwrap();

    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::FunctionCall(func_call) = &**expr {
            assert_eq!(func_call.function_name, "print");
            assert_eq!(func_call.arguments.len(), 1);

            if let Expression::Str(str_lit) = &func_call.arguments[0] {
                assert_eq!(str_lit.value, "Hello World");
            } else {
                panic!("Expected string literal in print argument");
            }
        } else {
            panic!("Expected function call in expression");
        }
    } else {
        panic!("Expected StatementExpression");
    }
}

#[test]
fn test_escaped_string() {
    let input = "print(\"The message is \\\"Hello World\\\"\");";
    let program = hulk_compiler::parser::ProgramParser::new()
        .parse(input)
        .unwrap();

    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::FunctionCall(func_call) = &**expr {
            assert_eq!(func_call.function_name, "print");
            assert_eq!(func_call.arguments.len(), 1);

            if let Expression::Str(str_lit) = &func_call.arguments[0] {
                // The parsed string should contain actual quotes, not escape characters
                assert_eq!(str_lit.value, "The message is \"Hello World\"");
            } else {
                panic!("Expected string literal in print argument");
            }
        } else {
            panic!("Expected function call in expression");
        }
    } else {
        panic!("Expected StatementExpression");
    }
}
