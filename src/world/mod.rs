//! World management - regions, collision, etc.
//!
//! For a small server, this can be very simple.
//! Add complexity only when needed.

use crate::game::Position;
use once_cell::sync::Lazy;
use std::collections::HashSet;

// Hardcoded collision map for Lumbridge area (temporary)
// This covers the main Lumbridge castle and surroundings
// Later: Load from JSON files
static BLOCKED_TILES: Lazy<HashSet<(i32, i32)>> = Lazy::new(|| {
    let mut set = HashSet::new();

    // Lumbridge Castle walls (example - add more as needed)
    // These are approximate coordinates around 3220, 3218

    // North wall of castle
    for x in 3206..=3210 {
        for y in 3228..=3229 {
            set.insert((x, y));
        }
    }

    // South wall of castle
    for x in 3206..=3210 {
        for y in 3206..=3207 {
            set.insert((x, y));
        }
    }

    // West wall of castle
    for x in 3206..=3207 {
        for y in 3207..=3228 {
            set.insert((x, y));
        }
    }

    // East wall of castle
    for x in 3209..=3210 {
        for y in 3207..=3228 {
            set.insert((x, y));
        }
    }

    // Add some trees/obstacles (examples)
    set.insert((3222, 3222));
    set.insert((3223, 3222));
    set.insert((3224, 3222));

    set
});

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

/// Get all players visible from a given position
/// Returns player IDs and references to players within viewing distance
pub fn get_visible_players<'a>(
    position: &Position,
    all_players: &'a dashmap::DashMap<u32, crate::game::Player>,
) -> Vec<dashmap::mapref::multiple::RefMulti<'a, u32, crate::game::Player>> {
    all_players
        .iter()
        .filter(|player_ref| in_view_distance(position, &player_ref.position))
        .collect()
}

/// Get all NPCs visible from a given position
/// Returns NPC IDs and references to NPCs within viewing distance
pub fn get_visible_npcs<'a>(
    position: &Position,
    all_npcs: &'a dashmap::DashMap<u32, crate::game::Npc>,
) -> Vec<dashmap::mapref::multiple::RefMulti<'a, u32, crate::game::Npc>> {
    all_npcs
        .iter()
        .filter(|npc_ref| in_view_distance(position, &npc_ref.position))
        .collect()
}

/// Get nearby region IDs (including the region the position is in)
/// Returns a list of region IDs within 1 region radius
pub fn get_nearby_regions(position: &Position) -> Vec<u32> {
    let base_region_x = (position.x >> 6) as i32;
    let base_region_y = (position.y >> 6) as i32;

    let mut regions = Vec::with_capacity(9);

    // Include center region and 8 surrounding regions
    for dx in -1..=1 {
        for dy in -1..=1 {
            let region_x = (base_region_x + dx) as u32;
            let region_y = (base_region_y + dy) as u32;
            regions.push((region_x << 8) | region_y);
        }
    }

    regions
}

/// Get all ground items visible from a given position
/// Returns ground item IDs and references within viewing distance
pub fn get_visible_ground_items<'a>(
    position: &Position,
    all_items: &'a dashmap::DashMap<u32, crate::game::GroundItem>,
) -> Vec<dashmap::mapref::multiple::RefMulti<'a, u32, crate::game::GroundItem>> {
    all_items
        .iter()
        .filter(|item_ref| in_view_distance(position, &item_ref.position))
        .collect()
}

/// Check if a player can move to a specific position
/// Returns true if movement is allowed, false if blocked
pub fn can_move_to(from: &Position, to: &Position) -> bool {
    // Can't move to different plane without stairs/ladder
    if from.z != to.z {
        return false;
    }

    // Can't move too far in one step (anti-cheat)
    let dx = (to.x - from.x).abs();
    let dy = (to.y - from.y).abs();
    if dx > 2 || dy > 2 {
        return false; // Teleporting is suspicious
    }

    // Check if destination tile is blocked
    if is_blocked(to.x, to.y, to.z) {
        return false;
    }

    // Movement is valid
    true
}

