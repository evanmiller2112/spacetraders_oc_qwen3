//! Agent management module for SpaceTraders API

use reqwest;
use serde::{Deserialize, Serialize};

/// Structure to hold agent data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentInfo {
    pub symbol: String,
    pub faction: String,
    pub credits: i64,
    pub headquarters: String,
    pub system_symbol: Option<String>,
}

/// Structure to hold registration data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterResponse {
    pub agent: AgentInfo,
    pub token: String,
}

/// Structure for registration request
#[derive(Debug, Clone, Serialize)]
pub struct RegisterRequest {
    pub faction: String,
    pub symbol: String,
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
    println!("Raw agent data: {}...", &raw_text[..std::cmp::min(200, raw_text.len())]);
    
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

/// Registers a new agent with the SpaceTraders API
pub async fn register_agent(
    client: &reqwest::Client,
    faction: &str,
    symbol: &str
) -> Result<RegisterResponse, Box<dyn std::error::Error>> {
    println!("\n=== Registering New Agent ===");
    
    let url = "https://api.spacetraders.io/v2/register";
    
    let register_request = RegisterRequest {
        faction: faction.to_string(),
        symbol: symbol.to_string(),
    };
    
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&register_request)
        .send()
        .await?;
        
    println!("Registration response status: {}", response.status());
    
    let raw_text = response.text().await?;
    println!("Raw registration data: {}...", &raw_text[..std::cmp::min(200, raw_text.len())]);
    
    // Try to parse the response
    let register_response = match serde_json::from_str::<serde_json::Value>(&raw_text) {
        Ok(value) => {
            let data = value.get("data").unwrap_or(&serde_json::Value::Null);
            
            println!("\n=== Parsed Registration Response ===");
            
            // Extract agent info
            let agent_data = data.get("agent").unwrap_or(&serde_json::Value::Null);
            let symbol = agent_data.get("symbol").and_then(|s| s.as_str()).unwrap_or("").to_string();
            let faction = agent_data.get("startingFaction").and_then(|s| s.as_str()).unwrap_or("").to_string();
            let credits = agent_data.get("credits").and_then(|c| c.as_i64()).unwrap_or(0);
            let headquarters = agent_data.get("headquarters").and_then(|s| s.as_str()).unwrap_or("").to_string();
            
            // Try to get system_symbol from the agent's location
            let system_symbol = agent_data.get("location").and_then(|l| l.get("systemSymbol")).and_then(|s| s.as_str()).map(|s| s.to_string());
            
            let agent_info = AgentInfo {
                symbol,
                faction,
                credits,
                headquarters,
                system_symbol,
            };
            
            // Extract token
            let token = data.get("token").and_then(|t| t.as_str()).unwrap_or("").to_string();
            
            println!("Agent Symbol: {}", agent_info.symbol);
            println!("Faction: {}", agent_info.faction);
            println!("Credits: {}", agent_info.credits);
            println!("Headquarters: {}", agent_info.headquarters);
            if let Some(system) = &agent_info.system_symbol {
                println!("Current System: {}", system);
            }
            println!("Token: {}...", &token[..std::cmp::min(10, token.len())]);
            
            RegisterResponse {
                agent: agent_info,
                token,
            }
        }
        Err(e) => {
            println!("\nError parsing registration data: {:?}", e);
            return Err("Failed to parse registration response".into());
        }
    };
    
    Ok(register_response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[tokio::test]
    async fn test_agent_info_struct() {
        let agent = AgentInfo {
            symbol: "AGENT-123".to_string(),
            faction: "TEST_FACTION".to_string(),
            credits: 1000,
            headquarters: "X1-ABCD-1234".to_string(),
            system_symbol: Some("X1-TT88".to_string()),
        };
        
        assert_eq!(agent.symbol, "AGENT-123");
        assert_eq!(agent.faction, "TEST_FACTION");
        assert_eq!(agent.credits, 1000);
        assert_eq!(agent.headquarters, "X1-ABCD-1234");
        assert_eq!(agent.system_symbol.as_deref(), Some("X1-TT88"));
    }
    
    #[tokio::test]
    async fn test_register_request_struct() {
        let request = RegisterRequest {
            faction: "TEST_FACTION".to_string(),
            symbol: "AGENT-123".to_string(),
        };
        
        assert_eq!(request.faction, "TEST_FACTION");
        assert_eq!(request.symbol, "AGENT-123");
    }
    
    #[tokio::test]
    async fn test_register_response_struct() {
        let agent = AgentInfo {
            symbol: "AGENT-123".to_string(),
            faction: "TEST_FACTION".to_string(),
            credits: 1000,
            headquarters: "X1-ABCD-1234".to_string(),
            system_symbol: Some("X1-TT88".to_string()),
        };
        
        let response = RegisterResponse {
            agent,
            token: "test_token_12345".to_string(),
        };
        
        assert_eq!(response.agent.symbol, "AGENT-123");
        assert_eq!(response.token, "test_token_12345");
    }
}