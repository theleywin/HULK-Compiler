use hulk_compiler::ast_nodes::expression::Expression;
use hulk_compiler::ast_nodes::program::Statement;
use hulk_compiler::parser::ProgramParser;
use hulk_compiler::tokens::OperatorToken;

#[test]
fn test_arrow_function_definition() {
    let input = "function tan(x) => sin(x) / cos(x);";
    let program = ProgramParser::new().parse(input).unwrap();

    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementFunctionDef(func_def) = &program.statements[0] {
        // Verify function metadata
        assert_eq!(func_def.name, "tan");
        assert_eq!(func_def.params, vec!["x"]);
        
        // Verify function body structure
        if let Expression::BinaryOp(div_op) = &func_def.body {
            assert_eq!(div_op.operator, OperatorToken::DIV);
            
            // Verify left operand: sin(x)
            if let Expression::FunctionCall(sin_call) = &*div_op.left {
                assert_eq!(sin_call.function_name, "sin");
                assert_eq!(sin_call.arguments.len(), 1);
                assert!(matches!(&sin_call.arguments[0], Expression::Identifier(id) if id.value == "x"));
            } else {
                panic!("Expected sin function call in numerator");
            }
            
            // Verify right operand: cos(x)
            if let Expression::FunctionCall(cos_call) = &*div_op.right {
                assert_eq!(cos_call.function_name, "cos");
                assert_eq!(cos_call.arguments.len(), 1);
                assert!(matches!(&cos_call.arguments[0], Expression::Identifier(id) if id.value == "x"));
            } else {
                panic!("Expected cos function call in denominator");
            }
        } else {
            panic!("Expected division operation in function body");
        }
    } else {
        panic!("Expected function definition statement");
    }
}

#[test]
fn test_trig_function_chain() {
    let input = r#"
        function cot(x) => 1 / tan(x);
        function tan(x) => sin(x) / cos(x);
        print(tan(PI) ^ 2 + cot(PI) ^ 2);
    "#;

    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();
    assert_eq!(program.statements.len(), 3);

    // Verify cotangent function
    if let Statement::StatementFunctionDef(cot_def) = &program.statements[0] {
        assert_eq!(cot_def.name, "cot");
        assert_eq!(cot_def.params, vec!["x"]);
        
        if let Expression::BinaryOp(div_op) = &cot_def.body {
            assert_eq!(div_op.operator, OperatorToken::DIV);
            assert!(matches!(&*div_op.left, Expression::Number(n) if n.value == 1.0));
            
            if let Expression::FunctionCall(tan_call) = &*div_op.right {
                assert_eq!(tan_call.function_name, "tan");
                assert!(matches!(&tan_call.arguments[0], Expression::Identifier(id) if id.value == "x"));
            }
        }
    }

    // Verify tangent function
    if let Statement::StatementFunctionDef(tan_def) = &program.statements[1] {
        assert_eq!(tan_def.name, "tan");
        assert_eq!(tan_def.params, vec!["x"]);
        
        if let Expression::BinaryOp(div_op) = &tan_def.body {
            assert_eq!(div_op.operator, OperatorToken::DIV);
            
            if let Expression::FunctionCall(sin_call) = &*div_op.left {
                assert_eq!(sin_call.function_name, "sin");
            }
            
            if let Expression::FunctionCall(cos_call) = &*div_op.right {
                assert_eq!(cos_call.function_name, "cos");
            }
        }
    }

    // Verify print expression
    if let Statement::StatementExpression(print_expr) = &program.statements[2] {
        if let Expression::FunctionCall(print_call) = &**print_expr {
            assert_eq!(print_call.function_name, "print");
            
            if let Expression::BinaryOp(add_op) = &print_call.arguments[0] {
                assert_eq!(add_op.operator, OperatorToken::PLUS);
                
                // Verify left exponent: tan(PI)**2
                if let Expression::BinaryOp(tan_pow) = &*add_op.left {
                    assert_eq!(tan_pow.operator, OperatorToken::POW);
                    verify_function_call(&tan_pow.left, "tan", "PI");
                    verify_number_literal(&tan_pow.right, 2.0);
                }
                
                // Verify right exponent: cot(PI)**2
                if let Expression::BinaryOp(cot_pow) = &*add_op.right {
                    assert_eq!(cot_pow.operator, OperatorToken::POW);
                    verify_function_call(&cot_pow.left, "cot", "PI");
                    verify_number_literal(&cot_pow.right, 2.0);
                }
            }
        }
    }
}

// Helper functions
fn verify_function_call(expr: &Expression, expected_fn: &str, expected_arg: &str) {
    if let Expression::FunctionCall(call) = expr {
        assert_eq!(call.function_name, expected_fn);
        assert_eq!(call.arguments.len(), 1);
        assert!(matches!(&call.arguments[0], Expression::Identifier(id) if id.value == expected_arg));
    } else {
        panic!("Expected function call to {}", expected_fn);
    }
}

fn verify_number_literal(expr: &Expression, expected_value: f64) {
    if let Expression::Number(num) = expr {
        assert_eq!(num.value, expected_value);
    } else {
        panic!("Expected number literal {}", expected_value);
    }
}

#[test]
fn test_function_with_code_block() {
    let input = r#"
        function operate(x, y) {
            print(x + y);
            print(x - y);
            print(x * y);
            print(x / y);
        }
    "#;

    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();
    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementFunctionDef(func_def) = &program.statements[0] {
        assert_eq!(func_def.name, "operate");
        assert_eq!(func_def.params, vec!["x", "y"]);

        // Verify code block body
        if let Expression::CodeBlock(block) = &func_def.body {
            let expressions = &block.expression_list.expressions;
            assert_eq!(expressions.len(), 4);

            // Define expected operators for each print statement
            let expected_operators = [
                OperatorToken::PLUS,
                OperatorToken::MINUS,
                OperatorToken::MUL,
                OperatorToken::DIV,
            ];

            // Verify each print statement and its operation
            for (i, expr) in expressions.iter().enumerate() {
                if let Expression::FunctionCall(print_call) = expr {
                    assert_eq!(print_call.function_name, "print");
                    assert_eq!(print_call.arguments.len(), 1);

                    // Verify the binary operation
                    if let Expression::BinaryOp(bin_op) = &print_call.arguments[0] {
                        assert_eq!(bin_op.operator, expected_operators[i]);
                        
                        // Verify left operand is x
                        assert!(matches!(&*bin_op.left, Expression::Identifier(id) if id.value == "x"));
                        
                        // Verify right operand is y
                        assert!(matches!(&*bin_op.right, Expression::Identifier(id) if id.value == "y"));
                    } else {
                        panic!("Expected binary operation in print argument at index {}", i);
                    }
                } else {
                    panic!("Expected print call at index {}", i);
                }
            }
        } else {
            panic!("Expected code block in function body");
        }
    } else {
        panic!("Expected function definition statement");
    }
}
