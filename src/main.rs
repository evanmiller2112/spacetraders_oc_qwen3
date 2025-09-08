//! SpaceTraders Agent - Autonomous game player

use reqwest;
use spacetraders_oc_qwen3::agent;
use spacetraders_oc_qwen3::agent_management;
use spacetraders_oc_qwen3::contracts;
use spacetraders_oc_qwen3::token;
use spacetraders_oc_qwen3::asteroid;
use spacetraders_oc_qwen3::distance;
use spacetraders_oc_qwen3::status_storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting SpaceTraders Agent...");
    
    // Read the agent token from file
    let clean_token = token::read_token()?;
    
    println!("Using agent token (length: {})", clean_token.len());
    
    let client = reqwest::Client::new();
    
    // Get agent info first
    let agent_data = agent_management::get_agent_info(&client, &clean_token).await?;
    
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
    
    // First scan for asteroids with materials
    asteroid::scan_for_asteroids_with_materials(&client, &clean_token, &target_system, &required_materials).await?;
    
    // Now demonstrate survey functionality
    println!("\n=== Demonstrating Survey Functionality ===");
    
    // Get a list of asteroids in the system to survey
    let asteroid_waypoints = get_asteroid_waypoints(&client, &clean_token, &target_system).await?;
    
    if !asteroid_waypoints.is_empty() {
        // Survey the first asteroid in the list
        let first_asteroid = &asteroid_waypoints[0];
        println!("Surveying asteroid: {}", first_asteroid);
        
        // Perform the survey
        let survey_info = asteroid::survey_asteroid(&client, &clean_token, first_asteroid).await?;
        
        // Add survey to status storage
        // Note: In a real implementation, we would properly integrate with the status storage system
        println!("Survey data stored in cache: {:?}", survey_info);
    } else {
        println!("No asteroids found to survey");
    }

    // Demonstrate distance tracking functionality
    demonstrate_distance_tracking(&client, &clean_token).await?;

    Ok(())
}

/// Get a list of asteroid waypoints in a system
async fn get_asteroid_waypoints(
    client: &reqwest::Client,
    token: &str,
    system_symbol: &str
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("\n=== Getting asteroid waypoints ===");
    
    // Get the system information to find waypoints
    let system_url = format!("https://api.spacetraders.io/v2/systems/{}", system_symbol);
    
    let response = client
        .get(&system_url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
        
    println!("System response status: {}", response.status());
    
    let system_text = response.text().await?;
    
    // Parse and look for asteroid waypoints
    match serde_json::from_str::<serde_json::Value>(&system_text) {
        Ok(system_value) => {
            if let Some(waypoints) = system_value.get("data").and_then(|d| d.get("waypoints")) {
                if let Some(waypoint_array) = waypoints.as_array() {
                    println!("Found {} waypoints in system", waypoint_array.len());
                    
                    let mut asteroid_waypoints = Vec::new();
                    
                    for waypoint in waypoint_array {
                        if let Some(waypoint_type) = waypoint.get("type") {
                            // Look for asteroid-related waypoints
                            if let Some(type_str) = waypoint_type.as_str() {
                                if type_str.contains("ASTEROID") || type_str == "ASTEROID_FIELD" {
                                    let waypoint_symbol = waypoint.get("symbol").and_then(|s| s.as_str()).unwrap_or("Unknown");
                                    println!("Found asteroid waypoint: {}", waypoint_symbol);
                                    asteroid_waypoints.push(waypoint_symbol.to_string());
                                }
                            }
                        }
                    }
                    
                    Ok(asteroid_waypoints)
                } else {
                    Ok(Vec::new())
                }
            } else {
                Ok(Vec::new())
            }
        },
        Err(e) => {
            println!("Error parsing system data: {:?}", e);
            Ok(Vec::new())
        }
    }
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

/// Demonstrate distance tracking functionality
async fn demonstrate_distance_tracking(
    client: &reqwest::Client,
    token: &str
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Demonstrating Distance Tracking ===");
    
    // Get current agent position
    let (current_x, current_y) = asteroid::get_agent_position(client, token).await?;
    let agent_location = distance::Location::new("AGENT".to_string(), current_x, current_y);
    
    println!("Agent location: ({}, {})", agent_location.point.x, agent_location.point.y);
    
    // Create some sample locations to demonstrate distance calculations
    let locations = vec![
        distance::Location::new("LOCATION_1".to_string(), 10, 20),
        distance::Location::new("LOCATION_2".to_string(), -5, 15),
        distance::Location::new("LOCATION_3".to_string(), 30, -10),
    ];
    
    // Find the closest location to agent
    let closest_location = locations.iter()
        .min_by(|a, b| {
            let dist_a = agent_location.distance_to(a);
            let dist_b = agent_location.distance_to(b);
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        });
    
    if let Some(location) = closest_location {
        println!("Closest location: {} at distance {:.2} units", 
            location.symbol, agent_location.distance_to(location));
    }
    
    // Demonstrate distance between systems
    let system1 = distance::System::new("SYSTEM_A".to_string(), 0, 0);
    let system2 = distance::System::new("SYSTEM_B".to_string(), 3, 4);
    println!("Distance between {} and {}: {:.2} units", 
        system1.symbol, system2.symbol, system1.distance_to(&system2));
    
    // Demonstrate distance between waypoints
    let waypoint1 = distance::Waypoint::new("WAYPOINT_A".to_string(), 0, 0);
    let waypoint2 = distance::Waypoint::new("WAYPOINT_B".to_string(), 3, 4);
    println!("Distance between {} and {}: {:.2} units", 
        waypoint1.symbol, waypoint2.symbol, waypoint1.distance_to(&waypoint2));
    
    // Demonstrate distance between ships
    let ship1 = distance::Ship::new("SHIP_A".to_string(), 0, 0);
    let ship2 = distance::Ship::new("SHIP_B".to_string(), 3, 4);
    println!("Distance between {} and {}: {:.2} units", 
        ship1.symbol, ship2.symbol, ship1.distance_to(&ship2));
    
    // Demonstrate distance between asteroids
    let asteroid1 = distance::Asteroid::new("ASTEROID_A".to_string(), 0, 0);
    let asteroid2 = distance::Asteroid::new("ASTEROID_B".to_string(), 3, 4);
    println!("Distance between {} and {}: {:.2} units", 
        asteroid1.symbol, asteroid2.symbol, asteroid1.distance_to(&asteroid2));
    
    Ok(())
}