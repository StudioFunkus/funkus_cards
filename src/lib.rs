//! # Funkus Cards
//!
//! A collection of utilities for card-based systems in the Bevy game engine.

use bevy::prelude::*;

// Module declarations
mod card;
mod deck;
mod events;
mod hand;

pub struct CardsPlugin;

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((card::plugin, hand::plugin, deck::plugin));
    }
}

pub mod prelude {
    pub use super::CardsPlugin;
    pub use crate::card::*;
    pub use crate::deck::*;
    pub use crate::events::*;
    pub use crate::hand::*;
}
