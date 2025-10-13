use crate::templates::FunctionTemplate;

pub struct PromptGenerator;

impl PromptGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_basic_prompt(&self, template: &FunctionTemplate) -> String {
        format!(
            "Generate a Rust function following these specifications:

Function Category: {}
Function Name: {}
Description: {}
Example Signature: {}
Return Type: {}
Parameters: {:?}
Constraints: {:?}

Requirements:
1. Write only the function implementation, no additional explanations
2. Follow Rust best practices and idioms
3. Include proper error handling where appropriate
4. Ensure the function compiles and is safe
5. Add inline comments for complex logic
6. Use appropriate Rust types and patterns

Please generate the complete function implementation:",
            template.category,
            template.name,
            template.description,
            template.example_signature,
            template.return_type,
            template.parameters,
            template.constraints
        )
    }

    pub fn generate_constrained_prompt(&self, template: &FunctionTemplate, constraints: &[String]) -> String {
        let mut prompt = self.generate_basic_prompt(template);
        
        prompt.push_str("\n\nAdditional Constraints:\n");
        for (i, constraint) in constraints.iter().enumerate() {
            prompt.push_str(&format!("{}. {}\n", i + 1, constraint));
        }
        
        prompt
    }
}