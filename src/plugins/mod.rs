//! This module contains all the plugins for the game.

const LAND_Z: f32 = 0.;
/// Plants and zombies
///
/// The real z is PLAYERS_Z + row_index,
/// so players at the bottom is on top of those at top
const PLAYERS_Z: f32 = 1.;
/// Flying peanuts or floating suns
const FLYING_Z: f32 = 7.;
/// Toolbar for planting and other actions
const TOOLBAR_Z: f32 = 8.;
/// Floating things like the card that follows mouse when
/// adding plant
const FLOATING_Z: f32 = 9.;

pub mod land;
pub mod plants;
pub mod player;
pub mod toolbar;
pub mod zombies;

pub use land::GridPos;
pub use player::PlayerTextureResources;
