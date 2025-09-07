//! Agent information module

use reqwest;
use serde_json;

/// Gets agent information from the SpaceTraders API
pub async fn get_agent_info(
    client: &reqwest::Client,
    token: &str
) -> Result<(), Box<dyn std::error::Error>> {
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
    match serde_json::from_str::<serde_json::Value>(&raw_text) {
        Ok(value) => {
            println!("\n=== Parsed Agent Information ===");
            if let Some(symbol) = value.get("data").and_then(|d| d.get("symbol")) {
                println!("Symbol: {}", symbol);
            }
            if let Some(faction) = value.get("data").and_then(|d| d.get("startingFaction")) {
                println!("Faction: {}", faction);
            }
            if let Some(credits) = value.get("data").and_then(|d| d.get("credits")) {
                println!("Credits: {}", credits);
            }
            if let Some(headquarters) = value.get("data").and_then(|d| d.get("headquarters")) {
                println!("Headquarters: {}", headquarters);
            }
        }
        Err(e) => {
            println!("\nError parsing agent data: {:?}", e);
        }
    }
    
    Ok(())
}
