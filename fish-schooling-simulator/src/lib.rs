use wasm_bindgen::prelude::*;

mod agent;
mod behavior;
mod config;
mod vector;

use agent::{Fish, Shark, SharkTarget};
use behavior::{
    best_shark_target, clamp_position_within_bounds, fish_steering, index_direction, move_toward,
    shark_speed_for_target, spawn_fish,
};
use config::{SimulationConfig, default_config};
use vector::Vec2;

#[wasm_bindgen]
pub struct Simulation {
    config: SimulationConfig,
    fish: Vec<Fish>,
    shark: Shark,
    shark_target: Option<SharkTarget>,
    fish_eaten: usize,
    fish_positions: Vec<f32>,
    elapsed_seconds: f32,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::with_config(default_config())
    }

    pub fn reset(&mut self) {
        *self = Self::with_config(self.config);
    }

    pub fn tick(&mut self, delta_seconds: f32) {
        let dt = delta_seconds.max(0.0).min(self.config.max_delta_seconds);

        if dt == 0.0 {
            return;
        }

        self.elapsed_seconds += dt;
        self.update_fish(dt);
        self.fish_eaten += self.eat_colliding_fish();
        self.update_shark(dt);
    }

    pub fn fish_positions(&mut self) -> Vec<f32> {
        self.fish_positions.clear();
        self.fish_positions.reserve(self.fish.len() * 2);
        for fish in &self.fish {
            self.fish_positions.push(fish.position.x);
            self.fish_positions.push(fish.position.y);
        }
        self.fish_positions.clone()
    }

    pub fn shark_position(&self) -> Vec<f32> {
        vec![self.shark.position.x, self.shark.position.y]
    }

    pub fn shark_heading(&self) -> Vec<f32> {
        let heading = self.shark.velocity.normalized_or(Vec2::new(1.0, 0.0));
        vec![heading.x, heading.y]
    }

    pub fn shark_target_position(&self) -> Vec<f32> {
        self.shark_target
            .map(|target| vec![target.position.x, target.position.y])
            .unwrap_or_default()
    }

    pub fn fish_count(&self) -> usize {
        self.fish.len()
    }

    pub fn fish_eaten(&self) -> usize {
        self.fish_eaten
    }

    pub fn fish_eaten_per_minute(&self) -> f32 {
        if self.elapsed_seconds <= f32::EPSILON {
            0.0
        } else {
            self.fish_eaten as f32 / self.elapsed_seconds * 60.0
        }
    }

    pub fn elapsed_seconds(&self) -> f32 {
        self.elapsed_seconds
    }

    pub fn world_width(&self) -> f32 {
        self.config.world_width
    }

    pub fn world_height(&self) -> f32 {
        self.config.world_height
    }

    pub fn fish_personal_space(&self) -> f32 {
        self.config.fish_personal_space
    }

    pub fn shark_detection_radius(&self) -> f32 {
        self.config.fish_flee_radius
    }

    pub fn shark_eat_radius(&self) -> f32 {
        self.config.shark_eat_radius
    }
}

impl Simulation {
    fn with_config(config: SimulationConfig) -> Self {
        Self {
            config,
            fish: spawn_fish(config),
            shark: Shark {
                position: Vec2::new(config.world_width * 0.15, config.world_height * 0.5),
                velocity: Vec2::new(1.0, 0.35).normalized_or(Vec2::new(1.0, 0.0))
                    * config.shark_speed,
            },
            shark_target: None,
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        }
    }

    fn update_fish(&mut self, dt: f32) {
        let previous_fish = self.fish.clone();
        let shark_position = self.shark.position;

        for fish_index in 0..self.fish.len() {
            let steering = fish_steering(
                fish_index,
                &previous_fish,
                shark_position,
                self.config,
                self.elapsed_seconds,
            );
            let fish = &mut self.fish[fish_index];
            let desired_velocity = steering
                .normalized_or(fish.velocity.normalized_or(index_direction(fish_index)))
                * self.config.fish_max_speed;
            fish.velocity = move_toward(
                fish.velocity,
                desired_velocity,
                self.config.fish_acceleration * dt,
            );
            fish.position += fish.velocity * dt;
            clamp_position_within_bounds(
                &mut fish.position,
                self.config.world_width,
                self.config.world_height,
            );
        }
    }

