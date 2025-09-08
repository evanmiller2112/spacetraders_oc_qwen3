//! Status storage system for tracking ship activities and reducing API calls

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// Represents the current status of a ship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipStatus {
    pub ship_symbol: String,
    pub status_type: ShipStatusType,
    pub location: String,
    pub cargo: Vec<CargoItem>,
    pub fuel: i32,
    pub last_updated: u64, // Unix timestamp
    pub expires_at: Option<u64>, // Optional expiration time
}

/// Types of ship statuses we can track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShipStatusType {
    Idle,
    Traveling,
    Mining,
    Delivering,
    Refueling,
    Repairing,
}

/// Represents an item in a ship's cargo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoItem {
    pub trade_symbol: String,
    pub units: i32,
}

/// Represents a survey of an asteroid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Survey {
    pub symbol: String,
    pub deposits: Vec<String>,
    pub expiration: u64, // Unix timestamp when survey expires
    pub size: SurveySize,
}

/// Size of a survey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SurveySize {
    Small,
    Medium,
    Large,
}

/// Represents a scan of an asteroid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scan {
    pub symbol: String,
    pub materials: Vec<ScanMaterial>,
    pub expiration: u64, // Unix timestamp when scan expires
}

/// Material found in a scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanMaterial {
    pub symbol: String,
    pub units: i32,
}

/// Main status storage system
#[derive(Debug)]
pub struct StatusStorage {
    statuses: HashMap<String, ShipStatus>,
    surveys: HashMap<String, Survey>, // Keyed by waypoint symbol
    scans: HashMap<String, Scan>,     // Keyed by waypoint symbol
    max_age_seconds: u64,
}

impl StatusStorage {
    /// Creates a new status storage system with default max age of 300 seconds (5 minutes)
    pub fn new() -> Self {
        Self {
            statuses: HashMap::new(),
            surveys: HashMap::new(),
            scans: HashMap::new(),
            max_age_seconds: 300, // 5 minutes
        }
    }

    /// Creates a new status storage system with custom max age
    pub fn with_max_age(max_age_seconds: u64) -> Self {
        Self {
            statuses: HashMap::new(),
            surveys: HashMap::new(),
            scans: HashMap::new(),
            max_age_seconds,
        }
    }

    /// Updates or creates a ship status
    pub fn update_status(&mut self, status: ShipStatus) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Set the last updated time
        let mut status = status;
        status.last_updated = now;
        
        // Set expiration time if not already set
        if status.expires_at.is_none() {
            status.expires_at = Some(now + self.max_age_seconds);
        }
        
