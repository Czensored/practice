use crate::{
    agent::{Fish2d, Fish3d, SharkTarget2d, SharkTarget3d},
    config::SimulationConfig,
    vector::{Vec2, Vec3},
};

pub(crate) fn spawn_fish_2d(config: SimulationConfig) -> Vec<Fish2d> {
    let cluster_center = Vec2::new(config.world_width * 0.55, config.world_height * 0.52);
    let cluster_radius = config.world_width.min(config.world_height) * 0.15;

    (0..config.fish_count)
        .map(|index| {
            let direction = index_direction_2d(index);
            let distance =
                ((index + 1) as f32 / config.fish_count.max(1) as f32).sqrt() * cluster_radius;

            Fish2d {
                position: cluster_center + direction * distance,
                velocity: Vec2::new(1.0, 0.18).normalized_or(Vec2::new(1.0, 0.0))
                    * config.fish_max_speed
                    * 0.65,
            }
        })
        .collect()
}

pub(crate) fn fish_steering_2d(
    fish_index: usize,
    fish: &[Fish2d],
    shark_position: Vec2,
    config: SimulationConfig,
    elapsed_seconds: f32,
) -> Vec2 {
    let current = fish[fish_index];
    let mut steering =
        wander_direction_2d(fish_index, elapsed_seconds) * config.fish_wander_strength;
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
            steering += offset.normalized_or(index_direction_2d(fish_index + other_index + 1))
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
        steering += shark_offset.normalized_or(index_direction_2d(fish_index + 101))
            * strength
            * config.fish_flee_strength;
    }

    steering += boundary_steering_2d(current.position, config);

    steering
}

pub(crate) fn best_shark_target_2d(
    shark_position: Vec2,
    fish: &[Fish2d],
    config: SimulationConfig,
) -> Option<SharkTarget2d> {
    fish.iter()
        .enumerate()
        .filter_map(|(index, candidate)| {
            let distance = (candidate.position - shark_position).length();
            if distance > config.shark_scan_radius {
                return None;
            }

            let crowding = nearby_fish_count_2d(
                index,
                candidate.position,
                fish,
                config.shark_confusion_radius,
            );
            let score = distance + crowding as f32 * config.shark_crowding_penalty;

            Some((
                score,
                SharkTarget2d {
                    position: candidate.position,
                    crowding,
                },
            ))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_score, target)| target)
}

pub(crate) fn shark_speed_for_target_2d(target: SharkTarget2d, config: SimulationConfig) -> f32 {
    config.shark_speed * shark_confusion_speed_multiplier(target.crowding, config)
}

pub(crate) fn shark_search_steering_2d(
    position: Vec2,
    velocity: Vec2,
    elapsed_seconds: f32,
    config: SimulationConfig,
) -> Vec2 {
    let boundary = boundary_steering_2d(position, config);
    if boundary.length_squared() > f32::EPSILON {
        return boundary;
    }

    let heading = velocity.normalized_or(Vec2::new(1.0, 0.0));
    let random = random_walk_direction_2d(elapsed_seconds, config);

    heading + random * config.shark_search_wander_strength
}

