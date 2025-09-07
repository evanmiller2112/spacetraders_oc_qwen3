//! Asteroid scanning module for finding materials

use reqwest;
use serde_json;

/// Structure to hold asteroid information
#[derive(Debug)]
pub struct AsteroidInfo {
    pub symbol: String,
    pub x: i32,
    pub y: i32,
    pub materials: Vec<String>,
}

/// Finds asteroids in a system that contain specific materials
pub async fn scan_for_asteroids_with_materials(
    client: &reqwest::Client,
    token: &str,
    system_symbol: &str,
    required_materials: &[&str]
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Scanning for resource-rich asteroids ===");
    println!("System: {}", system_symbol);
    println!("Required materials: {:?}", required_materials);

    // First get the system information to find waypoints
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
                    println!("\nFound {} waypoints in system", waypoint_array.len());
                    
                    // Collect all asteroids with their coordinates and materials
                    let mut asteroids: Vec<AsteroidInfo> = Vec::new();
                    
                    for waypoint in waypoint_array {
                        if let Some(waypoint_type) = waypoint.get("type") {
                            // Look for asteroid-related waypoints
                            if let Some(type_str) = waypoint_type.as_str() {
                                if type_str.contains("ASTEROID") || type_str == "ASTEROID_FIELD" {
                                    let waypoint_symbol = waypoint.get("symbol").and_then(|s| s.as_str()).unwrap_or("Unknown");
                                    println!("\nFound asteroid waypoint: {}", waypoint_symbol);
                                    
                                    // Get detailed information about this asteroid
                                    if let Ok(asteroid_info) = check_asteroid_details(client, token, waypoint_symbol).await {
                                        // Check if this asteroid has the required materials
                                        let mut found_materials = Vec::new();
                                        for &material in required_materials {
                                            if asteroid_info.materials.iter().any(|m| m.contains(material) || material.contains(m.as_str())) {
                                                found_materials.push(material.to_string());
                                            }
                                        }
                                        
                                        if !found_materials.is_empty() {
                                            println!("  Found materials: {:?}", found_materials);
                                            asteroids.push(asteroid_info);
                                        } else {
                                            println!("  No matching materials found");
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // If we found asteroids with required materials, find the closest one
                    if !asteroids.is_empty() {
                        println!("\n=== Finding Closest Asteroid ===");
                        
                        // Get current agent position
                        let (current_x, current_y) = get_agent_position(client, token).await?;
                        
                        // Find the closest asteroid
                        let closest_asteroid = find_closest_asteroid(&asteroids, current_x, current_y);
                        
                        if let Some(asteroid) = closest_asteroid {
                            println!("Closest asteroid with required materials:");
                            println!("  Symbol: {}", asteroid.symbol);
                            println!("  Coordinates: ({}, {})", asteroid.x, asteroid.y);
                            println!("  Distance from current position: {:.2} units", 
                                calculate_distance(current_x, current_y, asteroid.x, asteroid.y));
                            println!("  Materials: {:?}", asteroid.materials);
                        } else {
                            println!("No asteroids with required materials found");
                        }
                    } else {
                        println!("\nNo asteroids with required materials found in this system");
                    }
                }
            }
        },
        Err(e) => {
            println!("Error parsing system data: {:?}", e);
        }
    }

    Ok(())
}

/// Get the agent's current position
async fn get_agent_position(
    client: &reqwest::Client,
    token: &str
) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    // Get agent info to get current location
    let url = "https://api.spacetraders.io/v2/my/agent";
    
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
        
    let raw_text = response.text().await?;
    
    // Parse the agent data to get location coordinates
    if let Ok(agent_value) = serde_json::from_str::<serde_json::Value>(&raw_text) {
        if let Some(data) = agent_value.get("data") {
            // Try to get location coordinates
            if let Some(location) = data.get("location") {
                if let Some(x) = location.get("x").and_then(|v| v.as_i64()) {
                    if let Some(y) = location.get("y").and_then(|v| v.as_i64()) {
                        return Ok((x as i32, y as i32));
                    }
                }
            }
        }
    }
    
    // If we can't get coordinates, return default (0, 0)
    Ok((0, 0))
}

/// Check asteroid details including coordinates and materials
async fn check_asteroid_details(
    client: &reqwest::Client,
    token: &str,
    waypoint_symbol: &str
) -> Result<AsteroidInfo, Box<dyn std::error::Error>> {
    // Get waypoint details to check if there are materials
    let waypoint_url = format!("https://api.spacetraders.io/v2/waypoints/{}", waypoint_symbol);
    
    let response = client
        .get(&waypoint_url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
        
    let waypoint_text = response.text().await?;
    
    // Parse the waypoint data
    match serde_json::from_str::<serde_json::Value>(&waypoint_text) {
        Ok(waypoint_value) => {
            if let Some(data) = waypoint_value.get("data") {
                // Get coordinates
                let x = data.get("x").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                let y = data.get("y").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                
                // Get materials from traits
                let mut materials = Vec::new();
                if let Some(traits) = data.get("traits") {
                    if let Some(traits_array) = traits.as_array() {
                        for trait_value in traits_array {
                            if let Some(trait_symbol) = trait_value.get("symbol").and_then(|s| s.as_str()) {
                                materials.push(trait_symbol.to_string());
                            }
                        }
                    }
                }
                
                Ok(AsteroidInfo {
                    symbol: waypoint_symbol.to_string(),
                    x,
                    y,
                    materials
                })
            } else {
                Err("Could not parse asteroid data".into())
            }
        },
        Err(e) => {
            println!("Error parsing waypoint data: {:?}", e);
            Err("Could not parse asteroid details".into())
        }
    }
}

/// Calculate the distance between two points
fn calculate_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
    let dx = (x1 - x2) as f64;
    let dy = (y1 - y2) as f64;
    (dx * dx + dy * dy).sqrt()
}

/// Find the closest asteroid to the given coordinates
fn find_closest_asteroid(asteroids: &[AsteroidInfo], current_x: i32, current_y: i32) -> Option<&AsteroidInfo> {
    asteroids.iter()
        .min_by(|a, b| {
            let dist_a = calculate_distance(current_x, current_y, a.x, a.y);
            let dist_b = calculate_distance(current_x, current_y, b.x, b.y);
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        })
}