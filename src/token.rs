//! Token management module

use std::fs;

/// Reads the agent token from file
pub fn read_token() -> Result<String, Box<dyn std::error::Error>> {
    let token_content = fs::read_to_string("AGENT_TOKEN")?;
    let clean_token = token_content.trim();
    
    Ok(clean_token.to_string())
}
