use std::collections::{HashMap, HashSet};

pub struct CodeGenContext {
    pub code: Vec<String>,
    pub globals: Vec<String>,
    pub temp_counter: usize,
    pub bool_vars: HashSet<String>,
    pub string_vars: HashSet<String>,
    pub string_literals: HashMap<String, String>,
    pub next_string_id: usize,
    pub runtime_functions: HashSet<String>,
}

impl Default for CodeGenContext {
    fn default() -> Self {
        Self {
            code: Vec::new(),
            globals: Vec::new(),
            temp_counter: 0,
            bool_vars: HashSet::new(),
            string_vars: HashSet::new(),
            string_literals: HashMap::new(),
            next_string_id: 0,
            runtime_functions: HashSet::new(),
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

    pub fn new_temp(&mut self) -> String {
        let id = self.temp_counter;
        self.temp_counter += 1;
        format!("%{}", id)
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

    pub fn is_bool(&self, name: &str) -> bool {
        self.bool_vars.contains(name)
    }

    pub fn add_bool_var(&mut self, name: String) {
        self.bool_vars.insert(name);
    }

    pub fn remove_bool_var(&mut self, name: &str) {
        self.bool_vars.remove(name);
    }

    pub fn is_string(&self, name: &str) -> bool {
        self.string_vars.contains(name)
    }

    pub fn add_string_var(&mut self, name: String) {
        self.string_vars.insert(name);
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
}