fn nearby_fish_count_2d(
    target_index: usize,
    target_position: Vec2,
    fish: &[Fish2d],
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

fn wander_direction_2d(fish_index: usize, elapsed_seconds: f32) -> Vec2 {
    let angle = elapsed_seconds * 0.75 + fish_index as f32 * 0.19;
    Vec2::new(angle.cos(), angle.sin())
}

pub(crate) fn move_toward_2d(current: Vec2, target: Vec2, max_delta: f32) -> Vec2 {
    let delta = target - current;
    let distance = delta.length();
    if distance <= max_delta || distance <= f32::EPSILON {
        target
    } else {
        current + delta / distance * max_delta
    }
}

pub(crate) fn boundary_steering_2d(position: Vec2, config: SimulationConfig) -> Vec2 {
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

pub(crate) fn clamp_position_within_bounds_2d(position: &mut Vec2, width: f32, height: f32) {
    position.x = position.x.clamp(0.0, width);
    position.y = position.y.clamp(0.0, height);
}

pub(crate) fn index_direction_2d(index: usize) -> Vec2 {
    let angle = index as f32 * 2.399_963_1;
    Vec2::new(angle.cos(), angle.sin())
}

pub(crate) fn spawn_fish_3d(config: SimulationConfig) -> Vec<Fish3d> {
    let cluster_center = Vec3::new(
        config.world_width * 0.55,
        config.world_height * 0.52,
        config.world_depth * 0.5,
    );
    let cluster_radius = config
        .world_width
        .min(config.world_height)
        .min(config.world_depth)
        * 0.15;

    (0..config.fish_count)
        .map(|index| {
            let direction = index_direction_3d(index);
            let distance =
                ((index + 1) as f32 / config.fish_count.max(1) as f32).sqrt() * cluster_radius;

            Fish3d {
                position: cluster_center + direction * distance,
                velocity: Vec3::new(1.0, 0.18, 0.22).normalized_or(Vec3::new(1.0, 0.0, 0.0))
                    * config.fish_max_speed
                    * 0.65,
            }
        })
        .collect()
}

pub(crate) fn fish_steering_3d(
    fish_index: usize,
    fish: &[Fish3d],
    shark_position: Vec3,
    config: SimulationConfig,
    elapsed_seconds: f32,
) -> Vec3 {
    let current = fish[fish_index];
    let mut steering =
        wander_direction_3d(fish_index, elapsed_seconds) * config.fish_wander_strength;
    let mut neighbor_count = 0;
    let mut neighbor_velocity = Vec3::ZERO;
    let mut neighbor_center = Vec3::ZERO;

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
            steering += offset.normalized_or(index_direction_3d(fish_index + other_index + 1))
                * strength
                * config.fish_separation_strength;
        }
    }

    if neighbor_count > 0 {
        let count = neighbor_count as f32;
        let average_velocity = neighbor_velocity / count;
        let center = neighbor_center / count;
        steering += average_velocity.normalized_or(current.velocity.normalized_or(Vec3::ZERO))
            * config.fish_alignment_strength;
        steering +=
            (center - current.position).normalized_or(Vec3::ZERO) * config.fish_cohesion_strength;
    }

    let shark_offset = current.position - shark_position;
    let shark_distance = shark_offset.length();
    if shark_distance < config.fish_flee_radius {
        let strength = 1.0 - shark_distance / config.fish_flee_radius;
        steering += shark_offset.normalized_or(index_direction_3d(fish_index + 101))
            * strength
            * config.fish_flee_strength;
    }

    steering += boundary_steering_3d(current.position, config);

    steering
}

pub(crate) fn best_shark_target_3d(
    shark_position: Vec3,
    fish: &[Fish3d],
    config: SimulationConfig,
) -> Option<SharkTarget3d> {
    fish.iter()
        .enumerate()
        .filter_map(|(index, candidate)| {
            let distance = (candidate.position - shark_position).length();
            if distance > config.shark_scan_radius {
                return None;
            }

            let crowding = nearby_fish_count_3d(
                index,
                candidate.position,
                fish,
                config.shark_confusion_radius,
            );
            let score = distance + crowding as f32 * config.shark_crowding_penalty;

            Some((
                score,
                SharkTarget3d {
                    position: candidate.position,
                    crowding,
                },
            ))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_score, target)| target)
}

pub(crate) fn shark_speed_for_target_3d(target: SharkTarget3d, config: SimulationConfig) -> f32 {
    config.shark_speed * shark_confusion_speed_multiplier(target.crowding, config)
}

pub(crate) fn shark_search_steering_3d(
    position: Vec3,
    velocity: Vec3,
    elapsed_seconds: f32,
    config: SimulationConfig,
) -> Vec3 {
    let boundary = boundary_steering_3d(position, config);
    if boundary.length_squared() > f32::EPSILON {
        return boundary;
    }

    let heading = velocity.normalized_or(Vec3::new(1.0, 0.0, 0.0));
    let random = random_walk_direction_3d(elapsed_seconds, config);

    heading + random * config.shark_search_wander_strength
}

