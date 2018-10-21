#[derive(Deserialize)]
pub struct Constants {
    #[serde(rename = "CAPTURE_ENABLED")]
    pub capture_enabled: bool,

    #[serde(rename = "CAPTURE_RADIUS")]
    pub capture_radius: usize,

    #[serde(rename = "DEFAULT_MAP_HEIGHT")]
    pub default_map_height: usize,

    #[serde(rename = "DEFAULT_MAP_WIDTH")]
    pub default_map_width: usize,

    #[serde(rename = "DROPOFF_COST")]
    pub dropoff_cost: usize,

    #[serde(rename = "DROPOFF_PENALTY_RATIO")]
    pub dropoff_penalty_ratio: usize,

    #[serde(rename = "EXTRACT_RATIO")]
    pub extract_ratio: usize,

    #[serde(rename = "FACTOR_EXP_1")]
    pub factor_exp_1: f64,

    #[serde(rename = "FACTOR_EXP_2")]
    pub factor_exp_2: f64,

    #[serde(rename = "INITIAL_ENERGY")]
    pub initial_halite: usize,

    #[serde(rename = "INSPIRATION_ENABLED")]
    pub inspiration_enabled: bool,

    #[serde(rename = "INSPIRATION_RADIUS")]
    pub inspiration_radius: usize,

    #[serde(rename = "INSPIRATION_SHIP_COUNT")]
    pub inspiration_ship_count: usize,

    #[serde(rename = "INSPIRED_BONUS_MULTIPLIER")]
    pub inspired_bonus_multiplier: f64,

    #[serde(rename = "INSPIRED_EXTRACT_RATIO")]
    pub inspired_extract_ratio: usize,

    #[serde(rename = "INSPIRED_MOVE_COST_RATIO")]
    pub inspired_move_cost_ratio: usize,

    #[serde(rename = "MAX_CELL_PRODUCTION")]
    pub max_cell_production: usize,

    #[serde(rename = "MAX_ENERGY")]
    pub max_halite: usize,

    #[serde(rename = "MAX_PLAYERS")]
    pub max_players: usize,

    #[serde(rename = "MAX_TURNS")]
    pub max_turns: usize,

    #[serde(rename = "MAX_TURN_THRESHOLD")]
    pub max_turn_threshold: usize,

    #[serde(rename = "MIN_CELL_PRODUCTION")]
    pub min_cell_production: usize,

    #[serde(rename = "MIN_TURNS")]
    pub min_turns: usize,

    #[serde(rename = "MIN_TURN_THRESHOLD")]
    pub min_turn_threshold: usize,

    #[serde(rename = "MOVE_COST_RATIO")]
    pub move_cost_ratio: usize,

    #[serde(rename = "NEW_ENTITY_ENERGY_COST")]
    pub new_entity_halite_cost: usize,

    #[serde(rename = "PERSISTENCE")]
    pub persistence: f64,

    #[serde(rename = "SHIPS_ABOVE_FOR_CAPTURE")]
    pub ships_above_for_capture: usize,

    #[serde(rename = "STRICT_ERRORS")]
    pub strict_errors: bool,

    pub game_seed: usize,
}

static mut CONSTANTS: Option<Constants> = None;

/// Set the global constants.
///
/// # Panics
///
/// If this function is called a second time, this function will panic. The constants can only be
/// set once!
pub fn set(constants: Constants) {
    unsafe {
        if CONSTANTS.is_some() {
            panic!("constants cannot be set a second time")
        } else {
            CONSTANTS = Some(constants);
        }
    }
}

/// Retrieve a reference to the global constants.
///
/// # Panics
///
/// If constants are accessed before being set (i.e. the Game has not started yet), then this
/// function will panic.
pub fn get() -> &'static Constants {
    unsafe {
        match CONSTANTS {
            Some(ref constants) => constants,
            None => panic!("constants were accessed before being set"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Constants;
    use serde_json;

    #[test]
    fn test_json_deserialization() {
        let data = r#"{ "CAPTURE_ENABLED": false,
                        "CAPTURE_RADIUS": 3,
                        "DEFAULT_MAP_HEIGHT": 32,
                        "DEFAULT_MAP_WIDTH": 32,
                        "DROPOFF_COST": 4000,
                        "DROPOFF_PENALTY_RATIO": 4,
                        "EXTRACT_RATIO": 4,
                        "FACTOR_EXP_1": 2.0,
                        "FACTOR_EXP_2": 2.0,
                        "INITIAL_ENERGY": 5000,
                        "INSPIRATION_ENABLED": true,
                        "INSPIRATION_RADIUS": 4,
                        "INSPIRATION_SHIP_COUNT": 2,
                        "INSPIRED_BONUS_MULTIPLIER": 2.0,
                        "INSPIRED_EXTRACT_RATIO": 4,
                        "INSPIRED_MOVE_COST_RATIO": 10,
                        "MAX_CELL_PRODUCTION": 1000,
                        "MAX_ENERGY": 1000,
                        "MAX_PLAYERS": 16,
                        "MAX_TURNS": 400,
                        "MAX_TURN_THRESHOLD": 64,
                        "MIN_CELL_PRODUCTION": 900,
                        "MIN_TURNS": 400,
                        "MIN_TURN_THRESHOLD": 32,
                        "MOVE_COST_RATIO": 10,
                        "NEW_ENTITY_ENERGY_COST": 1000,
                        "PERSISTENCE": 0.7,
                        "SHIPS_ABOVE_FOR_CAPTURE": 3,
                        "STRICT_ERRORS": false,
                        "game_seed": 1539764156
                    }"#;
        let constants: Constants = serde_json::from_str(data).unwrap();
        assert_eq!(constants.capture_enabled, false);
        assert_eq!(constants.capture_radius, 3);
        assert_eq!(constants.inspired_bonus_multiplier, 2.0);
    }
}
