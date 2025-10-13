use crate::llm::client::LLMClient;
use crate::templates::{FunctionTemplate, TemplateLibrary};
use crate::templates::prompt_generator::PromptGenerator;
use anyhow::{anyhow, Result};
use regex::Regex;
use std::path::Path;
use tokio::fs;

pub struct FunctionGenerator {
    llm_client: LLMClient,
    template_library: TemplateLibrary,
    prompt_generator: PromptGenerator,
}

#[derive(Debug, Clone)]
pub struct GeneratedFunction {
    pub name: String,
    pub code: String,
    pub category: String,
    pub complexity: u32,
    pub template_used: Option<String>,
}

impl FunctionGenerator {
    pub fn new() -> Result<Self> {
        Ok(Self {
            llm_client: LLMClient::new()?,
            template_library: TemplateLibrary::new(),
            prompt_generator: PromptGenerator::new(),
        })
    }

    pub async fn generate_function(&self, template: &FunctionTemplate) -> Result<GeneratedFunction> {
        let prompt = self.prompt_generator.generate_basic_prompt(template);
        
        println!("Generating function from template: {}", template.name);
        println!("Prompt: {}", prompt);
        
        let response = self.llm_client.generate_with_retry(&prompt, 3).await?;
        let function_code = self.extract_function_code(&response)?;
        
        Ok(GeneratedFunction {
            name: template.name.clone(),
            code: function_code,
            category: template.category.clone(),
            complexity: template.complexity,
            template_used: Some(template.name.clone()),
        })
    }

    pub async fn generate_constrained_function(
        &self,
        template: &FunctionTemplate,
        constraints: &[String],
    ) -> Result<GeneratedFunction> {
        let prompt = self.prompt_generator.generate_constrained_prompt(template, constraints);
        
        println!("Generating constrained function: {}", template.name);
        
        let response = self.llm_client.generate_with_retry(&prompt, 3).await?;
        let function_code = self.extract_function_code(&response)?;
        
        Ok(GeneratedFunction {
            name: format!("{}_constrained", template.name),
            code: function_code,
            category: template.category.clone(),
            complexity: template.complexity + 1,
            template_used: Some(template.name.clone()),
        })
    }

    pub async fn generate_batch(
        &self,
        category: &str,
        count: usize,
    ) -> Result<Vec<GeneratedFunction>> {
        let templates = self.template_library.get_templates(category);
        
        if templates.is_empty() {
            return Err(anyhow!("No templates found for category: {}", category));
        }
        
        let mut functions = Vec::new();
        
        for i in 0..count {
            let template = &templates[i % templates.len()];
            match self.generate_function(template).await {
                Ok(func) => functions.push(func),
                Err(e) => eprintln!("Failed to generate function {}: {}", i, e),
            }
        }
        
        Ok(functions)
    }

    pub async fn save_functions_to_directory(
        &self,
        functions: &[GeneratedFunction],
        output_dir: &Path,
    ) -> Result<()> {
        fs::create_dir_all(output_dir).await?;
        
        for (i, func) in functions.iter().enumerate() {
            let filename = format!("{:06}_{}.rs", i + 1, func.name);
            let filepath = output_dir.join(filename);
            
            let content = format!(
                "// Generated function: {}\n// Category: {}\n// Complexity: {}\n// Template: {:?}\n\n{}",
                func.name, func.category, func.complexity, func.template_used, func.code
            );
            
            fs::write(filepath, content).await?;
        }
        
        println!("Saved {} functions to {}", functions.len(), output_dir.display());
        Ok(())
    }

    fn extract_function_code(&self, response: &str) -> Result<String> {
        // Try to extract code between ```rust and ``` markers
        let rust_block_regex = Regex::new(r"```rust\s*(.*?)\s*```")?;
        if let Some(captures) = rust_block_regex.captures(response) {
            return Ok(captures.get(1).unwrap().as_str().to_string());
        }
        
        // Try to extract code between ``` markers (without language specification)
        let generic_block_regex = Regex::new(r"```\s*(.*?)\s*```")?;
        if let Some(captures) = generic_block_regex.captures(response) {
            let code = captures.get(1).unwrap().as_str();
            // Basic check if this looks like Rust code
            if code.contains("fn ") || code.contains("struct ") || code.contains("impl ") {
                return Ok(code.to_string());
            }
        }
        
        // Look for function definitions in the response
        let fn_regex = Regex::new(r"fn\s+\w+.*?\{.*?\}")?;
        if let Some(mat) = fn_regex.find(response) {
            return Ok(mat.as_str().to_string());
        }
        
        // If no code blocks found, return the whole response and let caller handle it
        Ok(response.trim().to_string())
    }
}