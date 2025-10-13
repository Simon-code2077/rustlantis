use crate::fuzzer::pipeline::FuzzingSession;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzingStatistics {
    pub total_functions_generated: usize,
    pub total_programs_composed: usize,
    pub total_bugs_found: usize,
    pub success_rate: f64,
    pub bug_categories: HashMap<String, usize>,
    pub backend_discrepancies: HashMap<String, usize>,
    pub complexity_distribution: HashMap<u32, usize>,
    pub category_effectiveness: HashMap<String, CategoryStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryStats {
    pub functions_generated: usize,
    pub bugs_found: usize,
    pub bug_rate: f64,
}

pub struct StatisticsCollector;

impl StatisticsCollector {
    pub fn analyze_sessions(sessions: &[FuzzingSession]) -> FuzzingStatistics {
        let total_functions_generated = sessions.iter().map(|s| s.generated_functions.len()).sum();
        let total_programs_composed = sessions.iter().map(|s| s.composed_programs.len()).sum();
        let total_bugs_found = sessions.iter().map(|s| s.bugs_found).sum();
        
        let success_rate = if total_programs_composed > 0 {
            total_bugs_found as f64 / total_programs_composed as f64
        } else {
            0.0
        };

        // Analyze bug categories
        let mut bug_categories = HashMap::new();
        let mut backend_discrepancies = HashMap::new();
        let mut complexity_distribution = HashMap::new();
        let mut category_function_counts = HashMap::new();
        let mut category_bug_counts = HashMap::new();

        for session in sessions {
            // Analyze complexity distribution
            for function in &session.generated_functions {
                *complexity_distribution.entry(function.complexity).or_insert(0) += 1;
                *category_function_counts.entry(function.category.clone()).or_insert(0) += 1;
            }

            // Analyze bug categories and backend discrepancies
            for report in &session.test_reports {
                for discrepancy in &report.discrepancies {
                    let discrepancy_type = format!("{:?}", discrepancy.discrepancy_type);
                    *bug_categories.entry(discrepancy_type).or_insert(0) += 1;
                    
                    let backend_pair = format!("{}-{}", discrepancy.backend1, discrepancy.backend2);
                    *backend_discrepancies.entry(backend_pair).or_insert(0) += 1;
                }
                
                // Map bugs back to function categories (simplified approach)
                if !report.discrepancies.is_empty() {
                    // Find the program in the session and its functions
                    if let Some(program) = session.composed_programs.iter().find(|p| p.name == report.program_name) {
                        for function in &program.functions {
                            *category_bug_counts.entry(function.category.clone()).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        // Calculate category effectiveness
        let mut category_effectiveness = HashMap::new();
        for category in category_function_counts.keys() {
            let functions_generated = category_function_counts.get(category).unwrap_or(&0);
            let bugs_found = category_bug_counts.get(category).unwrap_or(&0);
            let bug_rate = if *functions_generated > 0 {
                *bugs_found as f64 / *functions_generated as f64
            } else {
                0.0
            };
            
            category_effectiveness.insert(
                category.clone(),
                CategoryStats {
                    functions_generated: *functions_generated,
                    bugs_found: *bugs_found,
                    bug_rate,
                },
            );
        }

        FuzzingStatistics {
            total_functions_generated,
            total_programs_composed,
            total_bugs_found,
            success_rate,
            bug_categories,
            backend_discrepancies,
            complexity_distribution,
            category_effectiveness,
        }
    }

    pub fn print_statistics(stats: &FuzzingStatistics) {
        println!("\n📊 Fuzzing Statistics Summary");
        println!("═══════════════════════════════");
        
        println!("\n🔢 Overall Numbers:");
        println!("  Functions Generated: {}", stats.total_functions_generated);
        println!("  Programs Composed: {}", stats.total_programs_composed);
        println!("  Bugs Found: {}", stats.total_bugs_found);
        println!("  Success Rate: {:.2}%", stats.success_rate * 100.0);
        
        println!("\n🐛 Bug Categories:");
        for (category, count) in &stats.bug_categories {
            println!("  {}: {}", category, count);
        }
        
        println!("\n⚙️ Backend Discrepancies:");
        for (backend_pair, count) in &stats.backend_discrepancies {
            println!("  {}: {}", backend_pair, count);
        }
        
        println!("\n📈 Complexity Distribution:");
        let mut complexity_items: Vec<_> = stats.complexity_distribution.iter().collect();
        complexity_items.sort_by_key(|(k, _)| *k);
        for (complexity, count) in complexity_items {
            println!("  Complexity {}: {}", complexity, count);
        }
        
        println!("\n🎯 Category Effectiveness:");
        let mut category_items: Vec<_> = stats.category_effectiveness.iter().collect();
        category_items.sort_by(|(_, a), (_, b)| b.bug_rate.partial_cmp(&a.bug_rate).unwrap());
        for (category, stats) in category_items {
            println!(
                "  {}: {:.2}% bug rate ({} bugs / {} functions)",
                category,
                stats.bug_rate * 100.0,
                stats.bugs_found,
                stats.functions_generated
            );
        }
    }

    pub fn generate_recommendations(stats: &FuzzingStatistics) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Analyze success rate
        if stats.success_rate < 0.1 {
            recommendations.push("Consider increasing function complexity or adding more diverse templates.".to_string());
        } else if stats.success_rate > 0.5 {
            recommendations.push("High bug detection rate! Consider focusing on the most effective categories.".to_string());
        }
        
        // Analyze category effectiveness
        let best_category = stats.category_effectiveness
            .iter()
            .max_by(|(_, a), (_, b)| a.bug_rate.partial_cmp(&b.bug_rate).unwrap())
            .map(|(name, _)| name);
        
        if let Some(best_category) = best_category {
            recommendations.push(format!("'{}' category shows highest bug detection rate. Consider generating more functions in this category.", best_category));
        }
        
        // Analyze backend discrepancies
        let total_discrepancies: usize = stats.backend_discrepancies.values().sum();
        if total_discrepancies > 0 {
            let most_problematic_backends = stats.backend_discrepancies
                .iter()
                .max_by_key(|(_, count)| *count)
                .map(|(backends, _)| backends);
            
            if let Some(backends) = most_problematic_backends {
                recommendations.push(format!("Backend pair '{}' shows most discrepancies. Focus testing on these backends.", backends));
            }
        }
        
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
                recommendations.push("Consider generating more high-complexity functions to find deeper bugs.".to_string());
            }
        }
        
        recommendations
    }
}