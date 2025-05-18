use hulk_compiler::ast_nodes::expression::Expression;
use hulk_compiler::ast_nodes::program::Statement;
use hulk_compiler::parser::ProgramParser;
use hulk_compiler::tokens::OperatorToken;

#[test]
fn test_let_in_with_if_else() {
    let input = r#"
        let a = 42 in if (a % 2 == 0) print("Even") else print("odd");
    "#;

    let program = ProgramParser::new().parse(input).unwrap();
    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::LetIn(let_in) = &**expr {
            // Verify assignment
            assert_eq!(let_in.assignments.len(), 1);
            let a_assign = &let_in.assignments[0];
            assert_eq!(a_assign.identifier, "a");
            verify_number_literal(&a_assign.expression, 42.0);

            // Verify if-else structure
            if let Expression::IfElse(if_else) = &*let_in.body {
                // Check condition: (a % 2 == 0)
                if let Expression::BinaryOp(eq_op) = &*if_else.condition {
                    assert_eq!(eq_op.operator, OperatorToken::EQ);
                    
                    // Verify modulo operation
                    if let Expression::BinaryOp(mod_op) = &*eq_op.left {
                        assert_eq!(mod_op.operator, OperatorToken::MOD);
                        verify_identifier(&mod_op.left, "a");
                        verify_number_literal(&mod_op.right, 2.0);
                    }
                    
                    // Verify right side of equality
                    verify_number_literal(&eq_op.right, 0.0);
                }

                // Verify "Even" print branch
                if let Expression::FunctionCall(then_print) = &*if_else.then_expression {
                    assert_eq!(then_print.function_name, "print");
                    verify_string_literal(&then_print.arguments[0], "Even");
                }

                // Verify "odd" print branch
                if let Expression::FunctionCall(else_print) = &*if_else.else_expression {
                    assert_eq!(else_print.function_name, "print");
                    verify_string_literal(&else_print.arguments[0], "odd");
                }
            } else {
                panic!("Expected if-else expression in let-in body");
            }
        } else {
            panic!("Expected let-in expression");
        }
    } else {
        panic!("Expected StatementExpression");
    }
}

// Helper functions
fn verify_number_literal(expr: &Expression, expected: f64) {
    if let Expression::Number(num) = expr {
        assert_eq!(num.value, expected);
    } else {
        panic!("Expected number literal {}, got {:?}", expected, expr);
    }
}

fn verify_identifier(expr: &Expression, expected: &str) {
    if let Expression::Identifier(id) = expr {
        assert_eq!(id.value, expected);
    } else {
        panic!("Expected identifier '{}', got {:?}", expected, expr);
    }
}

fn verify_string_literal(expr: &Expression, expected: &str) {
    if let Expression::Str(s) = expr {
        assert_eq!(s.value, expected);
    } else {
        panic!("Expected string literal '{}', got {:?}", expected, expr);
    }
}

#[test]
fn test_let_in_with_conditional_print() {
    let input = r#"let a = 42 in print(if (a % 2 == 0) "even" else "odd");"#;
    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();

    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::LetIn(let_in) = &**expr {
            // Verify assignment
            assert_eq!(let_in.assignments.len(), 1);
            let a_assign = &let_in.assignments[0];
            assert_eq!(a_assign.identifier, "a");
            verify_number_literal(&a_assign.expression, 42.0);

            // Verify print statement
            if let Expression::FunctionCall(print_call) = &*let_in.body {
                assert_eq!(print_call.function_name, "print");
                assert_eq!(print_call.arguments.len(), 1);
                
                // Verify if-else in print argument
                if let Expression::IfElse(if_else) = &print_call.arguments[0] {
                    // Check condition: (a % 2 == 0)
                    if let Expression::BinaryOp(eq_op) = &*if_else.condition {
                        assert_eq!(eq_op.operator, OperatorToken::EQ);
                        
                        // Verify modulo operation
                        if let Expression::BinaryOp(mod_op) = &*eq_op.left {
                            assert_eq!(mod_op.operator, OperatorToken::MOD);
                            verify_identifier(&mod_op.left, "a");
                            verify_number_literal(&mod_op.right, 2.0);
                        }
                        
                        // Verify comparison value
                        verify_number_literal(&eq_op.right, 0.0);
                    }

                    // Verify "even" branch
                    verify_string_literal(&if_else.then_expression, "even");
                    
                    // Verify "odd" branch
                    verify_string_literal(&if_else.else_expression, "odd");
                } else {
                    panic!("Expected if-else expression in print argument");
                }
            } else {
                panic!("Expected print call in let-in body");
            }
        } else {
            panic!("Expected let-in expression");
        }
    } else {
        panic!("Expected StatementExpression");
    }
}

