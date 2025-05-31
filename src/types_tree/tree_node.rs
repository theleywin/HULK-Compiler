use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeNode {
    pub type_name: String,
    pub depth: i32,
    pub parent: Option<Box<TypeNode>>,
    pub children: Vec<Box<TypeNode>>,
    pub variables: HashMap<String, Box<TypeNode>>, // variables associated with this type
    pub methods: HashMap<String, Box<TypeNode>>, // methods associated with this type
}

impl TypeNode {
    pub fn new(type_name: String, depth: i32, parent: Option<Box<TypeNode>>) -> Self {
        TypeNode {
            type_name,
            depth,
            parent,
            children: Vec::new(),
            variables: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, mut child: Box<TypeNode>) {
        child.parent = Some(Box::new(self.clone()));
        self.children.push(child);
    }

    pub fn add_variable(&mut self, name: String, variable: Box<TypeNode>) {
        self.variables.insert(name, variable);
    }

    pub fn add_method(&mut self, name: String, method: Box<TypeNode>) {
        self.methods.insert(name, method);
    }

}