        self.statuses.insert(status.ship_symbol.clone(), status);
    }

    /// Gets a ship's current status, checking if it's still valid
    pub fn get_status(&self, ship_symbol: &str) -> Option<ShipStatus> {
        if let Some(status) = self.statuses.get(ship_symbol) {
            // Check if status is still valid (not expired)
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            if let Some(expires_at) = status.expires_at {
                if now < expires_at {
                    return Some(status.clone());
                }
            } else {
                // If no expiration time, always return the status
                return Some(status.clone());
            }
        }
        
        None
    }

    /// Removes a ship's status from storage
    pub fn remove_status(&mut self, ship_symbol: &str) {
        self.statuses.remove(ship_symbol);
    }

    /// Checks if a ship's status is still valid (not expired)
    pub fn is_valid(&self, ship_symbol: &str) -> bool {
        if let Some(status) = self.statuses.get(ship_symbol) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            if let Some(expires_at) = status.expires_at {
                return now < expires_at;
            }
        }
        
        false
    }

    /// Gets all valid statuses in the storage
    pub fn get_all_valid_statuses(&self) -> Vec<ShipStatus> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        self.statuses
            .values()
            .filter(|status| {
                if let Some(expires_at) = status.expires_at {
                    now < expires_at
                } else {
                    true // No expiration, so always valid
                }
            })
            .cloned()
            .collect()
    }

    /// Clears all expired statuses
    pub fn clear_expired(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Clear expired ship statuses
        self.statuses.retain(|_symbol, status| {
            if let Some(expires_at) = status.expires_at {
                now < expires_at
            } else {
                true // Keep statuses without expiration
            }
        });
        
        // Clear expired surveys
        self.surveys.retain(|_symbol, survey| {
            now < survey.expiration
        });
        
        // Clear expired scans
        self.scans.retain(|_symbol, scan| {
            now < scan.expiration
        });
    }

    /// Gets the number of stored statuses
    pub fn len(&self) -> usize {
        self.statuses.len()
    }

    /// Checks if the storage is empty
    pub fn is_empty(&self) -> bool {
        self.statuses.is_empty() && self.surveys.is_empty() && self.scans.is_empty()
    }

    /// Updates or creates a survey
    pub fn update_survey(&mut self, survey: Survey) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Set expiration time if not already set
        let mut survey = survey;
        if survey.expiration == 0 {
            survey.expiration = now + self.max_age_seconds;
        }
        
        self.surveys.insert(survey.symbol.clone(), survey);
    }

    /// Gets a survey by waypoint symbol
    pub fn get_survey(&self, waypoint_symbol: &str) -> Option<Survey> {
        self.surveys.get(waypoint_symbol).cloned()
    }

    /// Removes a survey from storage
    pub fn remove_survey(&mut self, waypoint_symbol: &str) {
        self.surveys.remove(waypoint_symbol);
    }

    /// Checks if a survey is still valid (not expired)
    pub fn is_survey_valid(&self, waypoint_symbol: &str) -> bool {
        if let Some(survey) = self.surveys.get(waypoint_symbol) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            now < survey.expiration
        } else {
            false
        }
    }

    /// Updates or creates a scan
    pub fn update_scan(&mut self, scan: Scan) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Set expiration time if not already set
        let mut scan = scan;
        if scan.expiration == 0 {
            scan.expiration = now + self.max_age_seconds;
        }
        
        self.scans.insert(scan.symbol.clone(), scan);
    }

    /// Gets a scan by waypoint symbol
    pub fn get_scan(&self, waypoint_symbol: &str) -> Option<Scan> {
        self.scans.get(waypoint_symbol).cloned()
    }

    /// Removes a scan from storage
    pub fn remove_scan(&mut self, waypoint_symbol: &str) {
        self.scans.remove(waypoint_symbol);
    }

    /// Checks if a scan is still valid (not expired)
    pub fn is_scan_valid(&self, waypoint_symbol: &str) -> bool {
        if let Some(scan) = self.scans.get(waypoint_symbol) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            now < scan.expiration
        } else {
            false
        }
    }

    /// Gets all valid surveys in the storage
    pub fn get_all_valid_surveys(&self) -> Vec<Survey> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        self.surveys
            .values()
            .filter(|survey| now < survey.expiration)
            .cloned()
            .collect()
    }

    /// Gets all valid scans in the storage
    pub fn get_all_valid_scans(&self) -> Vec<Scan> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        self.scans
            .values()
            .filter(|scan| now < scan.expiration)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship_status_creation() {
        let status = ShipStatus {
            ship_symbol: "SHIP-123".to_string(),
            status_type: ShipStatusType::Idle,
            location: "X1-ABCD-1234".to_string(),
            cargo: vec![],
            fuel: 100,
            last_updated: 0,
            expires_at: Some(3600),
        };

        assert_eq!(status.ship_symbol, "SHIP-123");
        assert_eq!(status.status_type, ShipStatusType::Idle);
        assert_eq!(status.location, "X1-ABCD-1234");
        assert_eq!(status.fuel, 100);
    }

    #[test]
    fn test_status_storage_creation() {
        let storage = StatusStorage::new();
        assert_eq!(storage.len(), 0);
        assert!(storage.is_empty());
    }

    #[test]
    fn test_update_and_get_status() {
        let mut storage = StatusStorage::new();
        
        let status = ShipStatus {
            ship_symbol: "SHIP-123".to_string(),
            status_type: ShipStatusType::Idle,
            location: "X1-ABCD-1234".to_string(),
            cargo: vec![],
            fuel: 100,
            last_updated: 0,
            expires_at: Some(3600),
        };
        
        storage.update_status(status.clone());
        let retrieved = storage.get_status("SHIP-123");
        
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().ship_symbol, "SHIP-123");
    }

    #[test]
    fn test_status_expiration() {
        let mut storage = StatusStorage::with_max_age(1); // Very short expiration
        
        let status = ShipStatus {
            ship_symbol: "SHIP-123".to_string(),
            status_type: ShipStatusType::Idle,
            location: "X1-ABCD-1234".to_string(),
            cargo: vec![],
            fuel: 100,
            last_updated: 0,
            expires_at: None, // Will be set to current time + max_age
        };
        
        storage.update_status(status);
        
        // Immediately check if status is valid (should be)
        assert!(storage.is_valid("SHIP-123"));
        
        // Clear expired statuses (should remove our status)
        storage.clear_expired();
        assert!(!storage.is_valid("SHIP-123"));
    }

    #[test]
    fn test_survey_storage() {
        let mut storage = StatusStorage::new();
        
        let survey = Survey {
            symbol: "X1-ABCD-1234".to_string(),
            deposits: vec!["IRON_ORE".to_string(), "SILVER".to_string()],
            expiration: 3600,
            size: SurveySize::Large,
        };
        
        storage.update_survey(survey.clone());
        let retrieved = storage.get_survey("X1-ABCD-1234");
        
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().symbol, "X1-ABCD-1234");
    }

    #[test]
    fn test_scan_storage() {
        let mut storage = StatusStorage::new();
        
        let scan = Scan {
            symbol: "X1-ABCD-1234".to_string(),
            materials: vec![
                ScanMaterial {
                    symbol: "IRON_ORE".to_string(),
                    units: 100,
                },
                ScanMaterial {
                    symbol: "SILVER".to_string(),
                    units: 50,
                }
            ],
            expiration: 3600,
        };
        
        storage.update_scan(scan.clone());
        let retrieved = storage.get_scan("X1-ABCD-1234");
        
        assert!(retrieved.is_some());
        let scan = retrieved.unwrap();
        assert_eq!(scan.symbol, "X1-ABCD-1234");
        assert_eq!(scan.materials.len(), 2);
    }
}