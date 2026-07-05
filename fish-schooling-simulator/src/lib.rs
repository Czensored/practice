use wasm_bindgen::prelude::*;

mod config {
    #[derive(Clone, Copy, Debug)]
    pub struct SimulationConfig {
        pub world_width: f32,
        pub world_height: f32,
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
        pub shark_confused_speed_multiplier: f32,
        pub boundary_margin: f32,
        pub boundary_avoidance_strength: f32,
        pub max_delta_seconds: f32,
    }

    impl Default for SimulationConfig {
        fn default() -> Self {
            Self {
                world_width: 960.0,
                world_height: 640.0,
                fish_count: 90,
                fish_max_speed: 96.0,
                fish_acceleration: 540.0,
                fish_school_radius: 92.0,
                fish_personal_space: 34.0,
                fish_separation_strength: 2.4,
                fish_alignment_strength: 0.8,
                fish_cohesion_strength: 0.65,
                fish_wander_strength: 0.45,
                fish_flee_radius: 185.0,
                fish_flee_strength: 4.0,
                shark_scan_radius: 360.0,
                shark_speed: 126.0,
                shark_eat_radius: 13.0,
                shark_confusion_radius: 48.0,
                shark_crowding_penalty: 65.0,
                shark_confused_speed_multiplier: 0.55,
                boundary_margin: 90.0,
                boundary_avoidance_strength: 3.0,
                max_delta_seconds: 1.0 / 30.0,
            }
        }
    }

    pub fn default_config() -> SimulationConfig {
        SimulationConfig::default()
    }
}

