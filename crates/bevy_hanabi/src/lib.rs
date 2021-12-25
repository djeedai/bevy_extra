#![deny(
    //warnings,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    //unused_import_braces,
    unused_qualifications,
    //missing_docs
)]

//! Hanabi -- a particle system plugin for the Bevy game engine.
//!
//! This library provides a particle system for the Bevy game engine.
//!
//! # Example
//!
//! Add the Hanabi plugin to your app:
//!
//! ```rust
//! # use bevy::prelude::*;
//! # use bevy_hanabi::*;
//! AppBuilder::default()
//!     .add_default_plugins()
//!     .add_plugin(HanabiPlugin)
//!     .run();
//! ```
//!
//! Animate the position ([`Transform::translation`]) of an [`Entity`]:
//!
//! ```rust
//! # use bevy_tweening::*;
//! # use std::time::Duration;
//! commands
//!     // Spawn a Sprite entity to animate the position of
//!     .spawn_bundle(SpriteBundle {
//!         material: materials.add(Color::RED.into()),
//!         sprite: Sprite {
//!             size: Vec2::new(size, size),
//!             ..Default::default()
//!         },
//!         ..Default::default()
//!     })
//!     // Add an Animator component to perform the animation
//!     .insert(Animator::new(
//!         // Use a quadratic easing on both endpoints
//!         EaseFunction::QuadraticInOut,
//!         // Loop animation back and forth over 1 second, with a 0.5 second
//!         // pause after each cycle (start -> end -> start).
//!         TweeningType::PingPong {
//!             duration: Duration::from_secs(1),
//!             pause: Some(Duration::from_millis(500)),
//!         },
//!         // The lens gives access to the Transform component of the Sprite,
//!         // for the Animator to animate it. It also contains the start and
//!         // end values associated with the animation ratios 0. and 1.
//!         TransformPositionLens {
//!             start: Vec3::new(0., 0., 0.),
//!             end: Vec3::new(1., 2., -4.),
//!         },
//!     ));
//! ```
//!
//! # Animators and lenses
//!
//! Bevy components and assets are animated with tweening animator components. Those animators determine
//! the fields to animate using lenses.
//!
//! ## Components animation
//!
//! Components are animated with the [`Animator`] component, which is generic over the type of component
//! it animates. This is a restriction imposed by Bevy, to access the animated component as a mutable
//! reference via a [`Query`] and comply with the ECS rules.
//!
//! The [`Animator`] itself is not generic over the subset of fields of the components it animates.
//! This limits the proliferation of generic types when animating e.g. both the position and rotation
//! of an entity.
//!
//! ## Assets animation
//!
//! Assets are animated in a similar way to component, via the [`AssetAnimator`] component. Because assets
//! are typically shared, and the animation applies to the asset itself, all users of the asset see the
//! animation. For example, animating the color of a [`ColorMaterial`] will change the color of all [`Sprite`]
//! components using that material.
//!
//! ## Lenses
//!
//! Both [`Animator`] and [`AssetAnimator`] access the field(s) to animate via a lens, a type that implements
//! the [`Lens`] trait. Several predefined lenses are provided for the most commonly animated fields, like the
//! components of a [`Transform`]. A custom lens can also be created by implementing the trait, allowing to
//! animate virtually any field of any Bevy component or asset.
//!
//! [`Transform::translation`]: bevy::transform::components::Transform::translation
//! [`Entity`]: bevy::ecs::entity::Entity
//! [`Query`]: bevy::ecs::system::Query
//! [`ColorMaterial`]: bevy::sprite::ColorMaterial
//! [`Sprite`]: bevy::sprite::Sprite
//! [`Transform`]: bevy::transform::components::Transform

use bevy::{prelude::*, reflect::TypeUuid};

mod plugin;
mod render;

pub use plugin::HanabiPlugin;

#[derive(Debug, Clone, Copy, Component, TypeUuid)]
#[uuid = "c48df8b5-7eca-4d25-831e-513c2575cf6c"]
pub struct ParticlesEffect {
    spawner: Spawner,
    updater: Updater,
}

impl ParticlesEffect {
    pub fn new_bundle(
        capacity: usize,
        spawner: Spawner,
        updater: Updater,
    ) -> (ParticlesEffect, SpawnState, UpdateState) {
        (
            ParticlesEffect { spawner, updater },
            SpawnState::default(),
            UpdateState::new(capacity),
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Spawner {
    /// Number of particles to spawn per second.
    rate: f32,
    origin: Vec3,
    velocity: Vec3,
}

impl Spawner {
    pub fn new(rate: f32, origin: Vec3, velocity: Vec3) -> Self {
        Spawner {
            rate,
            origin,
            velocity,
        }
    }

    pub fn spawn(&mut self, spawn_state: &mut SpawnState, state: &mut UpdateState, dt: f32) {
        // Tick
        spawn_state.count += self.rate * dt;

        // Allocate
        let count = (spawn_state.count as usize).min(state.buffer.len() - state.used);
        spawn_state.count = spawn_state.count.fract();

        // Initialize
        if count > 0 {
            let particles = &mut state.buffer[state.used..state.used + count];
            let acc = Vec3::new(0., -9.81, 0.);
            for p in particles {
                p.position = self.origin;
                p.init_velocity(self.velocity, dt);
                p.acceleration = acc;
            }
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Updater {}

impl Updater {
    pub fn update(&mut self, state: &mut UpdateState, dt: f32) {
        // Verlet integration
        let particles = &mut state.buffer[..state.used];
        for p in particles {
            let prev = p.position;
            p.position = p.position * 2.0 - p.prev_position + p.acceleration * dt * dt;
            p.prev_position = prev;
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Component)]
pub struct SpawnState {
    // Fractional remainder of the number of particles to spawn.
    count: f32,
}

#[derive(Debug, Default, Copy, Clone, Component)]
pub struct MotionState {
    position: Vec3,
    prev_position: Vec3,
    acceleration: Vec3,
}

#[derive(Debug, Clone, Component)]
pub struct UpdateState {
    buffer: Vec<MotionState>,
    used: usize,
}

impl UpdateState {
    pub fn new(capacity: usize) -> Self {
        let mut state = UpdateState {
            buffer: Vec::with_capacity(capacity),
            used: 0,
        };
        state.buffer.resize_with(capacity, Default::default);
        state
    }
}

impl MotionState {
    pub fn init_velocity(&mut self, velocity: Vec3, dt: f32) {
        self.prev_position = self.position - velocity * dt;
    }
}

/// Playback state of an animator.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum AnimatorState {
    /// The animation is playing.
    Playing,
    /// The animation is paused/stopped.
    Paused,
}

impl std::ops::Not for AnimatorState {
    type Output = AnimatorState;

    fn not(self) -> Self::Output {
        match self {
            AnimatorState::Paused => AnimatorState::Playing,
            AnimatorState::Playing => AnimatorState::Paused,
        }
    }
}