fn nearby_fish_count_3d(
    target_index: usize,
    target_position: Vec3,
    fish: &[Fish3d],
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

fn shark_confusion_speed_multiplier(crowding: usize, config: SimulationConfig) -> f32 {
    let full_crowding = config.shark_confusion_full_crowding.max(1.0);
    let confusion = (crowding as f32 / full_crowding).min(1.0);
    let max_slowdown = config.shark_max_confusion_slowdown.clamp(0.0, 1.0);

    1.0 - confusion * max_slowdown
}

fn wander_direction_3d(fish_index: usize, elapsed_seconds: f32) -> Vec3 {
    let angle = elapsed_seconds * 0.75 + fish_index as f32 * 0.19;
    let depth_angle = elapsed_seconds * 0.43 + fish_index as f32 * 0.31;
    Vec3::new(angle.cos(), angle.sin(), depth_angle.sin() * 0.6)
        .normalized_or(index_direction_3d(fish_index))
}

fn random_walk_direction_2d(elapsed_seconds: f32, config: SimulationConfig) -> Vec2 {
    let step = random_walk_step(elapsed_seconds, config);
    let angle = search_noise(step * 12.989_8 + 4.137) * std::f32::consts::TAU;
    Vec2::new(angle.cos(), angle.sin())
}

fn random_walk_direction_3d(elapsed_seconds: f32, config: SimulationConfig) -> Vec3 {
    let step = random_walk_step(elapsed_seconds, config);
    let angle = search_noise(step * 12.989_8 + 4.137) * std::f32::consts::TAU;
    let vertical = search_noise(step * 78.233 + 9.611) * 1.4 - 0.7;
    let horizontal = (1.0 - vertical * vertical).sqrt();

    Vec3::new(angle.cos() * horizontal, angle.sin() * horizontal, vertical)
        .normalized_or(Vec3::new(1.0, 0.0, 0.0))
}

fn random_walk_step(elapsed_seconds: f32, config: SimulationConfig) -> f32 {
    (elapsed_seconds / config.shark_search_turn_seconds.max(0.1)).floor()
}

fn search_noise(seed: f32) -> f32 {
    ((seed.sin() * 12.989_8 + seed.cos() * 78.233).sin() * 0.5 + 0.5).clamp(0.0, 1.0)
}

pub(crate) fn move_toward_3d(current: Vec3, target: Vec3, max_delta: f32) -> Vec3 {
    let delta = target - current;
    let distance = delta.length();
    if distance <= max_delta || distance <= f32::EPSILON {
        target
    } else {
        current + delta / distance * max_delta
    }
}

pub(crate) fn boundary_steering_3d(position: Vec3, config: SimulationConfig) -> Vec3 {
    let margin = config.boundary_margin.max(f32::EPSILON);
    let strength = config.boundary_avoidance_strength;
    let mut steering = Vec3::ZERO;

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

    if position.z < margin {
        steering.z += (1.0 - position.z / margin) * strength;
    } else if position.z > config.world_depth - margin {
        steering.z -= (1.0 - (config.world_depth - position.z) / margin) * strength;
    }

    steering
}

pub(crate) fn clamp_position_within_bounds_3d(
    position: &mut Vec3,
    width: f32,
    height: f32,
    depth: f32,
) {
    position.x = position.x.clamp(0.0, width);
    position.y = position.y.clamp(0.0, height);
    position.z = position.z.clamp(0.0, depth);
}

pub(crate) fn index_direction_3d(index: usize) -> Vec3 {
    let angle = index as f32 * 2.399_963_1;
    let vertical = (index as f32 * 1.618_033_9).sin() * 0.72;
    let horizontal = (1.0 - vertical * vertical).sqrt();
    Vec3::new(angle.cos() * horizontal, angle.sin() * horizontal, vertical)
}