use config::{SimulationConfig, default_config};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    const ZERO: Self = Self { x: 0.0, y: 0.0 };

    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    fn normalized_or(self, fallback: Self) -> Self {
        let length = self.length();
        if length > f32::EPSILON {
            self / length
        } else {
            fallback
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Clone, Copy, Debug)]
struct Agent {
    position: Vec2,
    velocity: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SharkTarget {
    position: Vec2,
    crowding: usize,
}

#[wasm_bindgen]
pub struct Simulation {
    config: SimulationConfig,
    fish: Vec<Agent>,
    shark: Agent,
    shark_target: Option<SharkTarget>,
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
        self.eat_colliding_fish();
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
            shark: Agent {
                position: Vec2::new(config.world_width * 0.15, config.world_height * 0.5),
                velocity: Vec2::new(1.0, 0.35).normalized_or(Vec2::new(1.0, 0.0))
                    * config.shark_speed,
            },
            shark_target: None,
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
            clamp_within_bounds(fish, self.config.world_width, self.config.world_height);
        }
    }

    fn update_shark(&mut self, dt: f32) {
        self.shark_target = best_shark_target(self.shark.position, &self.fish, self.config);

        if let Some(target) = self.shark_target {
            let speed = shark_speed_for_target(target, self.config);
            self.shark.velocity =
                (target.position - self.shark.position).normalized_or(Vec2::new(1.0, 0.0)) * speed;
            self.shark.position += self.shark.velocity * dt;
            clamp_within_bounds(
                &mut self.shark,
                self.config.world_width,
                self.config.world_height,
            );

            if self.eat_colliding_fish() > 0 {
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

fn spawn_fish(config: SimulationConfig) -> Vec<Agent> {
    let cluster_center = Vec2::new(config.world_width * 0.55, config.world_height * 0.52);
    let cluster_radius = config.world_width.min(config.world_height) * 0.15;

    (0..config.fish_count)
        .map(|index| {
            let direction = index_direction(index);
            let distance =
                ((index + 1) as f32 / config.fish_count.max(1) as f32).sqrt() * cluster_radius;

            Agent {
                position: cluster_center + direction * distance,
                velocity: Vec2::new(1.0, 0.18).normalized_or(Vec2::new(1.0, 0.0))
                    * config.fish_max_speed
                    * 0.65,
            }
        })
        .collect()
}

fn fish_steering(
    fish_index: usize,
    fish: &[Agent],
    shark_position: Vec2,
    config: SimulationConfig,
    elapsed_seconds: f32,
) -> Vec2 {
    let current = fish[fish_index];
    let mut steering = wander_direction(fish_index, elapsed_seconds) * config.fish_wander_strength;
    let mut neighbor_count = 0;
    let mut neighbor_velocity = Vec2::ZERO;
    let mut neighbor_center = Vec2::ZERO;

    for (other_index, other) in fish.iter().enumerate() {
        if fish_index == other_index {
            continue;
        }

        let offset = current.position - other.position;
        let distance = offset.length();

        if distance < config.fish_school_radius {
            neighbor_count += 1;
            neighbor_velocity += other.velocity;
            neighbor_center += other.position;
        }

        if distance < config.fish_personal_space {
            let strength = 1.0 - distance / config.fish_personal_space;
            steering += offset.normalized_or(index_direction(fish_index + other_index + 1))
                * strength
                * config.fish_separation_strength;
        }
    }

    if neighbor_count > 0 {
        let count = neighbor_count as f32;
        let average_velocity = neighbor_velocity / count;
        let center = neighbor_center / count;
        steering += average_velocity.normalized_or(current.velocity.normalized_or(Vec2::ZERO))
            * config.fish_alignment_strength;
        steering +=
            (center - current.position).normalized_or(Vec2::ZERO) * config.fish_cohesion_strength;
    }

    let shark_offset = current.position - shark_position;
    let shark_distance = shark_offset.length();
    if shark_distance < config.fish_flee_radius {
        let strength = 1.0 - shark_distance / config.fish_flee_radius;
        steering += shark_offset.normalized_or(index_direction(fish_index + 101))
            * strength
            * config.fish_flee_strength;
    }

    steering += boundary_steering(current.position, config);

    steering
}

fn best_shark_target(
    shark_position: Vec2,
    fish: &[Agent],
    config: SimulationConfig,
) -> Option<SharkTarget> {
    fish.iter()
        .enumerate()
        .filter_map(|(index, candidate)| {
            let distance = (candidate.position - shark_position).length();
            if distance > config.shark_scan_radius {
                return None;
            }

            let crowding = nearby_fish_count(
                index,
                candidate.position,
                fish,
                config.shark_confusion_radius,
            );
            let score = distance + crowding as f32 * config.shark_crowding_penalty;

            Some((
                score,
                SharkTarget {
                    position: candidate.position,
                    crowding,
                },
            ))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_score, target)| target)
}

fn shark_speed_for_target(target: SharkTarget, config: SimulationConfig) -> f32 {
    if target.crowding > 0 {
        config.shark_speed * config.shark_confused_speed_multiplier
    } else {
        config.shark_speed
    }
}

fn nearby_fish_count(
    target_index: usize,
    target_position: Vec2,
    fish: &[Agent],
    radius: f32,
) -> usize {
    let radius_squared = radius * radius;
    fish.iter()
        .enumerate()
        .filter(|(index, other)| {
            *index != target_index
                && (other.position - target_position).length_squared() <= radius_squared
        })
        .count()
}

fn wander_direction(fish_index: usize, elapsed_seconds: f32) -> Vec2 {
    let angle = elapsed_seconds * 0.75 + fish_index as f32 * 0.19;
    Vec2::new(angle.cos(), angle.sin())
}

fn move_toward(current: Vec2, target: Vec2, max_delta: f32) -> Vec2 {
    let delta = target - current;
    let distance = delta.length();
    if distance <= max_delta || distance <= f32::EPSILON {
        target
    } else {
        current + delta / distance * max_delta
    }
}

fn boundary_steering(position: Vec2, config: SimulationConfig) -> Vec2 {
    let margin = config.boundary_margin.max(f32::EPSILON);
    let strength = config.boundary_avoidance_strength;
    let mut steering = Vec2::ZERO;

    if position.x < margin {
        steering.x += (1.0 - position.x / margin) * strength;
    } else if position.x > config.world_width - margin {
        steering.x -= (1.0 - (config.world_width - position.x) / margin) * strength;
    }

    if position.y < margin {
        steering.y += (1.0 - position.y / margin) * strength;
    } else if position.y > config.world_height - margin {
        steering.y -= (1.0 - (config.world_height - position.y) / margin) * strength;
    }

    steering
}

fn clamp_within_bounds(agent: &mut Agent, width: f32, height: f32) {
    agent.position.x = agent.position.x.clamp(0.0, width);
    agent.position.y = agent.position.y.clamp(0.0, height);
}

fn index_direction(index: usize) -> Vec2 {
    let angle = index as f32 * 2.399_963_1;
    Vec2::new(angle.cos(), angle.sin())
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn agent(x: f32, y: f32) -> Agent {
        Agent {
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
        }
    }

    #[test]
    fn fish_steers_away_from_nearby_fish() {
        let fish = vec![
            Agent {
                position: Vec2::new(50.0, 50.0),
                velocity: Vec2::new(1.0, 0.0),
            },
            Agent {
                position: Vec2::new(55.0, 50.0),
                velocity: Vec2::new(-1.0, 0.0),
            },
        ];

        let steering = fish_steering(0, &fish, Vec2::new(180.0, 100.0), test_config(), 0.0);

        assert!(steering.x < 0.0);
    }

    #[test]
    fn fish_steers_away_from_shark_inside_flee_radius() {
        let fish = vec![Agent {
            position: Vec2::new(50.0, 50.0),
            velocity: Vec2::new(1.0, 0.0),
        }];

        let steering = fish_steering(0, &fish, Vec2::new(60.0, 50.0), test_config(), 0.0);

        assert!(steering.x < 0.0);
    }

    #[test]
    fn exposed_fish_can_be_selected_over_closer_dense_target() {
        let fish = vec![
            agent(30.0, 10.0),
            agent(33.0, 10.0),
            agent(30.0, 13.0),
            agent(33.0, 13.0),
            agent(70.0, 10.0),
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
            fish: vec![agent(50.0, 50.0), agent(55.0, 50.0)],
            shark: agent(180.0, 100.0),
            shark_target: None,
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
            fish: vec![agent(30.0, 10.0), agent(180.0, 100.0)],
            shark: agent(10.0, 10.0),
            shark_target: None,
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
            fish: vec![agent(0.0, 60.0)],
            shark: agent(10.0, 60.0),
            shark_target: None,
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
                Agent {
                    position: Vec2::new(50.0, 50.0),
                    velocity: Vec2::new(6.0, 0.0),
                },
                Agent {
                    position: Vec2::new(80.0, 50.0),
                    velocity: Vec2::new(6.0, 0.0),
                },
            ],
            shark: agent(180.0, 100.0),
            shark_target: None,
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
            fish: vec![agent(11.0, 10.0), agent(150.0, 100.0)],
            shark: agent(10.0, 10.0),
            shark_target: None,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };

        simulation.tick(0.1);

        assert_eq!(simulation.fish.len(), 1);
        assert!(simulation.fish[0].position.x > 100.0);
        assert!(simulation.fish[0].position.y > 80.0);
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
    fn agents_are_clamped_without_flipping_velocity() {
        let mut agent = Agent {
            position: Vec2::new(-2.0, 130.0),
            velocity: Vec2::new(-4.0, 5.0),
        };

        clamp_within_bounds(&mut agent, 200.0, 120.0);

        assert_eq!(agent.position, Vec2::new(0.0, 120.0));
        assert_eq!(agent.velocity, Vec2::new(-4.0, 5.0));
    }

    #[test]
    fn fish_near_edge_moves_back_toward_center() {
        let config = test_config();
        let mut simulation = Simulation {
            config,
            fish: vec![agent(5.0, 60.0)],
            shark: agent(180.0, 100.0),
            shark_target: None,
            fish_positions: Vec::new(),
            elapsed_seconds: 0.0,
        };

        simulation.tick(0.1);

        assert!(simulation.fish[0].position.x > 5.0);
    }
}
