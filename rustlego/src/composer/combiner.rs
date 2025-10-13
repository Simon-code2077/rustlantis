use crate::llm::function_generator::GeneratedFunction;
use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ComposedProgram {
    pub name: String,
    pub code: String,
    pub functions: Vec<GeneratedFunction>,
    pub complexity: u32,
    pub main_function: String,
}

pub struct FunctionCombiner {
    type_compatibility: HashMap<String, Vec<String>>,
}

impl FunctionCombiner {
    pub fn new() -> Self {
        let mut type_compatibility = HashMap::new();
        
        // Define basic type compatibility rules
        type_compatibility.insert("i32".to_string(), vec!["i64".to_string(), "f32".to_string(), "f64".to_string()]);
        type_compatibility.insert("i64".to_string(), vec!["i32".to_string(), "f64".to_string()]);
        type_compatibility.insert("f32".to_string(), vec!["f64".to_string(), "i32".to_string()]);
        type_compatibility.insert("f64".to_string(), vec!["f32".to_string(), "i64".to_string()]);
        type_compatibility.insert("String".to_string(), vec!["&str".to_string()]);
        type_compatibility.insert("&str".to_string(), vec!["String".to_string()]);
        type_compatibility.insert("Vec<T>".to_string(), vec!["&[T]".to_string()]);
        type_compatibility.insert("&[T]".to_string(), vec!["Vec<T>".to_string()]);
        
        Self { type_compatibility }
    }

    pub fn combine_functions(&self, functions: &[GeneratedFunction]) -> Result<ComposedProgram> {
        if functions.is_empty() {
            return Err(anyhow!("Cannot combine empty function list"));
        }

        let mut combined_code = String::new();
        let mut used_names = HashSet::new();
        let mut renamed_functions = Vec::new();

        // First pass: resolve naming conflicts and collect functions
        for (i, func) in functions.iter().enumerate() {
            let mut new_func = func.clone();
            
            // Extract function name from code
            let original_name = self.extract_function_name(&func.code)?;
            
            // Generate unique name if conflict exists
            let unique_name = if used_names.contains(&original_name) {
                format!("{}_{}", original_name, i)
            } else {
                original_name.clone()
            };
            
            used_names.insert(unique_name.clone());
            
            // Rename function in code if necessary
            if unique_name != original_name {
                new_func.code = self.rename_function(&func.code, &original_name, &unique_name)?;
                new_func.name = unique_name.clone();
            }
            
            renamed_functions.push(new_func);
        }

        // Second pass: combine function codes
        for func in &renamed_functions {
            combined_code.push_str(&func.code);
            combined_code.push_str("\n\n");
        }

        // Third pass: create main function that calls the combined functions
        let main_function = self.create_main_function(&renamed_functions)?;
        combined_code.push_str(&main_function);

        let total_complexity = functions.iter().map(|f| f.complexity).sum();

        Ok(ComposedProgram {
            name: format!("combined_{}_functions", functions.len()),
            code: combined_code,
            functions: renamed_functions,
            complexity: total_complexity,
            main_function,
        })
    }

    pub fn create_chained_composition(&self, functions: &[GeneratedFunction]) -> Result<ComposedProgram> {
        if functions.len() < 2 {
            return Err(anyhow!("Need at least 2 functions for chaining"));
        }

        // Analyze function signatures to find compatible chains
        let signatures = functions
            .iter()
            .map(|f| self.extract_function_signature(&f.code))
            .collect::<Result<Vec<_>>>()?;

        // Find compatible function pairs for chaining
        let chain = self.find_compatible_chain(&signatures)?;
        
        if chain.len() < 2 {
            return Err(anyhow!("No compatible function chain found"));
        }

        // Build the chained composition
        let chained_functions: Vec<_> = chain.iter().map(|&i| functions[i].clone()).collect();
        let mut combined_code = String::new();

        // Add all individual functions
        for func in &chained_functions {
            combined_code.push_str(&func.code);
            combined_code.push_str("\n\n");
        }

        // Create a main function that chains the calls
        let main_function = self.create_chained_main(&chained_functions)?;
        combined_code.push_str(&main_function);

        let total_complexity = chained_functions.iter().map(|f| f.complexity).sum::<u32>() + 2; // +2 for chaining complexity

        Ok(ComposedProgram {
            name: format!("chained_{}_functions", chained_functions.len()),
            code: combined_code,
            functions: chained_functions,
            complexity: total_complexity,
            main_function,
        })
    }

