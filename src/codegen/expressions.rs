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
        context.add_line(format!("{} = sitofp i1 {} to double", new_temp, value));
        context.remove_bool_var(&value);
        new_temp
    } else if context.is_string(&value) {
        panic!("Cannot use string in arithmetic operation");
    } else {
        value
    }
}

fn convert_to_bool(context: &mut CodeGenContext, value: String) -> String {
    if context.is_string(&value) {
        panic!("Cannot use string in boolean operation");
    }

    if context.is_bool(&value) {
        return value;
    }

    let temp = context.new_temp();
    context.add_line(format!("{} = fcmp one double {}, 0.0", temp, value));
    context.add_bool_var(temp.clone());
    temp
}

pub fn gen_binary_op(
    context: &mut CodeGenContext,
    left_val: String,
    op: OperatorToken,
    right_val: String,
) -> String {
    match op {
        OperatorToken::PLUS | OperatorToken::MINUS | OperatorToken::MUL | OperatorToken::DIV => {
            let left_val = convert_to_double(context, left_val);
            let right_val = convert_to_double(context, right_val);

            let opcode = match op {
                OperatorToken::PLUS => "fadd",
                OperatorToken::MINUS => "fsub",
                OperatorToken::MUL => "fmul",
                OperatorToken::DIV => "fdiv",
                _ => unreachable!(),
            };

            let temp = context.new_temp();
            context.add_line(format!(
                "{} = {} double {}, {}",
                temp, opcode, left_val, right_val
            ));
            temp
        }

        OperatorToken::MOD => {
            let left_val = convert_to_double(context, left_val);
            let right_val = convert_to_double(context, right_val);

            // Declare fmod function if not already declared
            if !context.runtime_functions.contains("fmod") {
                context.add_global_declaration("declare double @fmod(double, double)".to_string());
                context.runtime_functions.insert("fmod".to_string());
            }

            let temp = context.new_temp();
            context.add_line(format!(
                "{} = call double @fmod(double {}, double {})",
                temp, left_val, right_val
            ));
            temp
        }

        OperatorToken::POW => {
            let left_val = convert_to_double(context, left_val);
            let right_val = convert_to_double(context, right_val);

            // Declare pow function if not already declared
            if !context.runtime_functions.contains("pow") {
                context.add_global_declaration("declare double @pow(double, double)".to_string());
                context.runtime_functions.insert("pow".to_string());
            }

            let temp = context.new_temp();
            context.add_line(format!(
                "{} = call double @pow(double {}, double {})",
                temp, left_val, right_val
            ));
            temp
        }

        OperatorToken::EQ
        | OperatorToken::NEQ
        | OperatorToken::GT
        | OperatorToken::GTE
        | OperatorToken::LT
        | OperatorToken::LTE => {
            // For comparisons, we need to handle different types

            if context.is_bool(&left_val) && context.is_bool(&right_val) {
                // Boolean comparison
                let cmp_op = match op {
                    OperatorToken::EQ => "eq",
                    OperatorToken::NEQ => "ne",
                    _ => panic!("Invalid comparison operator for booleans: {:?}", op),
                };

                let temp = context.new_temp();
                context.add_line(format!(
                    "{} = icmp {} i1 {}, {}",
                    temp, cmp_op, left_val, right_val
                ));
                context.add_bool_var(temp.clone());
                temp
            } else {
                // Numeric comparison
                let left_val = convert_to_double(context, left_val);
                let right_val = convert_to_double(context, right_val);

                let cmp_op = match op {
                    OperatorToken::EQ => "oeq",
                    OperatorToken::NEQ => "one",
                    OperatorToken::GT => "ogt",
                    OperatorToken::GTE => "oge",
                    OperatorToken::LT => "olt",
                    OperatorToken::LTE => "ole",
                    _ => unreachable!(),
                };

                let temp = context.new_temp();
                context.add_line(format!(
                    "{} = fcmp {} double {}, {}",
                    temp, cmp_op, left_val, right_val
                ));
                context.add_bool_var(temp.clone());
                temp
            }
        }

        OperatorToken::AND | OperatorToken::OR => {
            // Logical operators require boolean operands
            let left_val = convert_to_bool(context, left_val);
            let right_val = convert_to_bool(context, right_val);

            let opcode = match op {
                OperatorToken::AND => "and",
                OperatorToken::OR => "or",
                _ => unreachable!(),
            };

            let temp = context.new_temp();
            context.add_line(format!(
                "{} = {} i1 {}, {}",
                temp, opcode, left_val, right_val
            ));
            context.add_bool_var(temp.clone());
            temp
        }

        OperatorToken::CONCAT => {
            // String concatenation
            if !context.is_string(&left_val) || !context.is_string(&right_val) {
                panic!("Concatenation requires string operands");
            }

            // Declare concat function if not already declared
            if !context.runtime_functions.contains("concat") {
                context.add_global_declaration("declare i8* @concat(i8*, i8*)".to_string());
                context.runtime_functions.insert("concat".to_string());
            }

            let temp = context.new_temp();
            context.add_line(format!(
                "{} = call i8* @concat(i8* {}, i8* {})",
                temp, left_val, right_val
            ));
            context.add_string_var(temp.clone());
            temp
        }

        _ => panic!("Unsupported binary operator: {:?}", op),
    }
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
