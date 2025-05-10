use super::Expression;

pub enum FunctionBody {
    Expr(Box<Expression>),
    Block(Vec<Expression>), 
}

pub struct FunctionDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: FunctionBody,
}

impl FunctionDef {
    pub fn new_expr(name: String, params: Vec<String>, expr: Expression) -> Self {
        FunctionDef {
            name,
            params,
            body: FunctionBody::Expr(Box::new(expr)),
        }
    }

    pub fn new_block(name: String, params: Vec<String>, block: Vec<Expression>) -> Self {
        FunctionDef {
            name,
            params,
            body: FunctionBody::Block(block),
        }
    }
}


// pub struct FunctionCall {
//     pub name: String,
//     pub args: Vec<Expression>,
// }
// 
// impl FunctionCall {
//     pub fn new(name: String, args: Vec<Expression>) -> Self {
//         FunctionCall {
//             name,
//             args,
//         }
//     }
// }