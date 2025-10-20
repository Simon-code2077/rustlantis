use crate::fuzzer::pipeline::FuzzingSession;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzingStatistics {
    pub total_functions_generated: usize,
    pub total_programs_composed: usize,
    pub total_files_generated: usize,
    pub complexity_distribution: HashMap<u32, usize>,
    pub category_distribution: HashMap<String, usize>,
}

pub struct StatisticsCollector;

impl StatisticsCollector {
    pub fn analyze_sessions(sessions: &[FuzzingSession]) -> FuzzingStatistics {
        let total_functions_generated = sessions.iter().map(|s| s.generated_functions.len()).sum();
        let total_programs_composed = sessions.iter().map(|s| s.composed_programs.len()).sum();
        let total_files_generated = sessions.iter().map(|s| s.output_files.len()).sum();
        
        let mut complexity_distribution = HashMap::new();
        let mut category_distribution = HashMap::new();

        for session in sessions {
            // Analyze complexity and category distribution
            for function in &session.generated_functions {
                *complexity_distribution.entry(function.complexity).or_insert(0) += 1;
                *category_distribution.entry(function.category.clone()).or_insert(0) += 1;
            }
        }

        FuzzingStatistics {
            total_functions_generated,
            total_programs_composed,
            total_files_generated,
            complexity_distribution,
            category_distribution,
        }
    }

    pub fn print_statistics(stats: &FuzzingStatistics) {
        println!("\n📊 Generation Statistics Summary");
        println!("═══════════════════════════════");
        
        println!("\n🔢 Overall Numbers:");
        println!("  Functions Generated: {}", stats.total_functions_generated);
        println!("  Programs Composed: {}", stats.total_programs_composed);
        println!("  Files Generated: {}", stats.total_files_generated);
        
        println!("\n📈 Complexity Distribution:");
        let mut complexity_items: Vec<_> = stats.complexity_distribution.iter().collect();
        complexity_items.sort_by_key(|(k, _)| *k);
        for (complexity, count) in complexity_items {
            println!("  Complexity {}: {}", complexity, count);
        }
        
        println!("\n📂 Category Distribution:");
        let mut category_items: Vec<_> = stats.category_distribution.iter().collect();
        category_items.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        for (category, count) in category_items {
            println!("  {}: {}", category, count);
        }
    }

    pub fn generate_recommendations(stats: &FuzzingStatistics) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Analyze complexity distribution
        let high_complexity_count = stats.complexity_distribution
            .iter()
            .filter(|(complexity, _)| **complexity > 3)
            .map(|(_, count)| count)
            .sum::<usize>();
        
        let total_functions = stats.complexity_distribution.values().sum::<usize>();
        
        if total_functions > 0 {
            let high_complexity_ratio = high_complexity_count as f64 / total_functions as f64;
            if high_complexity_ratio < 0.3 {
                recommendations.push("Consider generating more high-complexity functions for comprehensive testing.".to_string());
            }
            if high_complexity_ratio > 0.8 {
                recommendations.push("Consider adding some simpler functions to ensure baseline coverage.".to_string());
            }
        }
        
        // Analyze category distribution
        let most_common_category = stats.category_distribution
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(name, _)| name);
        
        if let Some(most_common_category) = most_common_category {
            recommendations.push(format!("'{}' is the most generated category. Consider diversifying with other categories.", most_common_category));
        }
        
        // General recommendations
        if stats.total_programs_composed > 0 {
            recommendations.push(format!("Generated {} programs ready for differential testing with difftest binary.", stats.total_programs_composed));
        }
        
        recommendations
    }
}