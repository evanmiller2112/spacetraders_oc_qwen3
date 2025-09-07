//! Contracts information module

use reqwest;
use serde_json;

/// Structure to hold contract data
#[derive(Debug)]
pub struct ContractInfo {
    pub id: String,
    pub faction_symbol: String,
    pub contract_type: String,
    pub delivery_item: Option<String>,
    pub destination_symbol: Option<String>,
    pub units_required: i64,
    pub payment_on_fulfillment: i64,
}

/// Gets contracts information from the SpaceTraders API
pub async fn get_contracts(
    client: &reqwest::Client,
    token: &str
) -> Result<Option<Vec<ContractInfo>>, Box<dyn std::error::Error>> {
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
    let mut contract_list = Vec::new();
    
    match serde_json::from_str::<serde_json::Value>(&contracts_text) {
        Ok(contracts_value) => {
            if let Some(contracts_array) = contracts_value.get("data") {
                if let Some(contracts) = contracts_array.as_array() {
                    if contracts.is_empty() {
                        println!("No active contracts found");
                    } else {
                        for (index, contract) in contracts.iter().enumerate() {
                            println!("\n--- Contract #{} ---", index + 1);
                            
                            let id = contract.get("id").and_then(|s| s.as_str()).unwrap_or("").to_string();
                            let faction_symbol = contract.get("factionSymbol").and_then(|s| s.as_str()).unwrap_or("").to_string();
                            let contract_type = contract.get("type").and_then(|s| s.as_str()).unwrap_or("").to_string();
                            
                            let (delivery_item, destination_symbol, units_required, payment_on_fulfillment) = 
                                if let Some(terms) = contract.get("terms") {
                                    // Get delivery information
                                    let (delivery_item, destination_symbol, units_required) = 
                                        if let Some(deliver_array) = terms.get("deliver").and_then(|d| d.as_array()) {
                                            if let Some(deliver) = deliver_array.first() {
                                                (
                                                    deliver.get("tradeSymbol").and_then(|s| s.as_str()).map(|s| s.to_string()),
                                                    deliver.get("destinationSymbol").and_then(|s| s.as_str()).map(|s| s.to_string()),
                                                    deliver.get("unitsRequired").and_then(|u| u.as_i64()).unwrap_or(0)
                                                )
                                            } else {
                                                (None, None, 0)
                                            }
                                        } else {
                                            (None, None, 0)
                                        };
                                    
                                    // Get payment information
                                    let payment_on_fulfillment = 
                                        if let Some(payment) = terms.get("payment") {
                                            payment.get("onFulfilled").and_then(|p| p.as_i64()).unwrap_or(0)
                                        } else {
                                            0
                                        };
                                    
                                    (delivery_item, destination_symbol, units_required, payment_on_fulfillment)
                                } else {
                                    (None, None, 0, 0)
                                };
                            
                            println!("ID: {}", id);
                            println!("Faction: {}", faction_symbol);
                            println!("Type: {}", contract_type);
                            
                            if let Some(item) = &delivery_item {
                                println!("Delivery Item: {}", item);
                            }
                            
                            if let Some(destination) = &destination_symbol {
                                println!("Destination: {}", destination);
                            }
                            
                            println!("Units Required: {}", units_required);
                            println!("Payment on Fulfillment: {} credits", payment_on_fulfillment);
                            
                            if let Some(accepted) = contract.get("accepted") {
                                println!("Accepted: {}", accepted);
                            }
                            
                            // Store the contract info
                            contract_list.push(ContractInfo {
                                id,
                                faction_symbol,
                                contract_type,
                                delivery_item,
                                destination_symbol,
                                units_required,
                                payment_on_fulfillment
                            });
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Error parsing contracts data: {:?}", e);
        }
    }
    
    Ok(Some(contract_list))
}
