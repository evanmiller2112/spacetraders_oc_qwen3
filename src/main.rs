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
    headquarters: String,
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
    
    // Get agent info first
    println!("\n=== Getting Agent Information ===");
    let url = "https://api.spacetraders.io/v2/my/agent";
    
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", clean_token))
        .send()
        .await?;
        
    println!("Response status: {}", response.status());
    
    let raw_text = response.text().await?;
    
    match serde_json::from_str::<AgentResponse>(&raw_text) {
        Ok(agent_response) => {
            let agent = agent_response.data;
            println!("\n=== Agent Information ===");
            println!("Symbol: {}", agent.symbol);
            println!("Faction: {}", agent.starting_faction);
            println!("Credits: {}", agent.credits);
            println!("Headquarters: {}", agent.headquarters);
            
            // Now let's get information about our starting system
            println!("\n=== Starting Location Analysis ===");
            
            // The headquarters symbol (e.g. X1-QD10-A1) contains:
            // - X1: system identifier
            // We can get more specific info about our starting location
            
            let parts: Vec<&str> = agent.headquarters.split('-').collect();
            if parts.len() >= 2 {
                let system_symbol = format!("{}-{}", parts[0], parts[1]);
                println!("Starting System: {}", system_symbol);
                
                // Let's try to get more specific waypoint info  
                println!("Starting Waypoint: {}", agent.headquarters);
            }
        }
        Err(e) => {
            println!("\nError parsing agent data: {:?}", e);
        }
    }

    Ok(())
}