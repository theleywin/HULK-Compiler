use hulk_compiler::ast_nodes::expression::Expression;
use hulk_compiler::ast_nodes::program::Statement;
use hulk_compiler::parser::ProgramParser;
use hulk_compiler::tokens::OperatorToken;

#[test]
fn test_complex_math_expression() {
    let input = "print(sin(2 * PI) ^ 2 + cos(3 * PI / log(4, 64)));";
    let program = ProgramParser::new().parse(input).unwrap();

    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::FunctionCall(print_call) = &**expr {
            assert_eq!(print_call.function_name, "print");
            assert_eq!(print_call.arguments.len(), 1);

            // Check the addition operation: (sin(...)^2) + (cos(...))
            if let Expression::BinaryOp(add_op) = &print_call.arguments[0] {
                assert_eq!(add_op.operator, OperatorToken::PLUS);

                // Left side: sin(2*PI)^2
                if let Expression::BinaryOp(pow_op) = &*add_op.left {
                    assert_eq!(pow_op.operator, OperatorToken::POW);

                    // Verify sin(2*PI)
                    if let Expression::FunctionCall(sin_call) = &*pow_op.left {
                        assert_eq!(sin_call.function_name, "sin");
                        assert_eq!(sin_call.arguments.len(), 1);
                        
                        // Verify 2*PI argument
                        if let Expression::BinaryOp(mul_op) = &sin_call.arguments[0] {
                            assert_eq!(mul_op.operator, OperatorToken::MUL);
                            assert!(matches!(&*mul_op.left, Expression::Number(n) if n.value == 2.0));
                            assert!(matches!(&*mul_op.right, Expression::Identifier(id) if id.value == "PI"));
                        }
                    }
                    
                    // Verify exponent 2
                    assert!(matches!(&*pow_op.right, Expression::Number(n) if n.value == 2.0));
                }

                // Right side: cos(3*PI/log(4,64))
                if let Expression::FunctionCall(cos_call) = &*add_op.right {
                    assert_eq!(cos_call.function_name, "cos");
                    assert_eq!(cos_call.arguments.len(), 1);
                    
                    // Verify division operation: 3*PI / log(4,64)
                    if let Expression::BinaryOp(div_op) = &cos_call.arguments[0] {
                        assert_eq!(div_op.operator, OperatorToken::DIV);
                        
                        // Verify numerator: 3*PI
                        if let Expression::BinaryOp(mul_op) = &*div_op.left {
                            assert_eq!(mul_op.operator, OperatorToken::MUL);
                            assert!(matches!(&*mul_op.left, Expression::Number(n) if n.value == 3.0));
                            assert!(matches!(&*mul_op.right, Expression::Identifier(id) if id.value == "PI"));
                        }
                        
                        // Verify denominator: log(4,64)
                        if let Expression::FunctionCall(log_call) = &*div_op.right {
                            assert_eq!(log_call.function_name, "log");
                            assert_eq!(log_call.arguments.len(), 2);
                            assert!(matches!(&log_call.arguments[0], Expression::Number(n) if n.value == 4.0));
                            assert!(matches!(&log_call.arguments[1], Expression::Number(n) if n.value == 64.0));
                        }
                    }
                }
            }
        } else {
            panic!("Expected print function call");
        }
    } else {
        panic!("Expected StatementExpression");
    }
}

#[test]
fn test_exponentiation_associativity() {
    let input = "5^3^2^1;";
    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();

    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        // Should parse as 5^(3^(2^1))
        if let Expression::BinaryOp(op1) = &**expr {
            assert_eq!(op1.operator, OperatorToken::POW);
            
            // Left operand: 5
            assert!(matches!(&*op1.left, Expression::Number(n) if n.value == 5.0));
            
            // Right operand: 3^(2^1)
            if let Expression::BinaryOp(op2) = &*op1.right {
                assert_eq!(op2.operator, OperatorToken::POW);
                
                // Left operand: 3
                assert!(matches!(&*op2.left, Expression::Number(n) if n.value == 3.0));
                
                // Right operand: 2^1
                if let Expression::BinaryOp(op3) = &*op2.right {
                    assert_eq!(op3.operator, OperatorToken::POW);
                    
                    // Left operand: 2
                    assert!(matches!(&*op3.left, Expression::Number(n) if n.value == 2.0));
                    
                    // Right operand: 1
                    assert!(matches!(&*op3.right, Expression::Number(n) if n.value == 1.0));
                } else {
                    panic!("Expected third level exponentiation");
                }
            } else {
                panic!("Expected second level exponentiation");
            }
        } else {
            panic!("Expected root level exponentiation");
        }
    } else {
        panic!("Expected StatementExpression");
    }
}