/// Check if a specific tile is blocked (has collision)
pub fn is_blocked(x: i32, y: i32, _z: i32) -> bool {
    // Check hardcoded blocked tiles
    if BLOCKED_TILES.contains(&(x, y)) {
        return true;
    }

    // Add more collision checks here:
    // - Water tiles
    // - Objects
    // - Dynamic obstacles

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{Equipment, Player, Skills};

    // Helper to create test players with all required fields
    fn create_test_player(id: u32, username: &str, x: i32, y: i32, z: i32) -> Player {
        Player {
            id,
            username: username.to_string(),
            position: Position { x, y, z },
            skills: Skills::default(),
            inventory: vec![None; 28],
            equipment: Equipment::default(),
            current_hp: 100,
            combat_target: None,
            last_combat_tick: 0,
            session: None,
            sender: None,
        }
    }

    #[test]
    fn test_region_id() {
        // Lumbridge is around 3222, 3218
        let region = get_region_id(3222, 3218);
        assert_eq!(region, 12850); // 50 << 8 | 50 = 12850
    }

    #[test]
    fn test_view_distance() {
        let a = Position {
            x: 100,
            y: 100,
            z: 0,
        };
        let b = Position {
            x: 110,
            y: 105,
            z: 0,
        };
        let c = Position {
            x: 200,
            y: 200,
            z: 0,
        };

        assert!(in_view_distance(&a, &b));
        assert!(!in_view_distance(&a, &c));
    }

    #[test]
    fn test_collision() {
        let from = Position {
            x: 3220,
            y: 3220,
            z: 0,
        };

        // Should allow movement to open tile
        let open_tile = Position {
            x: 3221,
            y: 3220,
            z: 0,
        };
        assert!(can_move_to(&from, &open_tile));

        // Should block movement to wall
        let wall = Position {
            x: 3206,
            y: 3228,
            z: 0,
        };
        assert!(!can_move_to(&from, &wall));

        // Should block teleporting
        let far_away = Position {
            x: 3230,
            y: 3230,
            z: 0,
        };
        assert!(!can_move_to(&from, &far_away));

        // Should block different plane
        let different_plane = Position {
            x: 3221,
            y: 3220,
            z: 1,
        };
        assert!(!can_move_to(&from, &different_plane));
    }

    #[test]
    fn test_is_blocked() {
        // Wall should be blocked
        assert!(is_blocked(3206, 3228, 0));

        // Open tile should not be blocked
        assert!(!is_blocked(3220, 3220, 0));

        // Tree should be blocked
        assert!(is_blocked(3222, 3222, 0));
    }

    #[test]
    fn test_nearby_regions() {
        let pos = Position {
            x: 3222,
            y: 3218,
            z: 0,
        };

        let regions = get_nearby_regions(&pos);

        // Should have 9 regions (3x3 grid)
        assert_eq!(regions.len(), 9);

        // Should include the center region
        let center_region = get_region_id(3222, 3218);
        assert!(regions.contains(&center_region));
    }

    #[test]
    fn test_visible_players() {
        use dashmap::DashMap;

        let players = DashMap::new();

        // Add nearby player
        let nearby = create_test_player(1, "Nearby", 3222, 3218, 0);
        players.insert(1, nearby);

        // Add far player
        let far = create_test_player(2, "Far", 3300, 3300, 0);
        players.insert(2, far);

        let viewer_pos = Position {
            x: 3220,
            y: 3220,
            z: 0,
        };
        let visible = get_visible_players(&viewer_pos, &players);

        // Should only see nearby player
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].id, 1);
    }

    #[test]
    fn test_player_enters_view_distance() {
        use dashmap::DashMap;

        let players = DashMap::new();

        // Player 1 at position (3220, 3220)
        let player1 = create_test_player(1, "Player1", 3220, 3220, 0);
        players.insert(1, player1);

        // Player 2 starts far away (outside 15-tile radius)
        let player2 = create_test_player(2, "Player2", 3250, 3250, 0);
        players.insert(2, player2);

        let p1_pos = players.get(&1).unwrap().position;
        let p2_pos = players.get(&2).unwrap().position;

        // Initially, players should NOT see each other (too far)
        assert!(!in_view_distance(&p1_pos, &p2_pos));

        // Player 2 moves closer (within 15 tiles)
        if let Some(mut p2) = players.get_mut(&2) {
            p2.position = Position {
                x: 3225, // 5 tiles away from player1
                y: 3220,
                z: 0,
            };
        }

        let p2_new_pos = players.get(&2).unwrap().position;

        // Now players should see each other
        assert!(in_view_distance(&p1_pos, &p2_new_pos));
    }

    #[test]
    fn test_player_leaves_view_distance() {
        use dashmap::DashMap;

        let players = DashMap::new();

        // Player 1 at position (3220, 3220)
        let player1 = create_test_player(1, "Player1", 3220, 3220, 0);
        players.insert(1, player1);

        // Player 2 starts nearby (within 15-tile radius)
        let player2 = create_test_player(2, "Player2", 3225, 3220, 0);
        players.insert(2, player2);

        let p1_pos = players.get(&1).unwrap().position;
        let p2_pos = players.get(&2).unwrap().position;

        // Initially, players should see each other
        assert!(in_view_distance(&p1_pos, &p2_pos));

        // Player 2 moves far away
        if let Some(mut p2) = players.get_mut(&2) {
            p2.position = Position {
                x: 3250, // 30 tiles away
                y: 3250,
                z: 0,
            };
        }

        let p2_new_pos = players.get(&2).unwrap().position;

        // Now players should NOT see each other
        assert!(!in_view_distance(&p1_pos, &p2_new_pos));
    }

    #[test]
    fn test_boundary_visibility() {
        // Test exactly at 15-tile boundary
        let pos1 = Position {
            x: 3200,
            y: 3200,
            z: 0,
        };

        // Exactly 15 tiles away (should be visible)
        let pos2 = Position {
            x: 3215,
            y: 3200,
            z: 0,
        };
        assert!(in_view_distance(&pos1, &pos2));

        // Exactly 15 tiles diagonal
        let pos3 = Position {
            x: 3210,
            y: 3210,
            z: 0,
        };
        assert!(in_view_distance(&pos1, &pos3));

        // 16 tiles away (should NOT be visible)
        let pos4 = Position {
            x: 3216,
            y: 3200,
            z: 0,
        };
        assert!(!in_view_distance(&pos1, &pos4));
    }

    #[test]
    fn test_no_cross_plane_visibility() {
        let ground_floor = Position {
            x: 3220,
            y: 3218,
            z: 0,
        };

        let first_floor = Position {
            x: 3220,
            y: 3218,
            z: 1, // Same x,y but different plane
        };

        // Should NOT be visible (different planes)
        assert!(!in_view_distance(&ground_floor, &first_floor));
    }

    #[test]
    fn test_visibility_with_multiple_players() {
        use dashmap::DashMap;

        let players = DashMap::new();

        // Center player at Lumbridge
        let center = create_test_player(1, "Center", 3222, 3218, 0);
        players.insert(1, center);

        // Nearby player (should be visible)
        let nearby = create_test_player(2, "Nearby", 3230, 3220, 0);
        players.insert(2, nearby);

        // Far player (should NOT be visible)
        let far = create_test_player(3, "Far", 3300, 3300, 0);
        players.insert(3, far);

        // Different plane player (should NOT be visible even if close in x/y)
        let different_plane = create_test_player(4, "Upstairs", 3222, 3218, 1);
        players.insert(4, different_plane);

        let center_pos = players.get(&1).unwrap().position;

        // Get visible players from center position
        let visible = get_visible_players(&center_pos, &players);

        // Should see: self (center), nearby
        // Should NOT see: far, different_plane
        let visible_ids: Vec<u32> = visible.iter().map(|p| p.id).collect();

        assert!(visible_ids.contains(&1)); // Self
        assert!(visible_ids.contains(&2)); // Nearby
        assert!(!visible_ids.contains(&3)); // Far
        assert!(!visible_ids.contains(&4)); // Different plane

        assert_eq!(visible_ids.len(), 2);
    }

    #[test]
    fn test_ground_item_visibility() {
        use crate::game::GroundItem;
        use dashmap::DashMap;

        let ground_items = DashMap::new();

        // Add nearby ground item
        let nearby_item = GroundItem {
            id: 1,
            item_id: 100,
            amount: 5,
            position: Position {
                x: 3225,
                y: 3220,
                z: 0,
            },
            owner_id: None,
            spawn_tick: 0,
        };
        ground_items.insert(1, nearby_item);

        // Add far ground item
        let far_item = GroundItem {
            id: 2,
            item_id: 101,
            amount: 10,
            position: Position {
                x: 3300,
                y: 3300,
                z: 0,
            },
            owner_id: None,
            spawn_tick: 0,
        };
        ground_items.insert(2, far_item);

        let viewer_pos = Position {
            x: 3220,
            y: 3220,
            z: 0,
        };

        let visible = get_visible_ground_items(&viewer_pos, &ground_items);

        // Should only see nearby item
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].id, 1);
        assert_eq!(visible[0].item_id, 100);
    }

    #[test]
    fn test_ground_item_different_planes() {
        use crate::game::GroundItem;
        use dashmap::DashMap;

        let ground_items = DashMap::new();

        // Ground floor item
        let ground_floor = GroundItem {
            id: 1,
            item_id: 100,
            amount: 1,
            position: Position {
                x: 3220,
                y: 3218,
                z: 0,
            },
            owner_id: None,
            spawn_tick: 0,
        };
        ground_items.insert(1, ground_floor);

        // First floor item (same x,y but different z)
        let first_floor = GroundItem {
            id: 2,
            item_id: 101,
            amount: 1,
            position: Position {
                x: 3220,
                y: 3218,
                z: 1,
            },
            owner_id: None,
            spawn_tick: 0,
        };
        ground_items.insert(2, first_floor);

        let viewer_pos = Position {
            x: 3220,
            y: 3218,
            z: 0,
        };

        let visible = get_visible_ground_items(&viewer_pos, &ground_items);

        // Should only see ground floor item
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].id, 1);
        assert_eq!(visible[0].position.z, 0);
    }

    #[test]
    fn test_dialogue_loading() {
        use crate::game::GameState;
        use std::sync::Arc;

        let state = Arc::new(GameState::load_from_files());

        // Skip test if dialogues didn't load (e.g., different working directory in tests)
        if state.dialogues.is_empty() {
            println!("Skipping dialogue test - no dialogue files loaded");
            return;
        }

        // Check that dialogues were loaded
        let dialogue_count = state.dialogues.len();
        assert!(
            dialogue_count >= 2,
            "Should have at least 2 NPC dialogues (Hans and Bob), found {}",
            dialogue_count
        );

        // Just check count, don't hold references
        println!("Loaded {} NPC dialogues", dialogue_count);
    }

    #[test]
    fn test_dialogue_structure() {
        use crate::game::GameState;
        use std::sync::Arc;

        let state = Arc::new(GameState::load_from_files());

        // Skip test if dialogues didn't load
        if state.dialogues.is_empty() {
            println!("Skipping dialogue structure test - no dialogue files loaded");
            return;
        }

        // Test that we can iterate dialogues without holding long-lived references
        for entry in state.dialogues.iter() {
            assert!(!entry.npc_name.is_empty(), "NPC should have a name");
            assert!(
                !entry.dialogues.is_empty(),
                "NPC should have at least one dialogue"
            );

            // Check each dialogue has valid structure
            for dialogue in &entry.dialogues {
                assert!(!dialogue.text.is_empty(), "Dialogue should have text");
                assert!(!dialogue.options.is_empty(), "Dialogue should have options");
            }
        }
    }
}
