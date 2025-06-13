use super::context::{CodeGenContext, Type};
use crate::tokens::OperatorToken;

fn convert_to_double(context: &mut CodeGenContext, value: String) -> String {
    match context.get_type(&value) {
        Type::Boolean => {
            let new_temp = context.new_temp(Type::Double);
            context.add_line(format!("{} = sitofp i1 {} to double", new_temp, value));
            new_temp
        }
        Type::String => {
            panic!("Cannot use string in arithmetic operation");
        }
        Type::Double => value,
    }
}

fn convert_to_bool(context: &mut CodeGenContext, value: String) -> String {
    match context.get_type(&value) {
        Type::String => {
            panic!("Cannot use string in boolean operation");
        }
        Type::Boolean => value,
        Type::Double => {
            let temp = context.new_temp(Type::Boolean);
            context.add_line(format!("{} = fcmp one double {}, 0.0", temp, value));
            temp
        }
    }
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

            let temp = context.new_temp(Type::Double);
            context.add_line(format!(
                "{} = {} double {}, {}",
                temp, opcode, left_val, right_val
            ));
            temp
        }

        OperatorToken::MOD => {
            let left_val = convert_to_double(context, left_val);
            let right_val = convert_to_double(context, right_val);

            if !context.runtime_functions.contains("fmod") {
                context.add_global_declaration("declare double @fmod(double, double)".to_string());
                context.runtime_functions.insert("fmod".to_string());
            }

            let temp = context.new_temp(Type::Double);
            context.add_line(format!(
                "{} = call double @fmod(double {}, double {})",
                temp, left_val, right_val
            ));
            temp
        }

        OperatorToken::POW => {
            let left_val = convert_to_double(context, left_val);
            let right_val = convert_to_double(context, right_val);

            if !context.runtime_functions.contains("pow") {
                context.add_global_declaration("declare double @pow(double, double)".to_string());
                context.runtime_functions.insert("pow".to_string());
            }

            let temp = context.new_temp(Type::Double);
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
            if context.get_type(&left_val) == Type::Boolean
                && context.get_type(&right_val) == Type::Boolean
            {
                let cmp_op = match op {
                    OperatorToken::EQ => "eq",
                    OperatorToken::NEQ => "ne",
                    _ => panic!("Invalid comparison operator for booleans: {:?}", op),
                };

                let temp = context.new_temp(Type::Boolean);
                context.add_line(format!(
                    "{} = icmp {} i1 {}, {}",
                    temp, cmp_op, left_val, right_val
                ));
                temp
            } else {
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

                let temp = context.new_temp(Type::Boolean);
                context.add_line(format!(
                    "{} = fcmp {} double {}, {}",
                    temp, cmp_op, left_val, right_val
                ));
                temp
            }
        }

        OperatorToken::AND | OperatorToken::OR => {
            let left_val = convert_to_bool(context, left_val);
            let right_val = convert_to_bool(context, right_val);

            let opcode = match op {
                OperatorToken::AND => "and",
                OperatorToken::OR => "or",
                _ => unreachable!(),
            };

            let temp = context.new_temp(Type::Boolean);
            context.add_line(format!(
                "{} = {} i1 {}, {}",
                temp, opcode, left_val, right_val
            ));
            temp
        }

        OperatorToken::CONCAT => {
            if context.get_type(&left_val) != Type::String
                || context.get_type(&right_val) != Type::String
            {
                panic!("Concatenation requires string operands");
            }

            if !context.runtime_functions.contains("concat") {
                context.add_global_declaration("declare i8* @concat(i8*, i8*)".to_string());
                context.runtime_functions.insert("concat".to_string());
            }

            let temp = context.new_temp(Type::String);
            context.add_line(format!(
                "{} = call i8* @concat(i8* {}, i8* {})",
                temp, left_val, right_val
            ));
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
            let temp = context.new_temp(Type::Double);
            context.add_line(format!("{} = fsub double 0.0, {}", temp, operand_val));
            temp
        }
        OperatorToken::NOT => {
            let operand_val = convert_to_bool(context, operand_val);
            let temp = context.new_temp(Type::Boolean);
            context.add_line(format!("{} = xor i1 {}, true", temp, operand_val));
            temp
        }
        _ => panic!("Unsupported unary operator: {:?}", op),
    }
}
