use wasm_bindgen::prelude::*;

mod agent;
mod behavior;
mod config;
mod vector;

use agent::{Fish2d, Fish3d, Shark2d, Shark3d, SharkTarget2d, SharkTarget3d};
use behavior::{
    best_shark_target_2d, best_shark_target_3d, clamp_position_within_bounds_2d,
    clamp_position_within_bounds_3d, fish_steering_2d, fish_steering_3d, index_direction_2d,
    index_direction_3d, move_toward_2d, move_toward_3d, shark_search_steering_2d,
    shark_search_steering_3d, shark_speed_for_target_2d, shark_speed_for_target_3d, spawn_fish_2d,
    spawn_fish_3d,
};
use config::{SimulationConfig, default_config};
use vector::{Vec2, Vec3};

const TWO_D: u8 = 2;
const THREE_D: u8 = 3;

#[derive(Clone)]
struct World2d {
    fish: Vec<Fish2d>,
    shark: Shark2d,
    shark_target: Option<SharkTarget2d>,
}

#[derive(Clone)]
struct World3d {
    fish: Vec<Fish3d>,
    shark: Shark3d,
    shark_target: Option<SharkTarget3d>,
}

#[derive(Clone)]
enum SimulationState {
    TwoD(World2d),
    ThreeD(World3d),
}

