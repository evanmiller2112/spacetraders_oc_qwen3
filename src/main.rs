//! SpaceTraders Agent - Autonomous game player

use reqwest;
use spacetraders_oc_qwen3::agent;
use spacetraders_oc_qwen3::contracts;
use spacetraders_oc_qwen3::token;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting SpaceTraders Agent...");
    
    // Read the agent token from file
    let clean_token = token::read_token()?;
    
    println!("Using agent token (length: {})", clean_token.len());
    
    let client = reqwest::Client::new();
    
    // Get agent info first
    agent::get_agent_info(&client, &clean_token).await?;
    
    // Try to get contracts
    contracts::get_contracts(&client, &clean_token).await?;

    Ok(())
}