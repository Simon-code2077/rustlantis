pub mod prompt_generator;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionTemplate {
    pub category: String,
    pub name: String,
    pub description: String,
    pub constraints: Vec<String>,
    pub example_signature: String,
    pub return_type: String,
    pub parameters: Vec<String>,
    pub complexity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLibrary {
    pub templates: HashMap<String, Vec<FunctionTemplate>>,
}

impl TemplateLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            templates: HashMap::new(),
        };
        library.initialize_templates();
        library
    }

    fn add_template(&mut self, category: &str, template: FunctionTemplate) {
        self.templates
            .entry(category.to_string())
            .or_insert_with(Vec::new)
            .push(template);
    }

    fn initialize_templates(&mut self) {
        // Arithmetic Templates
        self.add_template("arithmetic", FunctionTemplate {
            category: "arithmetic".to_string(),
            name: "add_numbers".to_string(),
            description: "Add two numbers with overflow checking".to_string(),
            constraints: vec!["must handle overflow".to_string(), "return Result type".to_string()],
            example_signature: "fn add_numbers(a: i32, b: i32) -> Result<i32, String>".to_string(),
            return_type: "Result<i32, String>".to_string(),
            parameters: vec!["i32".to_string(), "i32".to_string()],
            complexity: 2,
        });

        // Memory Templates
        self.add_template("memory", FunctionTemplate {
            category: "memory".to_string(),
            name: "process_slice".to_string(),
            description: "Process a slice of data with bounds checking".to_string(),
            constraints: vec!["must validate bounds".to_string(), "handle empty slices".to_string()],
            example_signature: "fn process_slice(data: &mut [u8], start: usize, end: usize) -> bool".to_string(),
            return_type: "bool".to_string(),
            parameters: vec!["&mut [u8]".to_string(), "usize".to_string(), "usize".to_string()],
            complexity: 4,
        });

        // Control Flow Templates
        self.add_template("control_flow", FunctionTemplate {
            category: "control_flow".to_string(),
            name: "conditional_process".to_string(),
            description: "Process input based on complex conditions".to_string(),
            constraints: vec!["multiple condition branches".to_string(), "early returns".to_string()],
            example_signature: "fn conditional_process(input: Option<i32>) -> Result<String, &'static str>".to_string(),
            return_type: "Result<String, &'static str>".to_string(),
            parameters: vec!["Option<i32>".to_string()],
            complexity: 5,
        });

        // Type Conversion Templates
        self.add_template("type_conversion", FunctionTemplate {
            category: "type_conversion".to_string(),
            name: "safe_convert".to_string(),
            description: "Safely convert between numeric types".to_string(),
            constraints: vec!["must handle conversion errors".to_string(), "no data loss".to_string()],
            example_signature: "fn safe_convert(input: f64) -> Result<i32, String>".to_string(),
            return_type: "Result<i32, String>".to_string(),
            parameters: vec!["f64".to_string()],
            complexity: 3,
        });

        // String Operations Templates
        self.add_template("string_ops", FunctionTemplate {
            category: "string_ops".to_string(),
            name: "format_and_validate".to_string(),
            description: "Format string with validation and error handling".to_string(),
            constraints: vec!["validate input strings".to_string(), "handle unicode".to_string()],
            example_signature: "fn format_and_validate(template: &str, args: &[String]) -> Result<String, &'static str>".to_string(),
            return_type: "Result<String, &'static str>".to_string(),
            parameters: vec!["&str".to_string(), "&[String]".to_string()],
            complexity: 3,
        });
    }

    pub fn get_templates(&self, category: &str) -> Vec<FunctionTemplate> {
        self.templates.get(category).cloned().unwrap_or_default()
    }

    pub fn get_all_templates(&self) -> &HashMap<String, Vec<FunctionTemplate>> {
        &self.templates
    }

    pub fn get_template_count(&self, category: &str) -> usize {
        self.templates.get(category).map_or(0, |v| v.len())
    }

    pub fn calculate_complexity_score(&self, template: &FunctionTemplate) -> u32 {
        template.complexity
    }
}