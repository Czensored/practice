#[derive(Clone, Copy, Debug)]
pub struct SimulationConfig {
    pub world_width: f32,
    pub world_height: f32,
    pub world_depth: f32,
    pub fish_count: usize,
    pub fish_max_speed: f32,
    pub fish_acceleration: f32,
    pub fish_school_radius: f32,
    pub fish_personal_space: f32,
    pub fish_separation_strength: f32,
    pub fish_alignment_strength: f32,
    pub fish_cohesion_strength: f32,
    pub fish_wander_strength: f32,
    pub fish_flee_radius: f32,
    pub fish_flee_strength: f32,
    pub shark_scan_radius: f32,
    pub shark_speed: f32,
    pub shark_eat_radius: f32,
    pub shark_confusion_radius: f32,
    pub shark_crowding_penalty: f32,
    pub shark_confusion_full_crowding: f32,
    pub shark_max_confusion_slowdown: f32,
    pub shark_search_speed_multiplier: f32,
    pub shark_search_wander_strength: f32,
    pub shark_search_turn_seconds: f32,
    pub shark_search_turn_acceleration: f32,
    pub boundary_margin: f32,
    pub boundary_avoidance_strength: f32,
    pub max_delta_seconds: f32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            world_width: 960.0,
            world_height: 640.0,
            world_depth: 520.0,
            fish_count: 90,
            fish_max_speed: 96.0,
            fish_acceleration: 540.0,
            fish_school_radius: 108.0,
            fish_personal_space: 34.0,
            fish_separation_strength: 2.4,
            fish_alignment_strength: 1.0,
            fish_cohesion_strength: 0.8,
            fish_wander_strength: 0.45,
            fish_flee_radius: 185.0,
            fish_flee_strength: 3.5,
            shark_scan_radius: 360.0,
            shark_speed: 126.0,
            shark_eat_radius: 13.0,
            shark_confusion_radius: 48.0,
            shark_crowding_penalty: 65.0,
            shark_confusion_full_crowding: 6.0,
            shark_max_confusion_slowdown: 0.45,
            shark_search_speed_multiplier: 0.55,
            shark_search_wander_strength: 0.75,
            shark_search_turn_seconds: 1.4,
            shark_search_turn_acceleration: 180.0,
            boundary_margin: 90.0,
            boundary_avoidance_strength: 3.0,
            max_delta_seconds: 1.0 / 30.0,
        }
    }
}

pub(crate) fn default_config() -> SimulationConfig {
    SimulationConfig::default()
}
