use bevy::{
    core_pipeline::Transparent2d,
    prelude::*,
    render::{
        render_phase::DrawFunctions, render_resource::SpecializedPipelines, RenderApp, RenderStage,
    },
};

use crate::{
    render::{
        extract_particles, extract_particles_events, prepare_particles, queue_particles,
        DrawParticles, ExtractedParticles, ImageBindGroups, ParticlesAssetEvents, ParticlesMeta,
        ParticlesPipeline, PARTICLES_SHADER_HANDLE,
    },
    ParticlesEffect, SpawnState, UpdateState,
};

/// Plugin to add systems related to Hanabi.
#[derive(Debug, Clone, Copy)]
pub struct HanabiPlugin;

impl Plugin for HanabiPlugin {
    fn build(&self, app: &mut App) {
        // Register the spawn and update systems
        app.add_system(hanabi_spawn.system())
            .add_system(hanabi_update.system());

        // Register the particles shader
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        let sprite_shader = Shader::from_wgsl(include_str!("render/particles.wgsl"));
        shaders.set_untracked(PARTICLES_SHADER_HANDLE, sprite_shader);

        // Register the component reflection
        //app.register_type::<ParticlesEffect>();

        // Register the custom render pipeline
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<ImageBindGroups>()
            .init_resource::<ParticlesPipeline>()
            .init_resource::<SpecializedPipelines<ParticlesPipeline>>()
            .init_resource::<ParticlesMeta>()
            .init_resource::<ExtractedParticles>()
            .init_resource::<ParticlesAssetEvents>()
            .add_system_to_stage(
                RenderStage::Extract,
                extract_particles, //.label(ParticlesSystem::ExtractParticles),
            )
            .add_system_to_stage(RenderStage::Extract, extract_particles_events)
            .add_system_to_stage(RenderStage::Prepare, prepare_particles)
            .add_system_to_stage(RenderStage::Queue, queue_particles);

        let draw_particles = DrawParticles::new(&mut render_app.world);
        render_app
            .world
            .get_resource::<DrawFunctions<Transparent2d>>()
            .unwrap()
            .write()
            .add(draw_particles);
    }
}

pub fn hanabi_spawn(
    time: Res<Time>,
    mut query: Query<(&mut ParticlesEffect, &mut SpawnState, &mut UpdateState)>,
) {
    for (ref mut effect, ref mut spawn_state, ref mut state) in query.iter_mut() {
        effect
            .spawner
            .spawn(spawn_state, state, time.delta_seconds());
    }
}

pub fn hanabi_update(time: Res<Time>, mut query: Query<(&mut ParticlesEffect, &mut UpdateState)>) {
    for (ref mut effect, ref mut motion) in query.iter_mut() {
        effect.updater.update(motion, time.delta_seconds());
    }
}
