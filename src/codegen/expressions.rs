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

pub fn gen_boolean(_context: &mut CodeGenContext, value: bool) -> String {
    if value {
        "1".to_string()
    } else {
        "0".to_string()
    }
}

pub fn gen_string(_context: &mut CodeGenContext, value: &str) -> String {
    let escaped = value
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\t", "\\t");
    format!("c\"{}\"", escaped)
}

// Helper to convert a value to double if it's a boolean (strings are not allowed in arithmetic)
fn convert_to_double(context: &mut CodeGenContext, value: String) -> String {
    if context.is_bool(&value) {
        let new_temp = context.new_temp();
        context.add_line(format!(
            "{} = sitofp i1 {} to double",
            new_temp, value
        ));
        context.remove_bool_var(&value);
        new_temp
    } else if context.is_string(&value) {
        panic!("Cannot use string in arithmetic operation");
    } else {
        value
    }
}

pub fn gen_binary_op(
    context: &mut CodeGenContext,
    left_val: String,
    op: OperatorToken,
    right_val: String,
) -> String {
    let left_val = convert_to_double(context, left_val);
    let right_val = convert_to_double(context, right_val);

    let opcode = match op {
        OperatorToken::PLUS => "fadd",
        OperatorToken::MINUS => "fsub",
        OperatorToken::MUL => "fmul",
        OperatorToken::DIV => "fdiv",
        _ => panic!("Unsupported binary operator: {:?}", op),
    };

    let temp = context.new_temp();
    context.add_line(format!(
        "{} = {} double {}, {}",
        temp, opcode, left_val, right_val
    ));
    temp
}

pub fn gen_unary_op(
    context: &mut CodeGenContext,
    op: OperatorToken,
    operand_val: String,
) -> String {
    match op {
        OperatorToken::MINUS => {
            let operand_val = convert_to_double(context, operand_val);
            let temp = context.new_temp();
            context.add_line(format!("{} = fsub double 0.0, {}", temp, operand_val));
            temp
        }
        OperatorToken::NOT => {
            // Ensure the operand is boolean
            if !context.is_bool(&operand_val) {
                panic!("Cannot apply logical not to non-boolean");
            }
            let temp = context.new_temp();
            context.add_line(format!("{} = xor i1 {}, true", temp, operand_val));
            context.add_bool_var(temp.clone());
            temp
        }
        _ => panic!("Unsupported unary operator: {:?}", op),
    }
}