#[test]
fn test_let_in_with_block_if_else() {
    let input = r#"
        let a = 42 in
            if (a % 2 == 0) {
                print(a);
                print("Even");
            }
            else print("Odd");
    "#;

    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();
    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::LetIn(let_in) = &**expr {
            // Verify assignment
            assert_eq!(let_in.assignments.len(), 1);
            let a_assign = &let_in.assignments[0];
            assert_eq!(a_assign.identifier, "a");
            verify_number_literal(&a_assign.expression, 42.0);

            // Verify if-else structure
            if let Expression::IfElse(if_else) = &*let_in.body {
                // Check condition: (a % 2 == 0)
                if let Expression::BinaryOp(eq_op) = &*if_else.condition {
                    assert_eq!(eq_op.operator, OperatorToken::EQ);
                    
                    // Verify modulo operation
                    if let Expression::BinaryOp(mod_op) = &*eq_op.left {
                        assert_eq!(mod_op.operator, OperatorToken::MOD);
                        verify_identifier(&mod_op.left, "a");
                        verify_number_literal(&mod_op.right, 2.0);
                    }
                    
                    // Verify comparison value
                    verify_number_literal(&eq_op.right, 0.0);
                }

                // Verify then block (code block with 2 statements)
                if let Expression::CodeBlock(then_block) = &*if_else.then_expression {
                    let expressions = &then_block.expression_list.expressions;
                    assert_eq!(expressions.len(), 2);

                    // Verify print(a)
                    if let Expression::FunctionCall(print_a) = &expressions[0] {
                        assert_eq!(print_a.function_name, "print");
                        verify_identifier(&print_a.arguments[0], "a");
                    }

                    // Verify print("Even")
                    if let Expression::FunctionCall(print_even) = &expressions[1] {
                        assert_eq!(print_even.function_name, "print");
                        verify_string_literal(&print_even.arguments[0], "Even");
                    }
                } else {
                    panic!("Expected code block in then branch");
                }

                // Verify else branch (single print)
                if let Expression::FunctionCall(print_odd) = &*if_else.else_expression {
                    assert_eq!(print_odd.function_name, "print");
                    verify_string_literal(&print_odd.arguments[0], "Odd");
                } else {
                    panic!("Expected print call in else branch");
                }
            } else {
                panic!("Expected if-else expression in let-in body");
            }
        } else {
            panic!("Expected let-in expression");
        }
    } else {
        panic!("Expected StatementExpression");
    }
}

#[test]
fn test_let_in_with_elif_chain() {
    let input = r#"
        let a = 42, let mod = a % 3 in
            print(
                if (mod == 0) "Magic"
                elif (mod % 3 == 1) "Woke"
                else "Dumb"
            );
    "#;

    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();
    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::LetIn(let_in) = &**expr {
            // Verify assignments
            assert_eq!(let_in.assignments.len(), 2);
            
            // Check a = 42
            let a_assign = &let_in.assignments[0];
            assert_eq!(a_assign.identifier, "a");
            verify_number_literal(&a_assign.expression, 42.0);

            // Check mod = a % 3
            let mod_assign = &let_in.assignments[1];
            assert_eq!(mod_assign.identifier, "mod");
            if let Expression::BinaryOp(mod_op) = &*mod_assign.expression {
                assert_eq!(mod_op.operator, OperatorToken::MOD);
                verify_identifier(&mod_op.left, "a");
                verify_number_literal(&mod_op.right, 3.0);
            }

            // Verify print with nested if-elif-else
            if let Expression::FunctionCall(print_call) = &*let_in.body {
                assert_eq!(print_call.function_name, "print");
                assert_eq!(print_call.arguments.len(), 1);
                
                // Main if-else structure
                if let Expression::IfElse(main_if) = &print_call.arguments[0] {
                    // First condition: mod == 0
                    verify_condition(&main_if.condition, "mod", 0.0);
                    verify_string_literal(&main_if.then_expression, "Magic");

                    // Elif branch (nested if-else)
                    if let Expression::IfElse(elif_if) = &*main_if.else_expression {
                        // Second condition: mod % 3 == 1
                        if let Expression::BinaryOp(cond_op) = &*elif_if.condition {
                            assert_eq!(cond_op.operator, OperatorToken::EQ);
                            
                            // mod % 3
                            if let Expression::BinaryOp(mod_op) = &*cond_op.left {
                                assert_eq!(mod_op.operator, OperatorToken::MOD);
                                verify_identifier(&mod_op.left, "mod");
                                verify_number_literal(&mod_op.right, 3.0);
                            }
                            
                            verify_number_literal(&cond_op.right, 1.0);
                        }
                        verify_string_literal(&elif_if.then_expression, "Woke");
                        verify_string_literal(&elif_if.else_expression, "Dumb");
                    } else {
                        panic!("Expected elif branch as nested if-else");
                    }
                } else {
                    panic!("Expected main if-else structure");
                }
            }
        }
    }
}

// Helper functions
fn verify_condition(expr: &Expression, var: &str, expected: f64) {
    if let Expression::BinaryOp(eq_op) = expr {
        assert_eq!(eq_op.operator, OperatorToken::EQ);
        verify_identifier(&eq_op.left, var);
        verify_number_literal(&eq_op.right, expected);
    } else {
        panic!("Expected equality condition");
    }
}