#[derive(Default)]
pub struct CodeGenContext {
    pub code: Vec<String>,
    pub temp_counter: u32,
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
        std::mem::take(&mut self.code)
    }
}
