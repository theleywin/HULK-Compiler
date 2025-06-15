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
    pub global_constants: HashSet<String>,
    pub str_constants: Vec<String>,
    pub str_counter: usize,
    pub temp_counter: usize,
    pub id: usize,
    scope_id: i32,
    pub temp_types: HashMap<String, String>,
    pub string_literals: HashMap<String, String>,
    pub next_string_id: usize,
    pub runtime_functions: HashSet<String>,
    pub variables: HashMap<String, String>,
    pub scopes: Vec<HashMap<String, String>>,
}

#[derive(Clone)]
pub struct VariableInfo {
    pub temp: String,
    pub ty: String,
}

impl Default for CodeGenContext {
    fn default() -> Self {
        Self {
            code: Vec::new(),
            globals: Vec::new(),
            global_constants: HashSet::new(),
            str_constants: Vec::new(),
            str_counter: 0,
            temp_counter: 1,
            id: 1,
            scope_id: 0,
            temp_types: HashMap::new(),
            string_literals: HashMap::new(),
            next_string_id: 0,
            runtime_functions: HashSet::new(),
            variables: HashMap::new(),
            scopes: Vec::new(),
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

    pub fn new_temp(&mut self, ty: String) -> String {
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
    pub fn add_global_constant(&mut self, name: &str) {
        self.global_constants.insert(name.to_string());
    }
    
    pub fn is_global_constant(&self, name: &str) -> bool {
        self.global_constants.contains(name)
    }

    pub fn take_globals(&mut self) -> Vec<String> {
        std::mem::take(&mut self.globals)
    }

    pub fn take_body(&mut self) -> Vec<String> {
        std::mem::take(&mut self.code)
    }

    pub fn get_type(&self, temp: &str) -> String {
        self.temp_types.get(temp).expect("Unknown temporary").clone()
    }

    pub fn is_bool(&self, name: &str) -> bool {
        self.get_type(name) == "Boolean"
    }

    pub fn is_string(&self, name: &str) -> bool {
        self.get_type(name) == "String"
    }

    pub fn enter_scope(&mut self) {
        self.scope_id += 1;
        self.scopes.push(self.variables.clone())
    }

    pub fn exit_scope(&mut self) {
        self.scope_id -= 1;
        self.variables = self.scopes.pop().unwrap_or_default();
    }

    pub fn get_scope(&self) -> i32 {
        self.scope_id
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

    pub fn add_str_const(&mut self, value: String, len: usize) -> String {
        let constant_name = format!("@.str.{}", self.str_counter);
        self.str_counter += 1;
        
        let escaped_value = value
            .replace('\\', "\\\\")
            .replace('\"', "\\\"")
            .replace('\n', "\\0A")
            .replace('\0', "\\00");
        
        let line = format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"", 
            constant_name, len + 1, escaped_value
        );
        
        if !self.str_constants.contains(&line) { 
            self.str_constants.push(line);
        }
        constant_name
    }
    
    pub fn add_global_declaration(&mut self, decl: String) {
        self.globals.push(decl);
    }
    
    pub fn add_variable(&mut self, name: String, ty: String) {
        self.variables.insert(
            name,
            ty,
        );
    }
    
    pub fn get_variable(&self, name: String) -> String {
        let mut current_scope = self.scope_id.clone();
        while current_scope >= 0 {
            let register = format!("%{}.{}",name,current_scope);
            if let Some(_) = self.variables.get(&register) {
                return register;
            }
            current_scope -= 1;
        }
        panic!("Variable not found: {}",name.to_string())
    }

    pub fn new_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        id
    }
}