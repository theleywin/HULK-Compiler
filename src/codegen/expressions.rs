use super::context::CodeGenContext;
use crate::tokens::OperatorToken;

pub fn gen_number(_context: &mut CodeGenContext, value: &str) -> String {
    if let Ok(int_value) = value.parse::<i64>() {
        format!("{}.0", int_value)
    } else if let Ok(float_value) = value.parse::<f64>() {
        format!("{}", float_value)
    } else {
        panic!("Invalid number literal: {}", value)
    }
}


pub fn gen_binary_op(
    context: &mut CodeGenContext,
    left_val: String,
    op: OperatorToken,
    right_val: String,
) -> String {
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
pub fn gen_unary_op(
    context: &mut CodeGenContext,
    op: OperatorToken,
    operand_val: String,
) -> String {
    if op == OperatorToken::MINUS {
        let temp = context.new_temp();
        context.add_line(format!("{} = fsub double 0.0, {}", temp, operand_val));
        temp
    } else {
        panic!("Unsupported unary operator: {:?}", op);
    }
}
