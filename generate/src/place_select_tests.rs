#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm_optimizer::{LLMConfig, LLMOptimizer};
    use std::rc::Rc;
    use mir::tyctxt::TyCtxt;

    #[test]
    fn test_llm_config_creation() {
        let config = LLMConfig {
            enabled: true,
            api_endpoint: "http://localhost:11434/v1/chat/completions".to_string(),
            api_key: None,
            model_name: "llama2".to_string(),
            optimization_frequency: 5,
            ..Default::default()
        };
        
        assert!(config.enabled);
        assert_eq!(config.optimization_frequency, 5);
    }

    #[test]
    fn test_place_selector_with_llm() {
        // Create a mock TyCtxt - this is a simplified test
        // In real usage, you'd need a proper TyCtxt instance
        let tcx = Rc::new(TyCtxt::default()); // This assumes TyCtxt has a default
        
        let config = LLMConfig {
            enabled: false, // Disabled for unit test
            ..Default::default()
        };
        
        let selector = PlaceSelector::for_operand(tcx)
            .with_llm_config(config.clone());
        
        assert_eq!(selector.llm_config.enabled, false);
        assert_eq!(selector.optimization_counter, 0);
    }

    #[test]
    fn test_llm_optimizer_creation() {
        let optimizer = LLMOptimizer::new(
            "http://localhost:11434/v1/chat/completions".to_string(),
            None,
        );
        
        assert_eq!(optimizer.api_endpoint, "http://localhost:11434/v1/chat/completions");
        assert!(optimizer.api_key.is_none());
    }

    #[test]
    fn test_optimization_counter() {
        let tcx = Rc::new(TyCtxt::default());
        let config = LLMConfig {
            enabled: true,
            optimization_frequency: 3,
            ..Default::default()
        };
        
        let mut selector = PlaceSelector::for_operand(tcx)
            .with_llm_config(config);
        
        // Test optimization frequency logic
        assert!(!selector.should_optimize_with_llm()); // counter = 1
        assert!(!selector.should_optimize_with_llm()); // counter = 2  
        assert!(selector.should_optimize_with_llm());  // counter = 3, should optimize
        assert!(!selector.should_optimize_with_llm()); // counter = 4
    }
}

// Integration test helper
#[cfg(test)]
pub fn create_test_place_features() -> Vec<crate::llm_optimizer::PlaceFeature> {
    vec![
        crate::llm_optimizer::PlaceFeature {
            place_id: 0,
            type_info: "i32".to_string(),
            is_ref: false,
            is_raw_ptr: false,
            has_known_val: true,
            is_uninit: false,
            complexity: 5,
            has_deref: false,
            is_offsetted: false,
            is_roundtripped: false,
            current_weight: 10,
        },
        crate::llm_optimizer::PlaceFeature {
            place_id: 1,
            type_info: "&mut i32".to_string(),
            is_ref: true,
            is_raw_ptr: false,
            has_known_val: false,
            is_uninit: true,
            complexity: 15,
            has_deref: true,
            is_offsetted: false,
            is_roundtripped: false,
            current_weight: 20,
        },
    ]
}