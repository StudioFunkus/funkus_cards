//! # Card
//!
//! Mark entities as cards with the [`Card`] component, then add any optional
//! components to add behaviours, such as [`CardRarity`].

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    info!("Adding card plugin");

    app.register_type::<Card>();
}

#[derive(Component, Reflect, Clone, Default, Debug)]
#[reflect(Component)]
pub struct Card;

/// ## Rarity
///
/// Implement this trait on an enum to configure your own card rarities.
/// See [`CardRarity`], the default implementation, for an example
pub trait Rarity {
    fn get(&self) -> &Self {
        self
    }
}

#[derive(Component, Reflect, Clone, Default, Debug)]
#[reflect(Component)]
pub enum CardRarity {
    #[default]
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Rarity for CardRarity {}
