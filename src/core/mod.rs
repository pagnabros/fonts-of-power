pub mod affixes;
pub mod dice;
pub mod resting;
pub mod skills;
pub mod stats;

use bevy::app::{App, Plugin};

pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, _app: &mut App) {}
}
