//! SpaceTraders Agent - Autonomous game player

use reqwest;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize)]
struct AgentData {
    #[serde(rename = "startingFaction")]
    starting_faction: String,
    symbol: String, 
    credits: i64,
}

#[derive(Debug, Deserialize)]
struct AgentResponse {
    data: AgentData,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting SpaceTraders Agent...");
    
    // Read the agent token from file
    let token_content = fs::read_to_string("AGENT_TOKEN")?;
    let clean_token = token_content.trim();
    
    println!("Using agent token (length: {})", clean_token.len());
    
    let client = reqwest::Client::new();
    
    // Make the correct API call to get agent info
    let url = "https://api.spacetraders.io/v2/my/agent";
    
    println!("Making request to: {}", url);
    
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", clean_token))
        .send()
        .await?;
        
    println!("Response status: {}", response.status());
    
    // Get the raw text for debugging
    let raw_text = response.text().await?;
    println!("Raw API response: {}", raw_text);
    
    // Parse the full response
    match serde_json::from_str::<AgentResponse>(&raw_text) {
        Ok(agent_response) => {
            let agent = agent_response.data;
            println!("\n=== Agent Information ===");
            println!("Symbol: {}", agent.symbol);
            println!("Faction: {}", agent.starting_faction);
            println!("Credits: {}", agent.credits);
        }
        Err(e) => {
            println!("\nError parsing agent data: {:?}", e);
        }
    }

    Ok(())
}