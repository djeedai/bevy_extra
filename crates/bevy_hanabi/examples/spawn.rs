use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use bevy_hanabi::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::TRACE,
            filter: "bevy_hanabi=trace".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(HanabiPlugin)
        .add_startup_system(setup.system())
        .run();

    Ok(())
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let data: &[u8] = &[255u8; 4];
    let image: Handle<Image> = images.add(Image::new_fill(
        Extent3d::default(),
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8Unorm,
    ));

    commands
        .spawn()
        .insert(Transform::identity())
        .insert(GlobalTransform::identity())
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .insert(image)
        .insert_bundle(ParticlesEffect::new_bundle(
            128,
            Spawner::new(10.0, Vec3::ZERO, Vec3::new(1., 2., 3.)),
            Updater::default(),
        ));
}
