use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest;
use tokio;
use crate::prompt_templates::PromptTemplate;

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMRequest {
    pub prompt: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMResponse {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeightOptimizationRequest {
    pub usage_type: String,
    pub place_features: Vec<PlaceFeature>,
    pub context_info: ContextInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlaceFeature {
    pub place_id: usize,
    pub type_info: String,
    pub is_ref: bool,
    pub is_raw_ptr: bool,
    pub has_known_val: bool,
    pub is_uninit: bool,
    pub complexity: usize,
    pub has_deref: bool,
    pub is_offsetted: bool,
    pub is_roundtripped: bool,
    pub current_weight: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContextInfo {
    pub current_bb_count: usize,
    pub current_stmt_count: usize,
    pub function_depth: usize,
    pub total_variables: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeightOptimizationResponse {
    pub optimized_weights: Vec<OptimizedWeight>,
    pub reasoning: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptimizedWeight {
    pub place_id: usize,
    pub weight: usize,
    pub adjustment_factor: f32,
}

#[derive(Debug, Clone)]
pub struct LLMConfig {
    pub enabled: bool,
    pub api_endpoint: String,
    pub api_key: Option<String>,
    pub model_name: String,
    pub cache_size_limit: usize,
    pub optimization_frequency: usize, // Optimize every N selections
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_endpoint: "http://localhost:11434/v1/chat/completions".to_string(),
            api_key: None,
            model_name: "llama2".to_string(),
            cache_size_limit: 1000,
            optimization_frequency: 10,
        }
    }
}

pub struct LLMOptimizer {
    api_endpoint: String,
    api_key: Option<String>,
    client: reqwest::Client,
    cache: HashMap<String, WeightOptimizationResponse>,
    prompt_template: PromptTemplate,
}

impl LLMOptimizer {
    pub fn new(api_endpoint: String, api_key: Option<String>) -> Self {
        Self {
            api_endpoint,
            api_key,
            client: reqwest::Client::new(),
            cache: HashMap::new(),
            prompt_template: PromptTemplate::default_optimization_template(),
        }
    }

    pub fn generate_optimization_prompt(&self, request: &WeightOptimizationRequest) -> String {
        let place_candidates = self.prompt_template.format_place_candidates(&request.place_features);
        let current_weights = self.prompt_template.format_weights_analysis(&request.place_features);
        let usage_guidelines = self.prompt_template.usage_specific_guidelines(&request.usage_type);

        self.prompt_template.user_prompt_template
            .replace("{usage_type}", &request.usage_type)
            .replace("{bb_count}", &request.context_info.current_bb_count.to_string())
            .replace("{stmt_count}", &request.context_info.current_stmt_count.to_string())
            .replace("{function_depth}", &request.context_info.function_depth.to_string())
            .replace("{total_variables}", &request.context_info.total_variables.to_string())
            .replace("{place_candidates}", &place_candidates)
            .replace("{current_weights}", &current_weights)
            .replace("{usage_specific_guidelines}", &usage_guidelines)
    }

    pub async fn optimize_weights(&mut self, request: WeightOptimizationRequest) -> Result<WeightOptimizationResponse, Box<dyn std::error::Error>> {
        // Create cache key
        let cache_key = format!("{:?}", request);
        
        // Check cache first
        if let Some(cached_response) = self.cache.get(&cache_key) {
            return Ok(cached_response.clone());
        }

        let system_prompt = &self.prompt_template.system_prompt;
        let user_prompt = self.generate_optimization_prompt(&request);
        
        // Build messages for chat completion
        let messages = serde_json::json!([
            {
                "role": "system",
                "content": system_prompt
            },
            {
                "role": "user", 
                "content": format!("{}\n\n请返回严格的JSON格式响应，不要包含任何markdown标记或其他文本。Expected JSON format:\n{}", user_prompt, self.prompt_template.response_format)
            }
        ]);

        let llm_request = if self.api_endpoint.contains("dashscope.aliyuncs.com") {
            // DashScope (阿里云) format - same as OpenAI format but with specific models
            serde_json::json!({
                "model": "qwen-plus", // Use qwen-plus model for DashScope
                "messages": messages,
                "max_tokens": 2048,
                "temperature": 0.1,
                "stream": false
            })
        } else {
            // OpenAI-compatible format
            serde_json::json!({
                "model": &self.prompt_template.model_name,
                "messages": messages,
                "max_tokens": 2048,
                "temperature": 0.7,
                "response_format": {"type": "json_object"}
            })
        };

        let mut request_builder = self.client
            .post(&self.api_endpoint)
            .json(&llm_request);

        if let Some(ref api_key) = self.api_key {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request_builder.send().await?;
        let response_text = response.text().await?;
        
        // Try to parse as chat completion response (both OpenAI and DashScope use same format)
        let content = if let Ok(chat_response) = serde_json::from_str::<serde_json::Value>(&response_text) {
            if let Some(choices) = chat_response.get("choices").and_then(|c| c.as_array()) {
                if let Some(first_choice) = choices.first() {
                    if let Some(message) = first_choice.get("message") {
                        if let Some(content) = message.get("content").and_then(|c| c.as_str()) {
                            content.to_string()
                        } else {
                            response_text.clone()
                        }
                    } else {
                        response_text.clone()
                    }
                } else {
                    response_text.clone()
                }
            } else {
                response_text.clone()
            }
        } else {
            response_text.clone()
        };

        // Parse the optimization response
        let optimization_response: WeightOptimizationResponse = {
            // Clean the content - remove markdown code blocks if present
            let cleaned_content = if content.contains("```json") {
                content.split("```json").nth(1)
                    .and_then(|s| s.split("```").next())
                    .unwrap_or(&content)
                    .trim()
            } else if content.contains("```") {
                content.split("```").nth(1)
                    .and_then(|s| s.split("```").next())
                    .unwrap_or(&content)
                    .trim()
            } else {
                content.trim()
            };
            
            serde_json::from_str(cleaned_content)
                .unwrap_or_else(|e| {
                    log::warn!("Failed to parse LLM response: {}. Content: {}", e, cleaned_content);
                    // Fallback: create response with minimal adjustments
                    WeightOptimizationResponse {
                        optimized_weights: request.place_features.iter().enumerate().map(|(i, f)| {
                            OptimizedWeight {
                                place_id: i,
                                weight: f.current_weight,
                                adjustment_factor: 1.0,
                            }
                        }).collect(),
                        reasoning: format!("Failed to parse LLM response: {}", e),
                    }
                })
        };

        // Cache the response
        self.cache.insert(cache_key, optimization_response.clone());

        Ok(optimization_response)
    }

    // Synchronous wrapper using blocking HTTP request
    pub fn optimize_weights_sync(&mut self, request: WeightOptimizationRequest) -> Result<WeightOptimizationResponse, Box<dyn std::error::Error>> {
        println!("DEBUG: LLM optimize_weights_sync开始，请求usage_type: {}, place数量: {}", 
                request.usage_type, request.place_features.len());
        
        // Generate cache key
        let cache_key = format!("{:?}", request);
        
        // Check cache first
        if let Some(cached_response) = self.cache.get(&cache_key) {
            println!("DEBUG: 使用缓存响应");
            return Ok(cached_response.clone());
        }

        println!("DEBUG: 生成LLM提示");
        // Generate prompts
        let system_prompt = self.prompt_template.system_prompt.clone();
        let user_prompt = self.generate_optimization_prompt(&request);
        
        // Build messages for chat completion
        let messages = serde_json::json!([
            {
                "role": "system",
                "content": system_prompt
            },
            {
                "role": "user", 
                "content": format!("{}\n\n请返回严格的JSON格式响应，不要包含任何markdown标记或其他文本。Expected JSON format:\n{}", user_prompt, self.prompt_template.response_format)
            }
        ]);

        let llm_request = if self.api_endpoint.contains("dashscope.aliyuncs.com") {
            // DashScope (阿里云) format - same as OpenAI format but with specific models
            serde_json::json!({
                "model": "qwen-plus", // Use qwen-plus model for DashScope
                "messages": messages,
                "max_tokens": 2048,
                "temperature": 0.1,
                "stream": false
            })
        } else {
            // OpenAI-compatible format
            serde_json::json!({
                "model": &self.prompt_template.model_name,
                "messages": messages,
                "max_tokens": 2048,
                "temperature": 0.7,
                "response_format": {"type": "json_object"}
            })
        };

        println!("DEBUG: 构建HTTP请求，端点: {}", self.api_endpoint);
        println!("DEBUG: 请求体内容: {}", serde_json::to_string_pretty(&llm_request).unwrap_or_else(|_| "无法序列化".to_string()));
        
        // Use blocking HTTP client with longer timeout for real API calls
        let blocking_client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(60))  // 增加超时时间
            .connect_timeout(std::time::Duration::from_secs(30))  // 增加连接超时
            .user_agent("rustlantis/1.0")  // 添加User-Agent
            .build()?;
        let mut request_builder = blocking_client
            .post(&self.api_endpoint)
            .json(&llm_request);

        if let Some(ref api_key) = self.api_key {
            println!("DEBUG: 添加API密钥认证");
            request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
        }

        println!("DEBUG: 发送HTTP请求到LLM API，开始连接...");
        println!("DEBUG: 尝试连接到: {}", &self.api_endpoint);
        
        let response = match request_builder.send() {
            Ok(resp) => {
                println!("DEBUG: HTTP请求发送成功，状态码: {}", resp.status());
                resp
            },
            Err(e) => {
                println!("DEBUG: HTTP请求失败详情: {:?}", e);
                if e.is_timeout() {
                    println!("DEBUG: 超时错误 - 可能的原因:");
                    println!("  1. 网络连接问题");
                    println!("  2. 防火墙阻止");
                    println!("  3. 代理设置问题");
                    println!("  4. 服务器响应慢");
                }
                if e.is_connect() {
                    println!("DEBUG: 连接错误 - 无法连接到服务器");
                }
                return Err(format!("HTTP请求失败: {}", e).into());
            }
        };
        let status = response.status();
        println!("DEBUG: HTTP响应状态码: {}", status);
        let response_text = response.text()?;
        println!("DEBUG: 收到LLM响应，长度: {}，内容: {}", response_text.len(), response_text);
        
        // Check for API errors first
        if !status.is_success() {
            return Err(format!("API请求失败，状态码: {}, 响应: {}", status, response_text).into());
        }
        
        // Try to parse as chat completion response (both OpenAI and DashScope use same format)
        let content = if let Ok(chat_response) = serde_json::from_str::<serde_json::Value>(&response_text) {
            if let Some(choices) = chat_response.get("choices").and_then(|c| c.as_array()) {
                if let Some(first_choice) = choices.first() {
                    if let Some(message) = first_choice.get("message") {
                        if let Some(content) = message.get("content").and_then(|c| c.as_str()) {
                            content.to_string()
                        } else {
                            response_text.clone()
                        }
                    } else {
                        response_text.clone()
                    }
                } else {
                    response_text.clone()
                }
            } else {
                response_text.clone()
            }
        } else {
            response_text.clone()
        };

        println!("DEBUG: 解析LLM响应内容");
        // Parse the optimization response
        let optimization_response: WeightOptimizationResponse = {
            // Clean the content - remove markdown code blocks if present
            let cleaned_content = if content.contains("```json") {
                content.split("```json").nth(1)
                    .and_then(|s| s.split("```").next())
                    .unwrap_or(&content)
                    .trim()
            } else if content.contains("```") {
                content.split("```").nth(1)
                    .and_then(|s| s.split("```").next())
                    .unwrap_or(&content)
                    .trim()
            } else {
                content.trim()
            };
            
            println!("DEBUG: 尝试解析清理后的内容，长度: {}，内容: {}", cleaned_content.len(), cleaned_content);
            serde_json::from_str(cleaned_content)
                .unwrap_or_else(|e| {
                    println!("DEBUG: JSON解析失败: {}", e);
                    // Fallback: create response with minimal adjustments
                    WeightOptimizationResponse {
                        optimized_weights: request.place_features.iter().enumerate().map(|(i, f)| {
                            OptimizedWeight {
                                place_id: i,
                                weight: f.current_weight,
                                adjustment_factor: 1.0,
                            }
                        }).collect(),
                        reasoning: format!("Failed to parse LLM response: {}", e),
                    }
                })
        };

        println!("DEBUG: 缓存响应并返回");
        // Cache the response
        self.cache.insert(cache_key, optimization_response.clone());

        Ok(optimization_response)
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn set_api_endpoint(&mut self, endpoint: String) {
        self.api_endpoint = endpoint;
    }

    pub fn set_api_key(&mut self, api_key: Option<String>) {
        self.api_key = api_key;
    }
}