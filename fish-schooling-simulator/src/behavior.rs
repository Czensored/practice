use crate::{
    agent::{Fish, SharkTarget},
    config::SimulationConfig,
    vector::Vec2,
};

pub(crate) fn spawn_fish(config: SimulationConfig) -> Vec<Fish> {
    let cluster_center = Vec2::new(config.world_width * 0.55, config.world_height * 0.52);
    let cluster_radius = config.world_width.min(config.world_height) * 0.15;

    (0..config.fish_count)
        .map(|index| {
            let direction = index_direction(index);
            let distance =
                ((index + 1) as f32 / config.fish_count.max(1) as f32).sqrt() * cluster_radius;

            Fish {
                position: cluster_center + direction * distance,
                velocity: Vec2::new(1.0, 0.18).normalized_or(Vec2::new(1.0, 0.0))
                    * config.fish_max_speed
                    * 0.65,
            }
        })
        .collect()
}

pub(crate) fn fish_steering(
    fish_index: usize,
    fish: &[Fish],
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

pub(crate) fn best_shark_target(
    shark_position: Vec2,
    fish: &[Fish],
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

pub(crate) fn shark_speed_for_target(target: SharkTarget, config: SimulationConfig) -> f32 {
    if target.crowding > 0 {
        config.shark_speed * config.shark_confused_speed_multiplier
    } else {
        config.shark_speed
    }
}

fn nearby_fish_count(
    target_index: usize,
    target_position: Vec2,
    fish: &[Fish],
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

pub(crate) fn move_toward(current: Vec2, target: Vec2, max_delta: f32) -> Vec2 {
    let delta = target - current;
    let distance = delta.length();
    if distance <= max_delta || distance <= f32::EPSILON {
        target
    } else {
        current + delta / distance * max_delta
    }
}

pub(crate) fn boundary_steering(position: Vec2, config: SimulationConfig) -> Vec2 {
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

pub(crate) fn clamp_position_within_bounds(position: &mut Vec2, width: f32, height: f32) {
    position.x = position.x.clamp(0.0, width);
    position.y = position.y.clamp(0.0, height);
}

pub(crate) fn index_direction(index: usize) -> Vec2 {
    let angle = index as f32 * 2.399_963_1;
    Vec2::new(angle.cos(), angle.sin())
}
