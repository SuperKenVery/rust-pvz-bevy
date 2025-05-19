use bevy::prelude::*;

use crate::GameState;

pub struct EndScreen;

impl Plugin for EndScreen {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::End { win: true }), show_win);
        app.add_systems(OnEnter(GameState::End { win: false }), show_lose);
    }
}

fn show_win(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("savedgames.png")),
        Text2d::new("You win"),
        Transform::from_xyz(0., 0., 100.),
    ));
}

fn show_lose(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("savedgames.png")),
        Text2d::new("You lose"),
        Transform::from_xyz(0., 0., 100.),
    ));
}
