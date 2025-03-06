use bevy::prelude::*;

/// Controller component for character movement
#[derive(Component)]
pub struct CharacterController {
    pub velocity: Vec3,
    pub rotational_velocity: f32,
}

/// Information about a leg
#[derive(Clone, Debug)]
pub struct LegInfo {
    pub hip: Entity,
    pub knee: Option<Entity>,
    pub ankle: Entity,
    pub toe: Option<Entity>,
    pub foot_width: f32,
    pub foot_length: f32,
    pub foot_offset: Vec2,
    // Cached values that will be filled during initialization
    pub ankle_heel_vector: Vec3,
    pub toe_toetip_vector: Vec3,
    pub leg_chain: Vec<Entity>,
    pub leg_length: f32,
}

/// State of a leg during animation
#[derive(Clone, Debug)]
pub struct LegState {
    // Footprint information
    pub step_from_position: Vec3,
    pub step_to_position: Vec3,
    pub step_to_position_goal: Vec3,
    pub step_from_time: f32,
    pub step_to_time: f32,
    
    // Cycle timing information
    pub cycle_time: f32,
    pub cycle_time_prev: f32,
    
    // Foot position references
    pub hip_reference: Vec3,
    pub ankle_reference: Vec3,
    pub foot_base: Vec3,
    pub foot_base_rotation: Quat,
    
    // Foot cycle event times (as normalized times between 0 and 1)
    pub stance_time: f32,
    pub lift_time: f32,
    pub liftoff_time: f32,
    pub post_lift_time: f32,
    pub pre_land_time: f32,
    pub strike_time: f32,
    pub land_time: f32,
    
    // Current phase
    pub phase: LegCyclePhase,
    
    // Standing logic
    pub parked: bool,
    
    // Cycle properties
    pub stance_position: Vec3,
    pub heel_toetip_vector: Vec3,
}

/// Phases of the leg cycle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegCyclePhase {
    Stance,
    Lift,
    Flight,
    Land,
}

/// Component to mark entities with locomotion capabilities
#[derive(Component)]
pub struct LocomotionCharacter {
    pub legs: Vec<LegInfo>,
    pub leg_states: Vec<LegState>,
    pub cycle_duration: f32,
    pub cycle_distance: f32,
    pub cycle_direction: Vec3,
    pub ground_plane_height: f32,
}

/// Data extracted from analyzing an animation
#[derive(Clone, Debug)]
pub struct LegCycleData {
    pub cycle_center: Vec3,
    pub cycle_scaling: f32,
    pub cycle_direction: Vec3,
    pub stance_time: f32,
    pub lift_time: f32,
    pub liftoff_time: f32,
    pub post_lift_time: f32,
    pub pre_land_time: f32,
    pub strike_time: f32,
    pub land_time: f32,
    pub cycle_distance: f32,
    pub stance_position: Vec3,
    pub heel_toetip_vector: Vec3,
}