    fn update_shark(&mut self, dt: f32) {
        self.shark_target = best_shark_target(self.shark.position, &self.fish, self.config);

        if let Some(target) = self.shark_target {
            let speed = shark_speed_for_target(target, self.config);
            self.shark.velocity =
                (target.position - self.shark.position).normalized_or(Vec2::new(1.0, 0.0)) * speed;
            self.shark.position += self.shark.velocity * dt;
            clamp_position_within_bounds(
                &mut self.shark.position,
                self.config.world_width,
                self.config.world_height,
            );

            let eaten = self.eat_colliding_fish();
            if eaten > 0 {
                self.fish_eaten += eaten;
                self.shark_target = None;
            }
        } else {
            self.shark.velocity = Vec2::ZERO;
        }
    }

    fn eat_colliding_fish(&mut self) -> usize {
        let before = self.fish.len();
        let shark_position = self.shark.position;
        let eat_radius_squared = self.config.shark_eat_radius * self.config.shark_eat_radius;
        self.fish
            .retain(|fish| (fish.position - shark_position).length_squared() > eat_radius_squared);
        before - self.fish.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::behavior::boundary_steering;

    fn test_config() -> SimulationConfig {
        SimulationConfig {
            world_width: 200.0,
            world_height: 120.0,
            fish_count: 0,
            fish_max_speed: 10.0,
            fish_acceleration: 100.0,
            fish_school_radius: 80.0,
            fish_personal_space: 20.0,
            fish_separation_strength: 2.0,
            fish_alignment_strength: 1.0,
            fish_cohesion_strength: 0.5,
            fish_wander_strength: 0.0,
            fish_flee_radius: 40.0,
            fish_flee_strength: 3.0,
            shark_scan_radius: 200.0,
            shark_speed: 12.0,
            shark_eat_radius: 5.0,
            shark_confusion_radius: 12.0,
            shark_crowding_penalty: 50.0,
            shark_confused_speed_multiplier: 0.5,
            boundary_margin: 20.0,
            boundary_avoidance_strength: 3.0,
            max_delta_seconds: 1.0,
        }
    }

    fn fish_at(x: f32, y: f32) -> Fish {
        Fish {
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
        }
    }

    fn shark_at(x: f32, y: f32) -> Shark {
        Shark {
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
        }
    }

    fn test_simulation(config: SimulationConfig) -> Simulation {
        Simulation {
            config,
            fish: Vec::new(),
            shark: shark_at(0.0, 0.0),
            shark_target: None,
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        }
    }

    #[test]
    fn fish_steers_away_from_nearby_fish() {
        let fish = vec![
            Fish {
                position: Vec2::new(50.0, 50.0),
                velocity: Vec2::new(1.0, 0.0),
            },
            Fish {
                position: Vec2::new(55.0, 50.0),
                velocity: Vec2::new(-1.0, 0.0),
            },
        ];

        let steering = fish_steering(0, &fish, Vec2::new(180.0, 100.0), test_config(), 0.0);

        assert!(steering.x < 0.0);
    }

    #[test]
    fn fish_steers_away_from_shark_inside_flee_radius() {
        let fish = vec![Fish {
            position: Vec2::new(50.0, 50.0),
            velocity: Vec2::new(1.0, 0.0),
        }];

        let steering = fish_steering(0, &fish, Vec2::new(60.0, 50.0), test_config(), 0.0);

        assert!(steering.x < 0.0);
    }

    #[test]
    fn exposed_fish_can_be_selected_over_closer_dense_target() {
        let fish = vec![
            fish_at(30.0, 10.0),
            fish_at(33.0, 10.0),
            fish_at(30.0, 13.0),
            fish_at(33.0, 13.0),
            fish_at(70.0, 10.0),
        ];

        let target = best_shark_target(Vec2::new(10.0, 10.0), &fish, test_config());

        assert_eq!(
            target,
            Some(SharkTarget {
                position: Vec2::new(70.0, 10.0),
                crowding: 0,
            })
        );
    }

    #[test]
    fn shark_speed_is_reduced_for_crowded_target() {
        let config = test_config();

        assert_eq!(
            shark_speed_for_target(
                SharkTarget {
                    position: Vec2::new(30.0, 10.0),
                    crowding: 2,
                },
                config
            ),
            config.shark_speed * config.shark_confused_speed_multiplier
        );
        assert_eq!(
            shark_speed_for_target(
                SharkTarget {
                    position: Vec2::new(30.0, 10.0),
                    crowding: 0,
                },
                config
            ),
            config.shark_speed
        );
    }

    #[test]
    fn tick_moves_crowded_fish_apart() {
        let config = test_config();
        let mut simulation = Simulation {
            config,
            fish: vec![fish_at(50.0, 50.0), fish_at(55.0, 50.0)],
            shark: shark_at(180.0, 100.0),
            shark_target: None,
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };

        simulation.tick(0.1);

        assert!(simulation.fish[0].position.x < 50.0);
        assert!(simulation.fish[1].position.x > 55.0);
    }

    #[test]
    fn tick_moves_shark_toward_target() {
        let config = test_config();
        let mut simulation = Simulation {
            config,
            fish: vec![fish_at(30.0, 10.0), fish_at(180.0, 100.0)],
            shark: shark_at(10.0, 10.0),
            shark_target: None,
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };
        let distance_before = (simulation.fish[0].position - simulation.shark.position).length();

        simulation.tick(0.1);

        let distance_after = (simulation.fish[0].position - simulation.shark.position).length();
        assert!(distance_after < distance_before);
        assert!(simulation.shark_target.is_some());
    }

    #[test]
    fn shark_can_follow_target_to_world_edge() {
        let mut config = test_config();
        config.fish_max_speed = 0.0;
        config.fish_acceleration = 0.0;
        let mut simulation = Simulation {
            config,
            fish: vec![fish_at(0.0, 60.0)],
            shark: shark_at(10.0, 60.0),
            shark_target: None,
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };

        simulation.tick(1.0);

        assert_eq!(simulation.shark.position.x, 0.0);
        assert_eq!(simulation.shark.position.y, 60.0);
    }

    #[test]
    fn fish_keep_schooling_without_nearby_shark() {
        let config = test_config();
        let mut simulation = Simulation {
            config,
            fish: vec![
                Fish {
                    position: Vec2::new(50.0, 50.0),
                    velocity: Vec2::new(6.0, 0.0),
                },
                Fish {
                    position: Vec2::new(80.0, 50.0),
                    velocity: Vec2::new(6.0, 0.0),
                },
            ],
            shark: shark_at(180.0, 100.0),
            shark_target: None,
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };

        simulation.tick(0.1);

        assert!(simulation.fish[0].position.x > 50.0);
        assert!(simulation.fish[1].position.x > 80.0);
        assert!(
            (simulation.fish[1].position - simulation.fish[0].position).length()
                < config.fish_school_radius
        );
    }

    #[test]
    fn shark_eats_fish_on_collision() {
        let config = test_config();
        let mut simulation = Simulation {
            config,
            fish: vec![fish_at(11.0, 10.0), fish_at(150.0, 100.0)],
            shark: shark_at(10.0, 10.0),
            shark_target: None,
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };

        simulation.tick(0.1);

        assert_eq!(simulation.fish.len(), 1);
        assert_eq!(simulation.fish_eaten(), 1);
        assert!(simulation.fish[0].position.x > 100.0);
        assert!(simulation.fish[0].position.y > 80.0);
    }

    #[test]
    fn fish_eaten_per_minute_uses_elapsed_simulation_time() {
        let mut simulation = test_simulation(test_config());

        assert_eq!(simulation.fish_eaten_per_minute(), 0.0);

        simulation.fish_eaten = 3;
        simulation.elapsed_seconds = 30.0;

        assert_eq!(simulation.fish_eaten_per_minute(), 6.0);
    }

    #[test]
    fn elapsed_seconds_reports_simulation_clock() {
        let mut simulation = test_simulation(test_config());

        simulation.tick(0.25);

        assert_eq!(simulation.elapsed_seconds(), 0.25);
    }

    #[test]
    fn boundary_steering_points_inward_near_each_edge() {
        let config = test_config();

        assert!(boundary_steering(Vec2::new(5.0, 60.0), config).x > 0.0);
        assert!(boundary_steering(Vec2::new(195.0, 60.0), config).x < 0.0);
        assert!(boundary_steering(Vec2::new(100.0, 5.0), config).y > 0.0);
        assert!(boundary_steering(Vec2::new(100.0, 115.0), config).y < 0.0);
        assert_eq!(
            boundary_steering(Vec2::new(100.0, 60.0), config),
            Vec2::ZERO
        );
    }

    #[test]
    fn species_positions_are_clamped_without_flipping_velocity() {
        let mut fish = Fish {
            position: Vec2::new(-2.0, 130.0),
            velocity: Vec2::new(-4.0, 5.0),
        };

        clamp_position_within_bounds(&mut fish.position, 200.0, 120.0);

        assert_eq!(fish.position, Vec2::new(0.0, 120.0));
        assert_eq!(fish.velocity, Vec2::new(-4.0, 5.0));
    }

    #[test]
    fn fish_near_edge_moves_back_toward_center() {
        let config = test_config();
        let mut simulation = Simulation {
            config,
            fish: vec![fish_at(5.0, 60.0)],
            shark: shark_at(180.0, 100.0),
            shark_target: None,
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };

        simulation.tick(0.1);

        assert!(simulation.fish[0].position.x > 5.0);
    }
}