#[wasm_bindgen]
pub struct Simulation {
    config: SimulationConfig,
    state: SimulationState,
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
        let dimensions = self.dimensions();
        self.state = if dimensions == THREE_D {
            SimulationState::ThreeD(Self::new_3d_world(self.config))
        } else {
            SimulationState::TwoD(Self::new_2d_world(self.config))
        };
        self.fish_eaten = 0;
        self.fish_positions.clear();
        self.elapsed_seconds = 0.0;
    }

    pub fn set_dimensions(&mut self, dimensions: u8) {
        match (dimensions, &self.state) {
            (TWO_D, SimulationState::ThreeD(world)) => {
                self.state = SimulationState::TwoD(Self::project_3d_world_to_2d(world));
            }
            (THREE_D, SimulationState::TwoD(world)) => {
                self.state = SimulationState::ThreeD(Self::lift_2d_world_to_3d(world, self.config));
            }
            _ => {}
        }
    }

    pub fn dimensions(&self) -> u8 {
        match self.state {
            SimulationState::TwoD(_) => TWO_D,
            SimulationState::ThreeD(_) => THREE_D,
        }
    }

    pub fn tick(&mut self, delta_seconds: f32) {
        let dt = delta_seconds.max(0.0).min(self.config.max_delta_seconds);

        if dt == 0.0 {
            return;
        }

        self.elapsed_seconds += dt;
        let eaten = match &mut self.state {
            SimulationState::TwoD(world) => {
                Self::tick_2d_world(world, self.config, self.elapsed_seconds, dt)
            }
            SimulationState::ThreeD(world) => {
                Self::tick_3d_world(world, self.config, self.elapsed_seconds, dt)
            }
        };
        self.fish_eaten += eaten;
    }

    pub fn fish_positions(&mut self) -> Vec<f32> {
        self.fish_positions.clear();
        match &self.state {
            SimulationState::TwoD(world) => {
                self.fish_positions.reserve(world.fish.len() * 2);
                for fish in &world.fish {
                    self.fish_positions.push(fish.position.x);
                    self.fish_positions.push(fish.position.y);
                }
            }
            SimulationState::ThreeD(world) => {
                self.fish_positions.reserve(world.fish.len() * 3);
                for fish in &world.fish {
                    self.fish_positions.push(fish.position.x);
                    self.fish_positions.push(fish.position.y);
                    self.fish_positions.push(fish.position.z);
                }
            }
        }
        self.fish_positions.clone()
    }

    pub fn shark_position(&self) -> Vec<f32> {
        match &self.state {
            SimulationState::TwoD(world) => vec![world.shark.position.x, world.shark.position.y],
            SimulationState::ThreeD(world) => vec![
                world.shark.position.x,
                world.shark.position.y,
                world.shark.position.z,
            ],
        }
    }

    pub fn shark_heading(&self) -> Vec<f32> {
        match &self.state {
            SimulationState::TwoD(world) => {
                let heading = world.shark.velocity.normalized_or(Vec2::new(1.0, 0.0));
                vec![heading.x, heading.y]
            }
            SimulationState::ThreeD(world) => {
                let heading = world.shark.velocity.normalized_or(Vec3::new(1.0, 0.0, 0.0));
                vec![heading.x, heading.y, heading.z]
            }
        }
    }

    pub fn shark_target_position(&self) -> Vec<f32> {
        match &self.state {
            SimulationState::TwoD(world) => world
                .shark_target
                .map(|target| vec![target.position.x, target.position.y])
                .unwrap_or_default(),
            SimulationState::ThreeD(world) => world
                .shark_target
                .map(|target| vec![target.position.x, target.position.y, target.position.z])
                .unwrap_or_default(),
        }
    }

    pub fn fish_count(&self) -> usize {
        match &self.state {
            SimulationState::TwoD(world) => world.fish.len(),
            SimulationState::ThreeD(world) => world.fish.len(),
        }
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

    pub fn world_depth(&self) -> f32 {
        self.config.world_depth
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
            state: SimulationState::TwoD(Self::new_2d_world(config)),
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        }
    }

    fn new_2d_world(config: SimulationConfig) -> World2d {
        World2d {
            fish: spawn_fish_2d(config),
            shark: Shark2d {
                position: Vec2::new(config.world_width * 0.15, config.world_height * 0.5),
                velocity: Vec2::new(1.0, 0.35).normalized_or(Vec2::new(1.0, 0.0))
                    * config.shark_speed,
            },
            shark_target: None,
        }
    }

    fn new_3d_world(config: SimulationConfig) -> World3d {
        World3d {
            fish: spawn_fish_3d(config),
            shark: Shark3d {
                position: Vec3::new(
                    config.world_width * 0.15,
                    config.world_height * 0.5,
                    config.world_depth * 0.5,
                ),
                velocity: Vec3::new(1.0, 0.35, 0.1).normalized_or(Vec3::new(1.0, 0.0, 0.0))
                    * config.shark_speed,
            },
            shark_target: None,
        }
    }

    fn tick_2d_world(
        world: &mut World2d,
        config: SimulationConfig,
        elapsed_seconds: f32,
        dt: f32,
    ) -> usize {
        let previous_fish = world.fish.clone();
        let shark_position = world.shark.position;

        for fish_index in 0..world.fish.len() {
            let steering = fish_steering_2d(
                fish_index,
                &previous_fish,
                shark_position,
                config,
                elapsed_seconds,
            );
            let fish = &mut world.fish[fish_index];
            let desired_velocity = steering
                .normalized_or(fish.velocity.normalized_or(index_direction_2d(fish_index)))
                * config.fish_max_speed;
            fish.velocity = move_toward_2d(
                fish.velocity,
                desired_velocity,
                config.fish_acceleration * dt,
            );
            fish.position += fish.velocity * dt;
            clamp_position_within_bounds_2d(
                &mut fish.position,
                config.world_width,
                config.world_height,
            );
        }

        let mut eaten = Self::eat_colliding_fish_2d(world, config);
        world.shark_target = best_shark_target_2d(world.shark.position, &world.fish, config);

        if let Some(target) = world.shark_target {
            let speed = shark_speed_for_target_2d(target, config);
            world.shark.velocity =
                (target.position - world.shark.position).normalized_or(Vec2::new(1.0, 0.0)) * speed;
            world.shark.position += world.shark.velocity * dt;
            clamp_position_within_bounds_2d(
                &mut world.shark.position,
                config.world_width,
                config.world_height,
            );

            let shark_eaten = Self::eat_colliding_fish_2d(world, config);
            if shark_eaten > 0 {
                eaten += shark_eaten;
                world.shark_target = None;
            }
        } else {
            let desired_velocity = shark_search_steering_2d(
                world.shark.position,
                world.shark.velocity,
                elapsed_seconds,
                config,
            )
            .normalized_or(world.shark.velocity.normalized_or(Vec2::new(1.0, 0.0)))
                * config.shark_speed
                * config.shark_search_speed_multiplier;
            world.shark.velocity = move_toward_2d(
                world.shark.velocity,
                desired_velocity,
                config.shark_search_turn_acceleration * dt,
            );
            world.shark.position += world.shark.velocity * dt;
            clamp_position_within_bounds_2d(
                &mut world.shark.position,
                config.world_width,
                config.world_height,
            );
        }

        eaten
    }

    fn tick_3d_world(
        world: &mut World3d,
        config: SimulationConfig,
        elapsed_seconds: f32,
        dt: f32,
    ) -> usize {
        let previous_fish = world.fish.clone();
        let shark_position = world.shark.position;

        for fish_index in 0..world.fish.len() {
            let steering = fish_steering_3d(
                fish_index,
                &previous_fish,
                shark_position,
                config,
                elapsed_seconds,
            );
            let fish = &mut world.fish[fish_index];
            let desired_velocity = steering
                .normalized_or(fish.velocity.normalized_or(index_direction_3d(fish_index)))
                * config.fish_max_speed;
            fish.velocity = move_toward_3d(
                fish.velocity,
                desired_velocity,
                config.fish_acceleration * dt,
            );
            fish.position += fish.velocity * dt;
            clamp_position_within_bounds_3d(
                &mut fish.position,
                config.world_width,
                config.world_height,
                config.world_depth,
            );
        }

        let mut eaten = Self::eat_colliding_fish_3d(world, config);
        world.shark_target = best_shark_target_3d(world.shark.position, &world.fish, config);

        if let Some(target) = world.shark_target {
            let speed = shark_speed_for_target_3d(target, config);
            world.shark.velocity = (target.position - world.shark.position)
                .normalized_or(Vec3::new(1.0, 0.0, 0.0))
                * speed;
            world.shark.position += world.shark.velocity * dt;
            clamp_position_within_bounds_3d(
                &mut world.shark.position,
                config.world_width,
                config.world_height,
                config.world_depth,
            );

            let shark_eaten = Self::eat_colliding_fish_3d(world, config);
            if shark_eaten > 0 {
                eaten += shark_eaten;
                world.shark_target = None;
            }
        } else {
            let desired_velocity = shark_search_steering_3d(
                world.shark.position,
                world.shark.velocity,
                elapsed_seconds,
                config,
            )
            .normalized_or(world.shark.velocity.normalized_or(Vec3::new(1.0, 0.0, 0.0)))
                * config.shark_speed
                * config.shark_search_speed_multiplier;
            world.shark.velocity = move_toward_3d(
                world.shark.velocity,
                desired_velocity,
                config.shark_search_turn_acceleration * dt,
            );
            world.shark.position += world.shark.velocity * dt;
            clamp_position_within_bounds_3d(
                &mut world.shark.position,
                config.world_width,
                config.world_height,
                config.world_depth,
            );
        }

        eaten
    }

    fn eat_colliding_fish_2d(world: &mut World2d, config: SimulationConfig) -> usize {
        let before = world.fish.len();
        let shark_position = world.shark.position;
        let eat_radius_squared = config.shark_eat_radius * config.shark_eat_radius;
        world
            .fish
            .retain(|fish| (fish.position - shark_position).length_squared() > eat_radius_squared);
        before - world.fish.len()
    }

    fn eat_colliding_fish_3d(world: &mut World3d, config: SimulationConfig) -> usize {
        let before = world.fish.len();
        let shark_position = world.shark.position;
        let eat_radius_squared = config.shark_eat_radius * config.shark_eat_radius;
        world
            .fish
            .retain(|fish| (fish.position - shark_position).length_squared() > eat_radius_squared);
        before - world.fish.len()
    }

    fn lift_2d_world_to_3d(world: &World2d, config: SimulationConfig) -> World3d {
        let center = fish_center_2d(&world.fish, config);
        let fish = world
            .fish
            .iter()
            .enumerate()
            .map(|(index, fish)| Fish3d {
                position: Vec3::new(
                    fish.position.x,
                    fish.position.y,
                    generated_depth(index, fish.position, center, config),
                ),
                velocity: lift_velocity(index, fish.velocity),
            })
            .collect();

        World3d {
            fish,
            shark: Shark3d {
                position: Vec3::new(
                    world.shark.position.x,
                    world.shark.position.y,
                    config.world_depth * 0.5,
                ),
                velocity: Vec3::new(world.shark.velocity.x, world.shark.velocity.y, 0.0),
            },
            shark_target: None,
        }
    }

    fn project_3d_world_to_2d(world: &World3d) -> World2d {
        World2d {
            fish: world
                .fish
                .iter()
                .map(|fish| Fish2d {
                    position: Vec2::new(fish.position.x, fish.position.y),
                    velocity: Vec2::new(fish.velocity.x, fish.velocity.y),
                })
                .collect(),
            shark: Shark2d {
                position: Vec2::new(world.shark.position.x, world.shark.position.y),
                velocity: Vec2::new(world.shark.velocity.x, world.shark.velocity.y),
            },
            shark_target: None,
        }
    }
}

