//! SpaceTraders Agent - Autonomous game player

use reqwest;
use spacetraders_oc_qwen3::agent;
use spacetraders_oc_qwen3::contracts;
use spacetraders_oc_qwen3::token;
use spacetraders_oc_qwen3::asteroid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting SpaceTraders Agent...");
    
    // Read the agent token from file
    let clean_token = token::read_token()?;
    
    println!("Using agent token (length: {})", clean_token.len());
    
    let client = reqwest::Client::new();
    
    // Get agent info first
    let agent_data = agent::get_agent_info(&client, &clean_token).await?;
    
    // Try to get contracts
    let _contract_data = contracts::get_contracts(&client, &clean_token).await?;
    
    // Get current system from agent data and scan for asteroids
    let target_system = if let Some(current_system) = &agent_data.system_symbol {
        println!("Scanning current system: {}", current_system);
        current_system.clone()
    } else {
        println!("Could not determine current system. Scanning for valid systems...");
        // Try to find a valid system to work with
        match get_valid_system(&client, &clean_token).await {
            Ok(system) => {
                println!("Using system: {}", system);
                system
            },
            Err(_) => {
                // If we can't find a valid system, use a known good one
                println!("Using default system: X1-TT88");
                "X1-TT88".to_string()
            }
        }
    };
    
    // For now, we'll look for common metal traits in asteroids
    let required_materials = vec!["COMMON_METALS", "RARE_METALS"];
    
    asteroid::scan_for_asteroids_with_materials(&client, &clean_token, &target_system, &required_materials).await?;

    Ok(())
}

/// Get a valid system to work with when we can't determine the current one
async fn get_valid_system(client: &reqwest::Client, token: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Try to get a list of systems from the API
    let systems_url = "https://api.spacetraders.io/v2/systems";
    
    let response = client
        .get(systems_url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
        
    if response.status().is_success() {
        let systems_text = response.text().await?;
        // Parse the system list and return a valid one
        if let Ok(systems_value) = serde_json::from_str::<serde_json::Value>(&systems_text) {
            if let Some(systems_array) = systems_value.get("data").and_then(|d| d.as_array()) {
                // Return the first system in the list as a fallback
                if let Some(first_system) = systems_array.first() {
                    if let Some(symbol) = first_system.get("symbol").and_then(|s| s.as_str()) {
                        return Ok(symbol.to_string());
                    }
                }
            }
        }
    }
    
    // If we can't get a valid system list, return an error
    Err("Could not determine a valid system to work with".into())
}