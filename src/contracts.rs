//! Contracts information module

use reqwest;
use serde_json;

/// Gets contracts information from the SpaceTraders API
pub async fn get_contracts(
    client: &reqwest::Client,
    token: &str
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Getting Contracts ===");
    
    let contracts_url = "https://api.spacetraders.io/v2/my/contracts";
    
    println!("Making request to contracts endpoint...");
    let contracts_response = client
        .get(contracts_url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
        
    println!("Contracts response status: {}", contracts_response.status());
    
    let contracts_text = contracts_response.text().await?;
    println!("\n=== Contract Information ===");
    
    // Parse and pretty print contracts
    match serde_json::from_str::<serde_json::Value>(&contracts_text) {
        Ok(contracts_value) => {
            if let Some(contracts_array) = contracts_value.get("data") {
                if let Some(contracts) = contracts_array.as_array() {
                    if contracts.is_empty() {
                        println!("No active contracts found");
                    } else {
                        for (index, contract) in contracts.iter().enumerate() {
                            println!("\n--- Contract #{} ---", index + 1);
                            
                            if let Some(contract_id) = contract.get("id") {
                                println!("ID: {}", contract_id);
                            }
                            
                            if let Some(faction_symbol) = contract.get("factionSymbol") {
                                println!("Faction: {}", faction_symbol);
                            }
                            
                            if let Some(contract_type) = contract.get("type") {
                                println!("Type: {}", contract_type);
                            }
                            
                            if let Some(terms) = contract.get("terms") {
                                if let Some(deliver_array) = terms.get("deliver").and_then(|d| d.as_array()) {
                                    if let Some(deliver) = deliver_array.first() {
                                        if let Some(trade_symbol) = deliver.get("tradeSymbol") {
                                            println!("Delivery Item: {}", trade_symbol);
                                        }
                                        if let Some(destination) = deliver.get("destinationSymbol") {
                                            println!("Destination: {}", destination);
                                        }
                                        if let Some(units_required) = deliver.get("unitsRequired") {
                                            println!("Units Required: {}", units_required);
                                        }
                                    }
                                }
                                
                                if let Some(payment) = terms.get("payment") {
                                    if let Some(on_fulfilled) = payment.get("onFulfilled") {
                                        println!("Payment on Fulfillment: {} credits", on_fulfilled);
                                    }
                                }
                            }
                            
                            if let Some(accepted) = contract.get("accepted") {
                                println!("Accepted: {}", accepted);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Error parsing contracts data: {:?}", e);
        }
    }
    
    Ok(())
}