fn fish_center_2d(fish: &[Fish2d], config: SimulationConfig) -> Vec2 {
    if fish.is_empty() {
        return Vec2::new(config.world_width * 0.5, config.world_height * 0.5);
    }

    let mut center = Vec2::ZERO;
    for fish in fish {
        center += fish.position;
    }
    center / fish.len() as f32
}

fn generated_depth(
    fish_index: usize,
    position: Vec2,
    school_center: Vec2,
    config: SimulationConfig,
) -> f32 {
    let offset_from_center = (position - school_center).length();
    let radial_scale = (offset_from_center / config.fish_school_radius.max(1.0)).min(1.0);
    let school_thickness = (config.world_depth * 0.34).min(config.fish_school_radius * 1.1);
    let wave = (fish_index as f32 * 1.618_033_9 + position.x * 0.013 + position.y * 0.017).sin();
    let edge_padding = config.boundary_margin.min(config.world_depth * 0.25);

    (config.world_depth * 0.5 + wave * school_thickness * (0.35 + radial_scale * 0.65))
        .clamp(edge_padding, config.world_depth - edge_padding)
}

fn lift_velocity(fish_index: usize, velocity: Vec2) -> Vec3 {
    let speed = velocity.length();
    if speed <= f32::EPSILON {
        Vec3::ZERO
    } else {
        let z_velocity = speed * 0.18 * (fish_index as f32 * 2.399_963_1).sin();
        Vec3::new(velocity.x, velocity.y, z_velocity).normalized_or(Vec3::new(1.0, 0.0, 0.0))
            * speed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::behavior::{boundary_steering_2d, boundary_steering_3d};

    fn test_config() -> SimulationConfig {
        SimulationConfig {
            world_width: 200.0,
            world_height: 120.0,
            world_depth: 100.0,
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
            shark_confusion_full_crowding: 4.0,
            shark_max_confusion_slowdown: 0.4,
            shark_search_speed_multiplier: 0.55,
            shark_search_wander_strength: 0.75,
            shark_search_turn_seconds: 1.0,
            shark_search_turn_acceleration: 100.0,
            boundary_margin: 20.0,
            boundary_avoidance_strength: 3.0,
            max_delta_seconds: 1.0,
        }
    }

    fn fish_2d(x: f32, y: f32) -> Fish2d {
        Fish2d {
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
        }
    }

    fn shark_2d(x: f32, y: f32) -> Shark2d {
        Shark2d {
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
        }
    }

    fn fish_3d(x: f32, y: f32, z: f32) -> Fish3d {
        Fish3d {
            position: Vec3::new(x, y, z),
            velocity: Vec3::ZERO,
        }
    }

    fn shark_3d(x: f32, y: f32, z: f32) -> Shark3d {
        Shark3d {
            position: Vec3::new(x, y, z),
            velocity: Vec3::ZERO,
        }
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.0001,
            "expected {actual} to be close to {expected}"
        );
    }

    fn test_simulation(config: SimulationConfig) -> Simulation {
        Simulation {
            config,
            state: SimulationState::TwoD(World2d {
                fish: Vec::new(),
                shark: shark_2d(0.0, 0.0),
                shark_target: None,
            }),
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        }
    }

    #[test]
    fn simulation_starts_in_2d_with_xy_positions() {
        let mut simulation = test_simulation(test_config());
        if let SimulationState::TwoD(world) = &mut simulation.state {
            world.fish = vec![fish_2d(1.0, 2.0), fish_2d(3.0, 4.0)];
        }

        assert_eq!(simulation.dimensions(), TWO_D);
        assert_eq!(simulation.fish_positions(), vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn fish_steers_away_from_nearby_fish_in_2d() {
        let fish = vec![
            Fish2d {
                position: Vec2::new(50.0, 50.0),
                velocity: Vec2::new(1.0, 0.0),
            },
            Fish2d {
                position: Vec2::new(55.0, 50.0),
                velocity: Vec2::new(-1.0, 0.0),
            },
        ];

        let steering = fish_steering_2d(0, &fish, Vec2::new(180.0, 100.0), test_config(), 0.0);

        assert!(steering.x < 0.0);
    }

    #[test]
    fn fish_steers_away_from_shark_inside_flee_radius_in_2d() {
        let fish = vec![Fish2d {
            position: Vec2::new(50.0, 50.0),
            velocity: Vec2::new(1.0, 0.0),
        }];

        let steering = fish_steering_2d(0, &fish, Vec2::new(60.0, 50.0), test_config(), 0.0);

        assert!(steering.x < 0.0);
    }

    #[test]
    fn exposed_fish_can_be_selected_over_closer_dense_target_in_2d() {
        let fish = vec![
            fish_2d(30.0, 10.0),
            fish_2d(33.0, 10.0),
            fish_2d(30.0, 13.0),
            fish_2d(33.0, 13.0),
            fish_2d(70.0, 10.0),
        ];

        let target = best_shark_target_2d(Vec2::new(10.0, 10.0), &fish, test_config());

        assert_eq!(
            target,
            Some(SharkTarget2d {
                position: Vec2::new(70.0, 10.0),
                crowding: 0,
            })
        );
    }

    #[test]
    fn shark_uses_full_speed_for_uncrowded_target() {
        let config = test_config();
        let target = SharkTarget2d {
            position: Vec2::new(70.0, 10.0),
            crowding: 0,
        };

        assert_close(
            shark_speed_for_target_2d(target, config),
            config.shark_speed,
        );
    }

    #[test]
    fn shark_confusion_slowdown_scales_with_crowding() {
        let config = test_config();
        let target = SharkTarget2d {
            position: Vec2::new(70.0, 10.0),
            crowding: 1,
        };

        assert_close(shark_speed_for_target_2d(target, config), 10.8);
    }

    #[test]
    fn shark_confusion_slowdown_reaches_configured_maximum() {
        let config = test_config();
        let target = SharkTarget2d {
            position: Vec2::new(70.0, 10.0),
            crowding: 4,
        };

        assert_close(shark_speed_for_target_2d(target, config), 7.2);
    }

    #[test]
    fn shark_confusion_slowdown_clamps_above_full_crowding() {
        let config = test_config();
        let target = SharkTarget2d {
            position: Vec2::new(70.0, 10.0),
            crowding: 12,
        };

        assert_close(shark_speed_for_target_2d(target, config), 7.2);
    }

    #[test]
    fn shark_confusion_speed_matches_in_2d_and_3d() {
        let config = test_config();
        let target_2d = SharkTarget2d {
            position: Vec2::new(70.0, 10.0),
            crowding: 2,
        };
        let target_3d = SharkTarget3d {
            position: Vec3::new(70.0, 10.0, 20.0),
            crowding: 2,
        };

        assert_close(
            shark_speed_for_target_2d(target_2d, config),
            shark_speed_for_target_3d(target_3d, config),
        );
    }

    #[test]
    fn tick_moves_crowded_fish_apart_in_2d() {
        let config = test_config();
        let mut simulation = Simulation {
            config,
            state: SimulationState::TwoD(World2d {
                fish: vec![fish_2d(50.0, 50.0), fish_2d(55.0, 50.0)],
                shark: shark_2d(180.0, 100.0),
                shark_target: None,
            }),
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };

        simulation.tick(0.1);

        let SimulationState::TwoD(world) = &simulation.state else {
            panic!("simulation should remain in 2d");
        };
        assert!(world.fish[0].position.x < 50.0);
        assert!(world.fish[1].position.x > 55.0);
    }

    #[test]
    fn shark_eats_fish_on_collision_in_2d() {
        let config = test_config();
        let mut simulation = Simulation {
            config,
            state: SimulationState::TwoD(World2d {
                fish: vec![fish_2d(11.0, 10.0), fish_2d(150.0, 100.0)],
                shark: shark_2d(10.0, 10.0),
                shark_target: None,
            }),
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };

        simulation.tick(0.1);

        assert_eq!(simulation.fish_count(), 1);
        assert_eq!(simulation.fish_eaten(), 1);
    }

    #[test]
    fn shark_searches_when_no_fish_are_in_scan_range_in_2d() {
        let mut config = test_config();
        config.shark_scan_radius = 10.0;
        let mut simulation = Simulation {
            config,
            state: SimulationState::TwoD(World2d {
                fish: vec![fish_2d(190.0, 100.0)],
                shark: shark_2d(10.0, 60.0),
                shark_target: None,
            }),
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };
        let start = simulation.shark_position();

        simulation.tick(0.5);

        let end = simulation.shark_position();
        assert_ne!(end, start);
        let SimulationState::TwoD(world) = &simulation.state else {
            panic!("simulation should remain in 2d");
        };
        assert!(world.shark_target.is_none());
    }

    #[test]
    fn shark_search_stays_clamped_near_edges_in_2d() {
        let mut config = test_config();
        config.shark_scan_radius = 1.0;
        config.shark_search_speed_multiplier = 1.0;
        let mut simulation = Simulation {
            config,
            state: SimulationState::TwoD(World2d {
                fish: vec![fish_2d(100.0, 60.0)],
                shark: Shark2d {
                    position: Vec2::new(199.0, 119.0),
                    velocity: Vec2::new(20.0, 20.0),
                },
                shark_target: None,
            }),
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };

        simulation.tick(1.0);

        let SimulationState::TwoD(world) = &simulation.state else {
            panic!("simulation should remain in 2d");
        };
        assert!(world.shark.velocity.x < 0.0);
        assert!(world.shark.velocity.y < 0.0);
        assert!(world.shark.position.x < 199.0);
        assert!(world.shark.position.y < 119.0);
        assert!(world.shark.position.x >= 0.0);
        assert!(world.shark.position.x <= config.world_width);
        assert!(world.shark.position.y >= 0.0);
        assert!(world.shark.position.y <= config.world_height);
    }

    #[test]
    fn shark_reacquires_fish_after_searching_into_scan_range() {
        let mut config = test_config();
        config.fish_max_speed = 0.0;
        config.fish_acceleration = 0.0;
        config.shark_scan_radius = 15.0;
        config.shark_search_speed_multiplier = 1.0;
        config.shark_search_wander_strength = 0.0;
        config.shark_search_turn_acceleration = 100.0;
        config.boundary_avoidance_strength = 0.0;
        let mut simulation = Simulation {
            config,
            state: SimulationState::TwoD(World2d {
                fish: vec![fish_2d(30.0, 60.0)],
                shark: Shark2d {
                    position: Vec2::new(0.0, 60.0),
                    velocity: Vec2::new(config.shark_speed, 0.0),
                },
                shark_target: None,
            }),
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };

        simulation.tick(1.0);
        simulation.tick(1.0);
        simulation.tick(1.0);

        let SimulationState::TwoD(world) = &simulation.state else {
            panic!("simulation should remain in 2d");
        };
        assert!(world.shark_target.is_some());
    }

    #[test]
    fn boundary_steering_points_inward_near_each_2d_edge() {
        let config = test_config();

        assert!(boundary_steering_2d(Vec2::new(5.0, 60.0), config).x > 0.0);
        assert!(boundary_steering_2d(Vec2::new(195.0, 60.0), config).x < 0.0);
        assert!(boundary_steering_2d(Vec2::new(100.0, 5.0), config).y > 0.0);
        assert!(boundary_steering_2d(Vec2::new(100.0, 115.0), config).y < 0.0);
        assert_eq!(
            boundary_steering_2d(Vec2::new(100.0, 60.0), config),
            Vec2::ZERO
        );
    }

    #[test]
    fn switching_to_3d_preserves_xy_and_generates_depth() {
        let mut simulation = test_simulation(test_config());
        if let SimulationState::TwoD(world) = &mut simulation.state {
            world.fish = vec![fish_2d(40.0, 50.0), fish_2d(70.0, 80.0)];
            world.fish[0].velocity = Vec2::new(4.0, 0.0);
            world.shark = shark_2d(10.0, 20.0);
        }
        simulation.elapsed_seconds = 12.0;
        simulation.fish_eaten = 3;

        simulation.set_dimensions(THREE_D);

        let SimulationState::ThreeD(world) = &simulation.state else {
            panic!("simulation should switch to 3d");
        };
        assert_eq!(simulation.dimensions(), THREE_D);
        assert_eq!(world.fish.len(), 2);
        assert_eq!(world.fish[0].position.x, 40.0);
        assert_eq!(world.fish[0].position.y, 50.0);
        assert!(world.fish[0].position.z >= 20.0);
        assert!(world.fish[0].position.z <= 80.0);
        assert_eq!(world.shark.position, Vec3::new(10.0, 20.0, 50.0));
        assert_eq!(simulation.elapsed_seconds(), 12.0);
        assert_eq!(simulation.fish_eaten(), 3);
    }

    #[test]
    fn generated_depth_is_deterministic() {
        let config = test_config();
        let center = Vec2::new(50.0, 60.0);
        let position = Vec2::new(75.0, 65.0);

        assert_eq!(
            generated_depth(4, position, center, config),
            generated_depth(4, position, center, config)
        );
    }

    #[test]
    fn fish_steers_away_from_nearby_fish_in_3d_depth() {
        let fish = vec![fish_3d(50.0, 50.0, 50.0), fish_3d(50.0, 50.0, 55.0)];

        let steering =
            fish_steering_3d(0, &fish, Vec3::new(180.0, 100.0, 90.0), test_config(), 0.0);

        assert!(steering.z < 0.0);
    }

    #[test]
    fn shark_targets_nearest_fish_by_3d_distance() {
        let fish = vec![fish_3d(20.0, 10.0, 90.0), fish_3d(45.0, 10.0, 10.0)];

        let target = best_shark_target_3d(Vec3::new(10.0, 10.0, 10.0), &fish, test_config());

        assert_eq!(
            target,
            Some(SharkTarget3d {
                position: Vec3::new(45.0, 10.0, 10.0),
                crowding: 0,
            })
        );
    }

    #[test]
    fn shark_eats_fish_on_collision_in_3d() {
        let config = test_config();
        let mut simulation = Simulation {
            config,
            state: SimulationState::ThreeD(World3d {
                fish: vec![fish_3d(10.0, 10.0, 14.0), fish_3d(150.0, 100.0, 90.0)],
                shark: shark_3d(10.0, 10.0, 10.0),
                shark_target: None,
            }),
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };

        simulation.tick(0.1);

        assert_eq!(simulation.fish_count(), 1);
        assert_eq!(simulation.fish_eaten(), 1);
    }

    #[test]
    fn shark_searches_when_no_fish_are_in_scan_range_in_3d() {
        let mut config = test_config();
        config.shark_scan_radius = 10.0;
        let mut simulation = Simulation {
            config,
            state: SimulationState::ThreeD(World3d {
                fish: vec![fish_3d(190.0, 100.0, 90.0)],
                shark: shark_3d(10.0, 60.0, 50.0),
                shark_target: None,
            }),
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };
        let start = simulation.shark_position();

        simulation.tick(0.5);

        let end = simulation.shark_position();
        assert_ne!(end, start);
        let SimulationState::ThreeD(world) = &simulation.state else {
            panic!("simulation should remain in 3d");
        };
        assert!(world.shark_target.is_none());
    }

    #[test]
    fn shark_search_turns_inward_near_3d_depth_edge() {
        let mut config = test_config();
        config.shark_scan_radius = 1.0;
        config.shark_search_speed_multiplier = 1.0;
        let mut simulation = Simulation {
            config,
            state: SimulationState::ThreeD(World3d {
                fish: vec![fish_3d(100.0, 60.0, 50.0)],
                shark: Shark3d {
                    position: Vec3::new(100.0, 60.0, 99.0),
                    velocity: Vec3::new(0.0, 0.0, 20.0),
                },
                shark_target: None,
            }),
            fish_eaten: 0,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };

        simulation.tick(1.0);

        let SimulationState::ThreeD(world) = &simulation.state else {
            panic!("simulation should remain in 3d");
        };
        assert!(world.shark.velocity.z < 0.0);
        assert!(world.shark.position.z < 99.0);
        assert!(world.shark.position.z >= 0.0);
        assert!(world.shark.position.z <= config.world_depth);
    }

    #[test]
    fn boundary_steering_points_inward_near_depth_edges() {
        let config = test_config();

        assert!(boundary_steering_3d(Vec3::new(100.0, 60.0, 5.0), config).z > 0.0);
        assert!(boundary_steering_3d(Vec3::new(100.0, 60.0, 95.0), config).z < 0.0);
    }

    #[test]
    fn switching_back_to_2d_projects_current_3d_xy_state() {
        let mut simulation = Simulation {
            config: test_config(),
            state: SimulationState::ThreeD(World3d {
                fish: vec![Fish3d {
                    position: Vec3::new(40.0, 50.0, 80.0),
                    velocity: Vec3::new(4.0, 5.0, 6.0),
                }],
                shark: Shark3d {
                    position: Vec3::new(10.0, 20.0, 30.0),
                    velocity: Vec3::new(1.0, 2.0, 3.0),
                },
                shark_target: Some(SharkTarget3d {
                    position: Vec3::new(40.0, 50.0, 80.0),
                    crowding: 0,
                }),
            }),
            fish_eaten: 2,
            fish_positions: Vec::new(),
            elapsed_seconds: 9.0,
        };

        simulation.set_dimensions(TWO_D);

        let SimulationState::TwoD(world) = &simulation.state else {
            panic!("simulation should switch to 2d");
        };
        assert_eq!(world.fish[0].position, Vec2::new(40.0, 50.0));
        assert_eq!(world.fish[0].velocity, Vec2::new(4.0, 5.0));
        assert_eq!(world.shark.position, Vec2::new(10.0, 20.0));
        assert_eq!(world.shark.velocity, Vec2::new(1.0, 2.0));
        assert_eq!(world.shark_target, None);
        assert_eq!(simulation.fish_positions(), vec![40.0, 50.0]);
        assert_eq!(simulation.fish_eaten(), 2);
        assert_eq!(simulation.elapsed_seconds(), 9.0);
    }

    #[test]
    fn fish_eaten_per_minute_uses_elapsed_simulation_time() {
        let mut simulation = test_simulation(test_config());

        assert_eq!(simulation.fish_eaten_per_minute(), 0.0);

        simulation.fish_eaten = 3;
        simulation.elapsed_seconds = 30.0;

        assert_eq!(simulation.fish_eaten_per_minute(), 6.0);
    }
}
