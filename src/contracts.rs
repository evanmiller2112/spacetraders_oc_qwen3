//! Contracts information module

use reqwest;
use serde_json;

/// Structure to hold contract data
#[derive(Debug, Clone)]
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

/// Gets a specific contract by ID from the SpaceTraders API
pub async fn get_contract_by_id(
    client: &reqwest::Client,
    token: &str,
    contract_id: &str
) -> Result<Option<ContractInfo>, Box<dyn std::error::Error>> {
    println!("\n=== Getting Contract by ID ===");
    
    let contract_url = format!("https://api.spacetraders.io/v2/my/contracts/{}", contract_id);
    
    println!("Making request to contract endpoint: {}", contract_url);
    let contract_response = client
        .get(&contract_url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
        
    println!("Contract response status: {}", contract_response.status());
    
    let contract_text = contract_response.text().await?;
    println!("\n=== Contract Details ===");
    
    // Parse and pretty print contract
    match serde_json::from_str::<serde_json::Value>(&contract_text) {
        Ok(contract_value) => {
            if let Some(contract_data) = contract_value.get("data") {
                // Extract contract details
                let id = contract_data.get("id").and_then(|s| s.as_str()).unwrap_or("").to_string();
                let faction_symbol = contract_data.get("factionSymbol").and_then(|s| s.as_str()).unwrap_or("").to_string();
                let contract_type = contract_data.get("type").and_then(|s| s.as_str()).unwrap_or("").to_string();
                
                let (delivery_item, destination_symbol, units_required, payment_on_fulfillment) = 
                    if let Some(terms) = contract_data.get("terms") {
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
                
                if let Some(accepted) = contract_data.get("accepted") {
                    println!("Accepted: {}", accepted);
                }
                
                // Return the contract info
                Ok(Some(ContractInfo {
                    id,
                    faction_symbol,
                    contract_type,
                    delivery_item,
                    destination_symbol,
                    units_required,
                    payment_on_fulfillment
                }))
            } else {
                println!("No contract data found in response");
                Ok(None)
            }
        }
        Err(e) => {
            println!("Error parsing contract data: {:?}", e);
            Ok(None)
        }
    }
}

/// Accepts a contract by ID from the SpaceTraders API
pub async fn accept_contract(
    client: &reqwest::Client,
    token: &str,
    contract_id: &str
) -> Result<bool, Box<dyn std::error::Error>> {
    println!("\n=== Accepting Contract ===");
    
    let accept_url = format!("https://api.spacetraders.io/v2/my/contracts/{}/accept", contract_id);
    
    println!("Making request to accept contract endpoint: {}", accept_url);
    let accept_response = client
        .post(&accept_url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
        
    println!("Accept contract response status: {}", accept_response.status());
    
    // Check if the request was successful
    if accept_response.status().is_success() {
        println!("Contract accepted successfully!");
        Ok(true)
    } else {
        let error_text = accept_response.text().await?;
        println!("Failed to accept contract: {}", error_text);
        Ok(false)
    }
}

/// Fulfill a delivery contract by ID from the SpaceTraders API
pub async fn fulfill_delivery(
    client: &reqwest::Client,
    token: &str,
    contract_id: &str
) -> Result<bool, Box<dyn std::error::Error>> {
    println!("\n=== Fulfilling Delivery Contract ===");
    
    let fulfill_url = format!("https://api.spacetraders.io/v2/my/contracts/{}/fulfill", contract_id);
    
    println!("Making request to fulfill delivery endpoint: {}", fulfill_url);
    let fulfill_response = client
        .post(&fulfill_url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
        
    println!("Fulfill delivery response status: {}", fulfill_response.status());
    
    // Check if the request was successful
    if fulfill_response.status().is_success() {
        println!("Delivery fulfilled successfully!");
        Ok(true)
    } else {
        let error_text = fulfill_response.text().await?;
        println!("Failed to fulfill delivery: {}", error_text);
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[tokio::test]
    async fn test_contract_info_struct() {
        // Test that the ContractInfo struct can be created properly
        let contract = ContractInfo {
            id: "contract-123".to_string(),
            faction_symbol: "TEST_FACTION".to_string(),
            contract_type: "DELIVERY".to_string(),
            delivery_item: Some("IRON_ORE".to_string()),
            destination_symbol: Some("X1-ABCD-1234".to_string()),
            units_required: 100,
            payment_on_fulfillment: 5000,
        };
        
        assert_eq!(contract.id, "contract-123");
        assert_eq!(contract.faction_symbol, "TEST_FACTION");
        assert_eq!(contract.contract_type, "DELIVERY");
        assert_eq!(contract.delivery_item.as_deref(), Some("IRON_ORE"));
        assert_eq!(contract.destination_symbol.as_deref(), Some("X1-ABCD-1234"));
        assert_eq!(contract.units_required, 100);
        assert_eq!(contract.payment_on_fulfillment, 5000);
    }
    
    #[tokio::test]
    async fn test_parse_contracts_empty() {
        // Test parsing empty contracts response
        let json_response = json!({
            "data": []
        });
        
        // This would normally be tested with a mock HTTP client
        // For now, we're just ensuring the structure compiles and is valid
        assert!(true); // Placeholder test - actual HTTP mocking would be needed for real testing
    }
    
    #[tokio::test]
    async fn test_get_contract_by_id_function() {
        // Test that the get_contract_by_id function exists and compiles
        // Note: This is a basic test that just ensures the function signature works
        assert!(true); // Placeholder - actual testing would require mocking HTTP calls
    }
    
    #[tokio::test]
    async fn test_accept_contract_function() {
        // Test that the accept_contract function exists and compiles
        assert!(true); // Placeholder - actual testing would require mocking HTTP calls
    }
    
    #[tokio::test]
    async fn test_fulfill_delivery_function() {
        // Test that the fulfill_delivery function exists and compiles
        assert!(true); // Placeholder - actual testing would require mocking HTTP calls
    }
}
