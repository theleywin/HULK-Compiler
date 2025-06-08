use super::context::CodeGenContext;
use crate::ast_nodes::expression::Expression;
use crate::tokens::OperatorToken;
use crate::visitor::accept::Accept;
use crate::visitor::visitor_trait::Visitor;

pub fn gen_number(_context: &mut CodeGenContext, value: &str) -> String {
    // Handle integers and floats properly
    if let Ok(int_value) = value.parse::<i64>() {
        format!("{}", int_value)
    } else if let Ok(float_value) = value.parse::<f64>() {
        format!("{:?}", float_value) // Use debug formatting for floats
    } else {
        panic!("Invalid number literal: {}", value)
    }
}

pub fn gen_binary_op<V: Visitor<String>>(
    context: &mut CodeGenContext,
    left: &Expression,
    op: OperatorToken,
    right: &Expression,
    visitor: &mut V,
) -> String {
    let left_val = left.accept(visitor);
    let right_val = right.accept(visitor);

    let opcode: &'static str = match op {
        OperatorToken::PLUS => "fadd",
        OperatorToken::MINUS => "fsub",
        OperatorToken::MUL => "fmul",
        OperatorToken::DIV => "fdiv",
        _ => panic!("Unsupported operator: {:?}", op),
    };

    let temp = context.new_temp();
    context.add_line(format!(
        "{} = {} double {}, {}",
        temp, opcode, left_val, right_val
    ));
    temp
}

// Similarly update gen_unary_op to be generic
pub fn gen_unary_op<V: Visitor<String>>(
    context: &mut CodeGenContext,
    op: OperatorToken,
    operand: &Expression,
    visitor: &mut V,
) -> String {
    if op == OperatorToken::MINUS {
        let operand_val = operand.accept(visitor);
        let temp = context.new_temp();
        context.add_line(format!("{} = fsub double 0.0, {}", temp, operand_val));
        temp
    } else {
        panic!("Unsupported unary operator: {:?}", op);
    }
}
