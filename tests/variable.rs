use hulk_compiler::ast_nodes::expression::Expression;
use hulk_compiler::ast_nodes::program::Statement;
use hulk_compiler::parser::ProgramParser;
use hulk_compiler::tokens::OperatorToken;

#[test]
fn test_let_in_string_assignment() {
    let input = r#"let msg = "Hello World" in print(msg);"#;
    let program = ProgramParser::new().parse(input).unwrap();

    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::LetIn(let_in) = &**expr {
            // Verify assignment
            assert_eq!(let_in.assignments.len(), 1);
            let assignment = &let_in.assignments[0];
            assert_eq!(assignment.identifier, "msg");
            
            // Verify string literal value
            if let Expression::Str(str_lit) = &*assignment.expression {
                assert_eq!(str_lit.value, "Hello World");
            } else {
                panic!("Expected string literal in assignment");
            }

            // Verify print statement in body
            if let Expression::FunctionCall(print_call) = &*let_in.body {
                assert_eq!(print_call.function_name, "print");
                assert_eq!(print_call.arguments.len(), 1);
                
                // Verify printed identifier
                if let Expression::Identifier(ident) = &print_call.arguments[0] {
                    assert_eq!(ident.value, "msg");
                } else {
                    panic!("Expected identifier in print arguments");
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
fn test_let_in_multiple_assignments() {
    let input = r#"
        let number = 42, text = "The meaning of life is" in
            print(text @ number);
    "#;
    
    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();
    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::LetIn(let_in) = &**expr {
            // Verify assignments
            assert_eq!(let_in.assignments.len(), 2);
            
            // Check number assignment
            let number_assignment = &let_in.assignments[0];
            assert_eq!(number_assignment.identifier, "number");
            if let Expression::Number(num) = &*number_assignment.expression {
                assert_eq!(num.value, 42.0);
            } else {
                panic!("Expected number literal in first assignment");
            }

            // Check text assignment
            let text_assignment = &let_in.assignments[1];
            assert_eq!(text_assignment.identifier, "text");
            if let Expression::Str(str_lit) = &*text_assignment.expression {
                assert_eq!(str_lit.value, "The meaning of life is");
            } else {
                panic!("Expected string literal in second assignment");
            }

            // Verify print operation
            if let Expression::FunctionCall(print_call) = &*let_in.body {
                assert_eq!(print_call.function_name, "print");
                assert_eq!(print_call.arguments.len(), 1);
                
                // Verify concatenation operator
                if let Expression::BinaryOp(concat_op) = &print_call.arguments[0] {
                    assert_eq!(concat_op.operator, OperatorToken::CONCAT);
                    
                    // Verify left operand (text identifier)
                    if let Expression::Identifier(text_id) = &*concat_op.left {
                        assert_eq!(text_id.value, "text");
                    }
                    
                    // Verify right operand (number identifier)
                    if let Expression::Identifier(number_id) = &*concat_op.right {
                        assert_eq!(number_id.value, "number");
                    }
                } else {
                    panic!("Expected concatenation operator @");
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
fn test_nested_let_in_assignments() {
    let input = r#"
        let number = 42 in
            let text = "The meaning of life is" in
                print(text @ number);
    "#;

    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();
    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(outer_expr) = &program.statements[0] {
        // Outer let-in: number = 42
        if let Expression::LetIn(outer_let) = &**outer_expr {
            assert_eq!(outer_let.assignments.len(), 1);
            
            // Verify number assignment
            let number_assign = &outer_let.assignments[0];
            assert_eq!(number_assign.identifier, "number");
            verify_number_literal(&number_assign.expression, 42.0);

            // Inner let-in: text = "..."
            if let Expression::LetIn(inner_let) = &*outer_let.body {
                assert_eq!(inner_let.assignments.len(), 1);
                
                // Verify text assignment
                let text_assign = &inner_let.assignments[0];
                assert_eq!(text_assign.identifier, "text");
                verify_string_literal(&text_assign.expression, "The meaning of life is");

                // Verify print operation
                if let Expression::FunctionCall(print_call) = &*inner_let.body {
                    assert_eq!(print_call.function_name, "print");
                    assert_eq!(print_call.arguments.len(), 1);
                    
                    // Verify concatenation operation
                    if let Expression::BinaryOp(concat_op) = &print_call.arguments[0] {
                        assert_eq!(concat_op.operator, OperatorToken::CONCAT);
                        
                        // Verify text identifier
                        verify_identifier(&concat_op.left, "text");
                        
                        // Verify number identifier
                        verify_identifier(&concat_op.right, "number");
                    } else {
                        panic!("Expected @ operator in print argument");
                    }
                } else {
                    panic!("Expected print call in inner let-in body");
                }
            } else {
                panic!("Expected inner let-in expression");
            }
        } else {
            panic!("Expected outer let-in expression");
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

fn verify_string_literal(expr: &Expression, expected: &str) {
    if let Expression::Str(s) = expr {
        assert_eq!(s.value, expected);
    } else {
        panic!("Expected string literal '{}', got {:?}", expected, expr);
    }
}

fn verify_identifier(expr: &Expression, expected: &str) {
    if let Expression::Identifier(id) = expr {
        assert_eq!(id.value, expected);
    } else {
        panic!("Expected identifier '{}', got {:?}", expected, expr);
    }
}

#[test]
fn test_parenthesized_let_in_nesting() {
    let input = r#"
        let number = 42 in (
            let text = "The meaning of life is" in (
                print(text @ number)
            )
        );
    "#;

    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();
    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(outer_expr) = &program.statements[0] {
        // Outer let-in: number = 42
        if let Expression::LetIn(outer_let) = &**outer_expr {
            assert_eq!(outer_let.assignments.len(), 1);
            
            // Verify number assignment
            let number_assign = &outer_let.assignments[0];
            assert_eq!(number_assign.identifier, "number");
            verify_number_literal(&number_assign.expression, 42.0);

            // Parenthesized inner let-in (should parse as normal)
            if let Expression::LetIn(inner_let) = &*outer_let.body {
                assert_eq!(inner_let.assignments.len(), 1);
                
                // Verify text assignment
                let text_assign = &inner_let.assignments[0];
                assert_eq!(text_assign.identifier, "text");
                verify_string_literal(&text_assign.expression, "The meaning of life is");

                // Parenthesized print statement
                if let Expression::FunctionCall(print_call) = &*inner_let.body {
                    assert_eq!(print_call.function_name, "print");
                    assert_eq!(print_call.arguments.len(), 1);
                    
                    // Verify concatenation operation
                    if let Expression::BinaryOp(concat_op) = &print_call.arguments[0] {
                        assert_eq!(concat_op.operator, OperatorToken::CONCAT);
                        
                        // Verify text identifier
                        verify_identifier(&concat_op.left, "text");
                        
                        // Verify number identifier (from outer scope)
                        verify_identifier(&concat_op.right, "number");
                    } else {
                        panic!("Expected @ operator in print argument");
                    }
                } else {
                    panic!("Expected print call in inner let-in body");
                }
            } else {
                panic!("Expected inner let-in expression after parentheses");
            }
        } else {
            panic!("Expected outer let-in expression");
        }
    } else {
        panic!("Expected StatementExpression");
    }
}

#[test]
fn test_let_in_variable_reference() {
    let input = "let a = 6, b = a * 7 in print(b);";
    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();

    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::LetIn(let_in) = &**expr {
            // Verify both assignments
            assert_eq!(let_in.assignments.len(), 2);

            // First assignment: a = 6
            let a_assignment = &let_in.assignments[0];
            assert_eq!(a_assignment.identifier, "a");
            verify_number_literal(&a_assignment.expression, 6.0);

            // Second assignment: b = a * 7
            let b_assignment = &let_in.assignments[1];
            assert_eq!(b_assignment.identifier, "b");
            if let Expression::BinaryOp(mul_op) = &*b_assignment.expression {
                assert_eq!(mul_op.operator, OperatorToken::MUL);
                
                // Verify left operand is 'a' reference
                verify_identifier(&mul_op.left, "a");
                
                // Verify right operand is number 7
                verify_number_literal(&mul_op.right, 7.0);
            } else {
                panic!("Expected multiplication in b assignment");
            }

            // Verify print body
            if let Expression::FunctionCall(print_call) = &*let_in.body {
                assert_eq!(print_call.function_name, "print");
                assert_eq!(print_call.arguments.len(), 1);
                
                // Verify printed identifier 'b'
                verify_identifier(&print_call.arguments[0], "b");
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
fn test_nested_let_in_variable_reference() {
    let input = r#"
        let a = 6 in
            let b = a * 7 in
                print(b);
    "#;

    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();
    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(outer_expr) = &program.statements[0] {
        // Outer let-in: a = 6
        if let Expression::LetIn(outer_let) = &**outer_expr {
            assert_eq!(outer_let.assignments.len(), 1);
            
            // Verify a assignment
            let a_assign = &outer_let.assignments[0];
            assert_eq!(a_assign.identifier, "a");
            verify_number_literal(&a_assign.expression, 6.0);

            // Inner let-in: b = a * 7
            if let Expression::LetIn(inner_let) = &*outer_let.body {
                assert_eq!(inner_let.assignments.len(), 1);
                
                // Verify b assignment
                let b_assign = &inner_let.assignments[0];
                assert_eq!(b_assign.identifier, "b");
                
                // Verify multiplication operation
                if let Expression::BinaryOp(mul_op) = &*b_assign.expression {
                    assert_eq!(mul_op.operator, OperatorToken::MUL);
                    verify_identifier(&mul_op.left, "a");
                    verify_number_literal(&mul_op.right, 7.0);
                } else {
                    panic!("Expected multiplication in b assignment");
                }

                // Verify print statement
                if let Expression::FunctionCall(print_call) = &*inner_let.body {
                    assert_eq!(print_call.function_name, "print");
                    assert_eq!(print_call.arguments.len(), 1);
                    verify_identifier(&print_call.arguments[0], "b");
                } else {
                    panic!("Expected print call in inner let-in body");
                }
            } else {
                panic!("Expected inner let-in expression");
            }
        } else {
            panic!("Expected outer let-in expression");
        }
    } else {
        panic!("Expected StatementExpression");
    }
}

#[test]
fn test_let_in_with_code_block() {
    let input = r#"
        let a = 5, b = 10, c = 20 in {
            print(a + b);
            print(b * c);
            print(c / a);
        }
    "#;

    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();
    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::LetIn(let_in) = &**expr {
            // Verify assignments
            assert_eq!(let_in.assignments.len(), 3);
            
            // Check a = 5
            let a_assign = &let_in.assignments[0];
            assert_eq!(a_assign.identifier, "a");
            verify_number_literal(&a_assign.expression, 5.0);

            // Check b = 10
            let b_assign = &let_in.assignments[1];
            assert_eq!(b_assign.identifier, "b");
            verify_number_literal(&b_assign.expression, 10.0);

            // Check c = 20
            let c_assign = &let_in.assignments[2];
            assert_eq!(c_assign.identifier, "c");
            verify_number_literal(&c_assign.expression, 20.0);

            // Verify code block body
            if let Expression::CodeBlock(block) = &*let_in.body {
                let expressions = &block.expression_list.expressions;
                assert_eq!(expressions.len(), 3);

                // First print: a + b
                if let Expression::FunctionCall(print1) = &expressions[0] {
                    assert_eq!(print1.function_name, "print");
                    verify_binary_operation(
                        &print1.arguments[0],
                        OperatorToken::PLUS,
                        |e| verify_identifier(e, "a"),
                        |e| verify_identifier(e, "b"),
                    );
                }

                // Second print: b * c
                if let Expression::FunctionCall(print2) = &expressions[1] {
                    assert_eq!(print2.function_name, "print");
                    verify_binary_operation(
                        &print2.arguments[0],
                        OperatorToken::MUL,
                        |e| verify_identifier(e, "b"),
                        |e| verify_identifier(e, "c"),
                    );
                }

                // Third print: c / a
                if let Expression::FunctionCall(print3) = &expressions[2] {
                    assert_eq!(print3.function_name, "print");
                    verify_binary_operation(
                        &print3.arguments[0],
                        OperatorToken::DIV,
                        |e| verify_identifier(e, "c"),
                        |e| verify_identifier(e, "a"),
                    );
                }
            } else {
                panic!("Expected code block in let-in body");
            }
        } else {
            panic!("Expected let-in expression");
        }
    } else {
        panic!("Expected StatementExpression");
    }
}

fn verify_binary_operation(
    expr: &Expression,
    expected_op: OperatorToken,
    check_left: impl Fn(&Expression),
    check_right: impl Fn(&Expression),
) {
    if let Expression::BinaryOp(bin_op) = expr {
        assert_eq!(bin_op.operator, expected_op);
        check_left(&bin_op.left);
        check_right(&bin_op.right);
    } else {
        panic!("Expected binary operation");
    }
}

#[test]
fn test_nested_let_in_assignment() {
    let input = "let a = (let b = 6 in b * 7) in print(a);";
    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();

    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::LetIn(outer_let) = &**expr {
            // Outer assignment: a = [...]
            assert_eq!(outer_let.assignments.len(), 1);
            let a_assign = &outer_let.assignments[0];
            assert_eq!(a_assign.identifier, "a");

            // Verify inner let-in: let b = 6 in b*7
            if let Expression::LetIn(inner_let) = &*a_assign.expression {
                assert_eq!(inner_let.assignments.len(), 1);
                
                // Inner assignment: b = 6
                let b_assign = &inner_let.assignments[0];
                assert_eq!(b_assign.identifier, "b");
                verify_number_literal(&b_assign.expression, 6.0);

                // Verify multiplication: b * 7
                if let Expression::BinaryOp(mul_op) = &*inner_let.body {
                    assert_eq!(mul_op.operator, OperatorToken::MUL);
                    verify_identifier(&mul_op.left, "b");
                    verify_number_literal(&mul_op.right, 7.0);
                }
            }

            // Verify print statement
            if let Expression::FunctionCall(print_call) = &*outer_let.body {
                assert_eq!(print_call.function_name, "print");
                verify_identifier(&print_call.arguments[0], "a");
            }
        }
    }
}

#[test]
fn test_let_in_as_print_argument() {
    let input = "print(let b = 6 in b * 7);";
    let program = hulk_compiler::parser::ProgramParser::new().parse(input).unwrap();

    assert_eq!(program.statements.len(), 1);

    if let Statement::StatementExpression(expr) = &program.statements[0] {
        if let Expression::FunctionCall(print_call) = &**expr {
            assert_eq!(print_call.function_name, "print");
            
            // Verify let-in in argument position
            if let Expression::LetIn(let_in) = &print_call.arguments[0] {
                assert_eq!(let_in.assignments.len(), 1);
                
                // Assignment: b = 6
                let b_assign = &let_in.assignments[0];
                assert_eq!(b_assign.identifier, "b");
                verify_number_literal(&b_assign.expression, 6.0);

                // Verify multiplication: b * 7
                if let Expression::BinaryOp(mul_op) = &*let_in.body {
                    assert_eq!(mul_op.operator, OperatorToken::MUL);
                    verify_identifier(&mul_op.left, "b");
                    verify_number_literal(&mul_op.right, 7.0);
                }
            }
        }
    }
}