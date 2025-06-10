use std::collections::{HashSet, HashMap};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Type {
    Double,
    Boolean,
    String,
}

pub struct CodeGenContext {
    pub code: Vec<String>,
    pub globals: Vec<String>,
    pub temp_counter: usize,
    pub temp_types: HashMap<String, Type>,
    pub string_literals: HashMap<String, String>,
    pub next_string_id: usize,
    pub runtime_functions: HashSet<String>,
    pub variables: HashMap<String, VariableInfo>,
}

#[derive(Clone)]
pub struct VariableInfo {
    pub temp: String,
    pub ty: Type,
}

impl Default for CodeGenContext {
    fn default() -> Self {
        Self {
            code: Vec::new(),
            globals: Vec::new(),
            temp_counter: 0,
            temp_types: HashMap::new(),
            string_literals: HashMap::new(),
            next_string_id: 0,
            runtime_functions: HashSet::new(),
            variables: HashMap::new(),
        }
    }
}

impl CodeGenContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_line(&mut self, line: String) {
        self.code.push(line);
    }

    pub fn new_temp(&mut self, ty: Type) -> String {
        let id = self.temp_counter;
        self.temp_counter += 1;
        let name = format!("%{}", id);
        self.temp_types.insert(name.clone(), ty);
        name
    }

    pub fn take_code(&mut self) -> Vec<String> {
        let mut result = Vec::new();
        result.extend(std::mem::take(&mut self.globals));
        result.extend(std::mem::take(&mut self.code));
        result
    }

    pub fn take_globals(&mut self) -> Vec<String> {
        std::mem::take(&mut self.globals)
    }

    pub fn take_body(&mut self) -> Vec<String> {
        std::mem::take(&mut self.code)
    }

    pub fn get_type(&self, temp: &str) -> Type {
        *self.temp_types.get(temp).expect("Unknown temporary")
    }

    pub fn is_bool(&self, name: &str) -> bool {
        self.get_type(name) == Type::Boolean
    }

    pub fn is_string(&self, name: &str) -> bool {
        self.get_type(name) == Type::String
    }

    pub fn add_string_literal(&mut self, value: &str) -> String {
        if let Some(name) = self.string_literals.get(value) {
            return name.clone();
        }

        let escaped = value
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", "\\n")
            .replace("\t", "\\t");

        let name = format!("@.str.{}", self.next_string_id);
        self.next_string_id += 1;

        let global = format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"",
            name,
            escaped.len() + 1,
            escaped
        );
        self.globals.push(global);
        self.string_literals.insert(value.to_string(), name.clone());
        name
    }
    
    pub fn add_global_declaration(&mut self, decl: String) {
        self.globals.push(decl);
    }
    
    pub fn add_variable(&mut self, name: &str, temp: String, ty: Type) {
        self.variables.insert(
            name.to_string(),
            VariableInfo { temp: temp.clone(), ty }
        );
        self.temp_types.insert(temp, ty);
    }
    
    pub fn get_variable(&self, name: &str) -> Option<&VariableInfo> {
        self.variables.get(name)
    }
}