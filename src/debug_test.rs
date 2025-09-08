use spacetraders_oc_qwen3::status_storage::*;

fn main() {
    let mut storage = StatusStorage::new();
    
    println!("Initial status count: {}", storage.len());
    
    let status = ShipStatus {
        ship_symbol: "SHIP-123".to_string(),
        status_type: ShipStatusType::Idle,
        location: "X1-ABCD-1234".to_string(),
        cargo: vec![],
        fuel: 100,
        last_updated: 0,
        expires_at: Some(3600),
    };
    
    println!("About to update status");
    storage.update_status(status.clone());
    println!("Status count after update: {}", storage.len());
    
    let retrieved = storage.get_status("SHIP-123");
    println!("Retrieved status: {:?}", retrieved);
    
    if let Some(retrieved_status) = retrieved {
        println!("Retrieved status symbol: {}", retrieved_status.ship_symbol);
    } else {
        println!("No status found");
    }
}