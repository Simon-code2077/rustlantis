#![feature(exact_size_is_empty)]
#![feature(iter_advance_by)]
#![feature(variant_count)]
#![feature(test)]
#![feature(try_blocks)]
#![feature(box_patterns)]

mod generation;
mod literal;
mod llm_optimizer;
mod mem;
mod place_select;
mod pgraph;
mod prompt_templates;
mod ty;

use std::time::Instant;

use clap::{arg, command, value_parser, Arg};
use log::{debug, info};

use crate::generation::GenerationCtx;
use crate::llm_optimizer::LLMConfig;

fn main() {
    env_logger::init();
    let matches = command!()
        .args(&[
            arg!(-d --debug "generate a program where values are printed instead of hashed (slow)"),
            Arg::new("call-syntax")
                .long("call-syntax")
                .value_parser(["v1", "v2", "v3", "v4"])
                .default_value("v4")
                .help("switch between different versions of Call syntaxes"),
            arg!(<seed> "generation seed").value_parser(value_parser!(u64)),
            arg!(--"llm-endpoint" <ENDPOINT> "LLM API endpoint for weight optimization")
                .required(false),
            arg!(--"llm-api-key" <KEY> "LLM API key")
                .required(false),
            arg!(--"llm-frequency" <FREQ> "Optimize weights every N selections")
                .value_parser(value_parser!(usize))
                .default_value("10"),
        ])
        .get_matches();

    let seed: u64 = *matches
        .get_one::<u64>("seed")
        .expect("need an integer as seed");
    let debug_dump = matches.get_one::<bool>("debug").copied().unwrap_or(false);
    
    // Setup LLM configuration
    println!("DEBUG: 开始设置LLM配置...");
    let llm_config = if let Some(endpoint) = matches.get_one::<String>("llm-endpoint") {
        println!("DEBUG: 检测到LLM端点: {}", endpoint);
        LLMConfig {
            enabled: true,
            api_endpoint: endpoint.clone(),
            api_key: matches.get_one::<String>("llm-api-key").cloned(),
            optimization_frequency: *matches.get_one::<usize>("llm-frequency").unwrap(),
            ..Default::default()
        }
    } else {
        println!("DEBUG: 未设置LLM端点，使用默认配置");
        LLMConfig::default()
    };
    println!("DEBUG: LLM配置完成，enabled={}", llm_config.enabled);

    info!("Generating a program with seed {seed}");
    if llm_config.enabled {
        info!("LLM optimization enabled with endpoint: {}", llm_config.api_endpoint);
    }
    
    println!("DEBUG: 创建GenerationCtx...");
    let call_syntax = matches.get_one::<String>("call-syntax").unwrap();
    let genctxt = GenerationCtx::new(seed, debug_dump).with_llm_config(llm_config);
    println!("DEBUG: 开始生成...");
    let time = Instant::now();
    let (program, tcx) = genctxt.generate();
    println!("{}", program.serialize(&tcx, call_syntax.as_str().into()));
    println!("{}", tcx.serialize());
    let dur = time.elapsed();
    debug!("took {}s to generate", dur.as_secs_f32());
}
