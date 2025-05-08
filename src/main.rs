use bevy::prelude::*;
use bevy::window::WindowResolution;
use vleue_kinetoscope::AnimatedImagePlugin;

mod plugins;
use plugins::{toolbar, GridPos, PlayerTextureResources};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Plant vs Zombies".into(),
                // resolution: WindowResolution::new(800., 600.).with_scale_factor_override(1.0),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AnimatedImagePlugin)
        .add_plugins(plugins::land::LandPlugin)
        .add_plugins(plugins::zombies::ZombiePlugin)
        .add_plugins(toolbar::ToolbarPlugin)
        .add_systems(Startup, setup)
        .add_systems(PreStartup, (PlayerTextureResources::setup, setup_resources))
        .add_systems(Startup, debug_setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_resources(mut commands: Commands) {
    commands.insert_resource(plugins::land::LandPlants::default());
}

fn debug_setup(mut commands: Commands, textures: Res<PlayerTextureResources>) {
    plugins::zombies::basic_zombie::BasicZombie::create(
        GridPos::new(8, 0),
        &mut commands,
        &textures,
    );
    plugins::zombies::basic_zombie::BasicZombie::create(
        GridPos::new(8, 1),
        &mut commands,
        &textures,
    );
}