    fn extract_function_name(&self, code: &str) -> Result<String> {
        let fn_regex = Regex::new(r"fn\s+(\w+)\s*\(")?;
        if let Some(captures) = fn_regex.captures(code) {
            Ok(captures.get(1).unwrap().as_str().to_string())
        } else {
            Err(anyhow!("Could not extract function name from code"))
        }
    }

    fn rename_function(&self, code: &str, old_name: &str, new_name: &str) -> Result<String> {
        let fn_regex = Regex::new(&format!(r"\bfn\s+{}\b", regex::escape(old_name)))?;
        Ok(fn_regex.replace(code, &format!("fn {}", new_name)).to_string())
    }

    fn extract_function_signature(&self, code: &str) -> Result<(String, Vec<String>, String)> {
        let fn_regex = Regex::new(r"fn\s+(\w+)\s*\((.*?)\)\s*(?:->\s*([^{]+))?")?;
        
        if let Some(captures) = fn_regex.captures(code) {
            let name = captures.get(1).unwrap().as_str().to_string();
            let params_str = captures.get(2).map_or("", |m| m.as_str());
            let return_type = captures.get(3).map_or("()", |m| m.as_str().trim()).to_string();
            
            // Parse parameters (simplified)
            let params = if params_str.trim().is_empty() {
                Vec::new()
            } else {
                params_str
                    .split(',')
                    .map(|p| {
                        // Extract type from "name: type" format
                        p.split(':')
                            .nth(1)
                            .unwrap_or(p)
                            .trim()
                            .to_string()
                    })
                    .collect()
            };
            
            Ok((name, params, return_type))
        } else {
            Err(anyhow!("Could not parse function signature"))
        }
    }

    fn find_compatible_chain(&self, signatures: &[(String, Vec<String>, String)]) -> Result<Vec<usize>> {
        // Simple greedy approach: find functions where output type matches next input type
        let mut chain = Vec::new();
        let mut used = vec![false; signatures.len()];
        
        // Start with the first function
        if !signatures.is_empty() {
            chain.push(0);
            used[0] = true;
        }
        
        while let Some(&last_idx) = chain.last() {
            let last_return_type = &signatures[last_idx].2;
            let mut found_next = false;
            
            for (i, (_, params, _)) in signatures.iter().enumerate() {
                if used[i] || params.is_empty() {
                    continue;
                }
                
                if self.types_compatible(last_return_type, &params[0]) {
                    chain.push(i);
                    used[i] = true;
                    found_next = true;
                    break;
                }
            }
            
            if !found_next {
                break;
            }
        }
        
        Ok(chain)
    }

    fn types_compatible(&self, output_type: &str, input_type: &str) -> bool {
        if output_type == input_type {
            return true;
        }
        
        if let Some(compatible_types) = self.type_compatibility.get(output_type) {
            compatible_types.contains(&input_type.to_string())
        } else {
            false
        }
    }

    fn create_main_function(&self, functions: &[GeneratedFunction]) -> Result<String> {
        let mut main_body = String::new();
        main_body.push_str("fn main() {\n");
        main_body.push_str("    println!(\"Running combined functions:\");\n\n");
        
        for func in functions {
            let func_name = self.extract_function_name(&func.code)?;
            // Create simple calls (this is a basic implementation)
            main_body.push_str(&format!("    // Call {}\n", func_name));
            main_body.push_str(&format!("    // {}();\n\n", func_name));
        }
        
        main_body.push_str("    println!(\"All functions completed.\");\n");
        main_body.push_str("}\n");
        
        Ok(main_body)
    }

    fn create_chained_main(&self, functions: &[GeneratedFunction]) -> Result<String> {
        let mut main_body = String::new();
        main_body.push_str("fn main() {\n");
        main_body.push_str("    println!(\"Running chained functions:\");\n\n");
        
        // This is a simplified chaining - in practice, you'd need more sophisticated type analysis
        for (i, func) in functions.iter().enumerate() {
            let func_name = self.extract_function_name(&func.code)?;
            main_body.push_str(&format!("    // Step {}: {}\n", i + 1, func_name));
            main_body.push_str(&format!("    // let result_{} = {}(/* appropriate params */);\n\n", i, func_name));
        }
        
        main_body.push_str("    println!(\"Chain execution completed.\");\n");
        main_body.push_str("}\n");
        
        Ok(main_body)
    }
}