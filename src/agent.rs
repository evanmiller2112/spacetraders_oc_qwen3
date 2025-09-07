//! Agent information module

use reqwest;
use serde_json;

/// Structure to hold agent data
#[derive(Debug)]
pub struct AgentInfo {
    pub symbol: String,
    pub faction: String,
    pub credits: i64,
    pub headquarters: String,
    pub system_symbol: Option<String>,
}

/// Gets agent information from the SpaceTraders API
pub async fn get_agent_info(
    client: &reqwest::Client,
    token: &str
) -> Result<AgentInfo, Box<dyn std::error::Error>> {
    println!("\n=== Getting Agent Information ===");
    
    let url = "https://api.spacetraders.io/v2/my/agent";
    
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
        
    println!("Response status: {}", response.status());
    
    let raw_text = response.text().await?;
    println!("Raw agent data: {}", raw_text);
    
    // Try to parse the response
    let agent_info = match serde_json::from_str::<serde_json::Value>(&raw_text) {
        Ok(value) => {
            let data = value.get("data").unwrap_or(&serde_json::Value::Null);
            
            println!("\n=== Parsed Agent Information ===");
            let symbol = data.get("symbol").and_then(|s| s.as_str()).unwrap_or("").to_string();
            let faction = data.get("startingFaction").and_then(|s| s.as_str()).unwrap_or("").to_string();
            let credits = data.get("credits").and_then(|c| c.as_i64()).unwrap_or(0);
            let headquarters = data.get("headquarters").and_then(|s| s.as_str()).unwrap_or("").to_string();
            
            // Try to get system_symbol from the agent's location
            let system_symbol = data.get("location").and_then(|l| l.get("systemSymbol")).and_then(|s| s.as_str()).map(|s| s.to_string());
            
            println!("Symbol: {}", symbol);
            println!("Faction: {}", faction);
            println!("Credits: {}", credits);
            println!("Headquarters: {}", headquarters);
            if let Some(system) = &system_symbol {
                println!("Current System: {}", system);
            }
            
            AgentInfo {
                symbol,
                faction,
                credits,
                headquarters,
                system_symbol,
            }
        }
        Err(e) => {
            println!("\nError parsing agent data: {:?}", e);
            AgentInfo {
                symbol: "".to_string(),
                faction: "".to_string(),
                credits: 0,
                headquarters: "".to_string(),
                system_symbol: None,
            }
        }
    };
    
    Ok(agent_info)
}
