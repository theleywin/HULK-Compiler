use std::collections::HashMap;
use super::tree_node::TypeNode;

pub enum BuiltInTypes {
    Object,
    String,
    Number,
    Boolean,
    Unknown,
}

impl BuiltInTypes {
    pub fn as_str(&self) -> &str {
        match self {
            BuiltInTypes::Object => "Object",
            BuiltInTypes::String => "String",
            BuiltInTypes::Number => "Number",
            BuiltInTypes::Boolean => "Boolean",
            BuiltInTypes::Unknown => "Unknown",
        }
    }
}

pub struct TypeTree {
    root: TypeNode,
    nodes: HashMap<String, TypeNode>,
}

impl TypeTree {
    pub fn new() -> Self {
        let mut newtree = TypeTree {
            root: TypeNode::new("Object".to_string(), 0, None),
            nodes: HashMap::new(),
        };
        newtree.nodes.insert("Object".to_string(), newtree.root.clone());
        newtree.add_type("String".to_string(), Some("Object".to_string()));
        newtree.add_type("Number".to_string(), Some("Object".to_string()));
        newtree.add_type("Boolean".to_string(), Some("Object".to_string()));
        newtree.add_type("Unknown".to_string(), Some("Object".to_string()));
        newtree
    }

    pub fn add_type(&mut self, type_name: String, parent_name: Option<String>) {
        match parent_name {
            Some(name) => {
                if let Some(parent) = self.nodes.get(&name) {
                    let new_node = TypeNode::new(type_name.clone(), parent.depth + 1, Some(Box::new(parent.clone())));
                    self.nodes.insert(type_name.clone(), new_node);
                }
            }
            None => {
                let new_node = TypeNode::new(type_name.clone(), 0, Some(Box::new(self.root.clone())));
                self.nodes.insert(type_name.clone(), new_node);
            }
        }
    }

    pub fn get_type(&self, type_name: &str) -> Option<TypeNode> {
        self.nodes.get(type_name).cloned()
    }

    pub fn find_lca(&self, type1: &TypeNode, type2: &TypeNode) -> TypeNode {
        if type1.type_name == type2.type_name {
            return type1.clone();
        }
        if type1.depth < type2.depth {
            return self.find_lca(type1, type2);
        } else if type2.depth < type1.depth {
            if let Some(ref parent1) = type1.parent {
                return self.find_lca(parent1, type2);
            } else {
                return self.root.clone();
            }
        } else {
            if let (Some(parent1), Some(parent2)) = (&type1.parent, &type2.parent) {
                return self.find_lca(parent1, parent2);
            } else {
                return self.root.clone();
            }
        }
    }

    pub fn is_ancestor(&self, ancestor: &TypeNode, descendant: &TypeNode) -> bool {
        let mut current = Some(descendant);
        while let Some(node) = current {
            if node.type_name == ancestor.type_name {
                return true;
            }
            current = node.parent.as_deref();
        }
        false
    }


}
