use crate::{ast_nodes::{expression::Expression, function_def::{FunctionDefNode, FunctionParams}, let_in::Assignment}, types_tree::tree_node::TypeNode};

pub struct TypeInherits {
    pub identifier: String,
    pub params: Vec<Expression>,
}

impl TypeInherits {
    pub fn new(identifier: String, params: Vec<Expression>) -> Self {
        TypeInherits { identifier, params }
    }
}

#[derive(Debug, PartialEq,Clone)]
pub enum TypeMember {
    Property(Assignment),
    Method(FunctionDefNode),
}

impl TypeMember {
    pub fn new_property(assignment: Assignment) -> Self {
        TypeMember::Property(assignment)
    }

    pub fn new_method(method: FunctionDefNode) -> Self {
        TypeMember::Method(method)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDefNode {
    pub identifier: String,
    pub params: Vec<FunctionParams>,
    pub parent: Option<String>,
    pub parent_args: Vec<Expression>,
    pub members: Vec<TypeMember>,
    pub node_type: Option<TypeNode>,
}

impl TypeDefNode {
    pub fn new(identifier: String, params: Vec<FunctionParams>, parent: Option<String>, parent_args: Vec<Expression>, members: Vec<TypeMember>) -> Self {
        TypeDefNode {
            identifier,
            params,
            parent,
            parent_args,
            members,
            node_type: None,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}