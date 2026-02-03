//! World management - regions, collision, etc.
//! 
//! For a small server, this can be very simple.
//! Add complexity only when needed.

use crate::game::Position;

/// Convert world coordinates to region ID (like RS does)
pub fn get_region_id(x: i32, y: i32) -> u32 {
    let region_x = (x >> 6) as u32;
    let region_y = (y >> 6) as u32;
    (region_x << 8) | region_y
}

/// Check if two positions are within viewing distance
pub fn in_view_distance(a: &Position, b: &Position) -> bool {
    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();
    dx <= 15 && dy <= 15 && a.z == b.z
}

/// Simple collision check placeholder
/// TODO: Load collision data from your map files
pub fn can_move_to(_from: &Position, _to: &Position) -> bool {
    // For now, allow all movement
    // Later: check collision maps
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_region_id() {
        // Lumbridge is around 3222, 3218
        let region = get_region_id(3222, 3218);
        assert_eq!(region, 12850); // 50 << 8 | 50 = 12850
    }
    
    #[test]
    fn test_view_distance() {
        let a = Position { x: 100, y: 100, z: 0 };
        let b = Position { x: 110, y: 105, z: 0 };
        let c = Position { x: 200, y: 200, z: 0 };
        
        assert!(in_view_distance(&a, &b));
        assert!(!in_view_distance(&a, &c));
    }
}
