use crate::composer::combiner::ComposedProgram;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::fs;
use tokio::process::Command as AsyncCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffTestConfig {
    pub backends: Vec<String>,
    pub optimization_levels: Vec<String>,
    pub timeout_seconds: u64,
    pub output_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub program_name: String,
    pub backend: String,
    pub optimization_level: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub execution_time_ms: u128,
    pub compilation_successful: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffTestReport {
    pub program_name: String,
    pub results: Vec<TestResult>,
    pub discrepancies: Vec<Discrepancy>,
    pub summary: TestSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discrepancy {
    pub backend1: String,
    pub backend2: String,
    pub optimization1: String,
    pub optimization2: String,
    pub discrepancy_type: DiscrepancyType,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscrepancyType {
    ExitCode,
    Stdout,
    Stderr,
    CompilationFailure,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSummary {
    pub total_tests: usize,
    pub successful_tests: usize,
    pub failed_tests: usize,
    pub discrepancies_found: usize,
    pub backends_tested: Vec<String>,
}

pub struct DiffTestRunner {
    config: DiffTestConfig,
}

impl DiffTestRunner {
    pub fn new(config: DiffTestConfig) -> Self {
        Self { config }
    }

    pub fn default_config() -> DiffTestConfig {
        DiffTestConfig {
            backends: vec![
                "llvm".to_string(),
                "cranelift".to_string(),
            ],
            optimization_levels: vec![
                "0".to_string(),
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ],
            timeout_seconds: 30,
            output_dir: PathBuf::from("difftest_results"),
        }
    }

    pub async fn run_differential_test(&self, program: &ComposedProgram) -> Result<DiffTestReport> {
        // Create output directory
        fs::create_dir_all(&self.config.output_dir).await?;
        
        // Write program to temporary file
        let temp_file = self.config.output_dir.join(format!("{}.rs", program.name));
        fs::write(&temp_file, &program.code).await?;
        
        let mut results = Vec::new();
        
        // Test with each backend and optimization level
        for backend in &self.config.backends {
            for opt_level in &self.config.optimization_levels {
                match self.run_single_test(&temp_file, backend, opt_level, &program.name).await {
                    Ok(result) => results.push(result),
                    Err(e) => {
                        eprintln!("Test failed for {} -C opt-level={}: {}", backend, opt_level, e);
                        // Create a failure result
                        results.push(TestResult {
                            program_name: program.name.clone(),
                            backend: backend.clone(),
                            optimization_level: opt_level.clone(),
                            exit_code: -1,
                            stdout: String::new(),
                            stderr: e.to_string(),
                            execution_time_ms: 0,
                            compilation_successful: false,
                        });
                    }
                }
            }
        }
        
        // Analyze results for discrepancies
        let discrepancies = self.find_discrepancies(&results);
        
        let summary = TestSummary {
            total_tests: results.len(),
            successful_tests: results.iter().filter(|r| r.compilation_successful && r.exit_code == 0).count(),
            failed_tests: results.iter().filter(|r| !r.compilation_successful || r.exit_code != 0).count(),
            discrepancies_found: discrepancies.len(),
            backends_tested: self.config.backends.clone(),
        };
        
        let report = DiffTestReport {
            program_name: program.name.clone(),
            results,
            discrepancies,
            summary,
        };
        
        // Save report
        let report_file = self.config.output_dir.join(format!("{}_report.json", program.name));
        let report_json = serde_json::to_string_pretty(&report)?;
        fs::write(report_file, report_json).await?;
        
        Ok(report)
    }

    async fn run_single_test(
        &self,
        source_file: &Path,
        backend: &str,
        opt_level: &str,
        program_name: &str,
    ) -> Result<TestResult> {
        let start_time = std::time::Instant::now();
        
        // Compile the program
        let output_file = self.config.output_dir.join(format!("{}_{}_opt{}", program_name, backend, opt_level));
        
        let compile_result = self.compile_program(source_file, &output_file, backend, opt_level).await?;
        
        if !compile_result.compilation_successful {
            return Ok(compile_result);
        }
        
        // Run the compiled program
        let run_result = self.run_program(&output_file).await?;
        
        let execution_time = start_time.elapsed().as_millis();
        
        Ok(TestResult {
            program_name: program_name.to_string(),
            backend: backend.to_string(),
            optimization_level: opt_level.to_string(),
            exit_code: run_result.0,
            stdout: run_result.1,
            stderr: run_result.2,
            execution_time_ms: execution_time,
            compilation_successful: true,
        })
    }

    async fn compile_program(
        &self,
        source_file: &Path,
        output_file: &Path,
        backend: &str,
        opt_level: &str,
    ) -> Result<TestResult> {
        let mut cmd = AsyncCommand::new("rustc");
        
        // Set backend
        match backend {
            "llvm" => {}, // Default backend
            "cranelift" => {
                cmd.arg("-C").arg("codegen-backend=cranelift");
            }
            _ => return Err(anyhow!("Unsupported backend: {}", backend)),
        }
        
        cmd.arg("-C")
            .arg(format!("opt-level={}", opt_level))
            .arg("-o")
            .arg(output_file)
            .arg(source_file)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        let output = cmd.output().await?;
        
        Ok(TestResult {
            program_name: source_file.file_stem().unwrap().to_string_lossy().to_string(),
            backend: backend.to_string(),
            optimization_level: opt_level.to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            execution_time_ms: 0,
            compilation_successful: output.status.success(),
        })
    }

    async fn run_program(&self, executable: &Path) -> Result<(i32, String, String)> {
        let output = AsyncCommand::new(executable)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;
        
        Ok((
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }

    fn find_discrepancies(&self, results: &[TestResult]) -> Vec<Discrepancy> {
        let mut discrepancies = Vec::new();
        
        // Group results by program name
        let mut grouped: HashMap<String, Vec<&TestResult>> = HashMap::new();
        for result in results {
            grouped.entry(result.program_name.clone()).or_default().push(result);
        }
        
        for (_, program_results) in grouped {
            // Compare all pairs of results
            for i in 0..program_results.len() {
                for j in i+1..program_results.len() {
                    let result1 = program_results[i];
                    let result2 = program_results[j];
                    
                    // Check for discrepancies
                    if result1.exit_code != result2.exit_code {
                        discrepancies.push(Discrepancy {
                            backend1: result1.backend.clone(),
                            backend2: result2.backend.clone(),
                            optimization1: result1.optimization_level.clone(),
                            optimization2: result2.optimization_level.clone(),
                            discrepancy_type: DiscrepancyType::ExitCode,
                            details: format!(
                                "Exit codes differ: {} vs {}",
                                result1.exit_code, result2.exit_code
                            ),
                        });
                    }
                    
                    if result1.stdout != result2.stdout {
                        discrepancies.push(Discrepancy {
                            backend1: result1.backend.clone(),
                            backend2: result2.backend.clone(),
                            optimization1: result1.optimization_level.clone(),
                            optimization2: result2.optimization_level.clone(),
                            discrepancy_type: DiscrepancyType::Stdout,
                            details: format!(
                                "Stdout differs:\n1: {}\n2: {}",
                                result1.stdout, result2.stdout
                            ),
                        });
                    }
                    
                    if result1.compilation_successful != result2.compilation_successful {
                        discrepancies.push(Discrepancy {
                            backend1: result1.backend.clone(),
                            backend2: result2.backend.clone(),
                            optimization1: result1.optimization_level.clone(),
                            optimization2: result2.optimization_level.clone(),
                            discrepancy_type: DiscrepancyType::CompilationFailure,
                            details: format!(
                                "Compilation success differs: {} vs {}",
                                result1.compilation_successful, result2.compilation_successful
                            ),
                        });
                    }
                }
            }
        }
        
        discrepancies
    }

    pub async fn run_batch_tests(&self, programs: &[ComposedProgram]) -> Result<Vec<DiffTestReport>> {
        let mut reports = Vec::new();
        
        for program in programs {
            println!("Running differential test for: {}", program.name);
            match self.run_differential_test(program).await {
                Ok(report) => {
                    println!("  ✓ Completed - {} discrepancies found", report.discrepancies.len());
                    reports.push(report);
                }
                Err(e) => {
                    eprintln!("  ✗ Failed: {}", e);
                }
            }
        }
        
        Ok(reports)
    }
}