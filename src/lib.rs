//! # Funkus Cards
//!
//! A collection of utilities for card-based systems in the Bevy game engine.

pub(crate) use bevy_app::prelude::*;
pub(crate) use bevy_core_pipeline::prelude::*;
pub(crate) use bevy_ecs::prelude::*;
pub(crate) use bevy_math::prelude::*;
pub(crate) use bevy_picking::prelude::*;
pub(crate) use bevy_reflect::prelude::*;
pub(crate) use bevy_render::prelude::*;
pub(crate) use bevy_transform::prelude::*;

pub(crate) use bevy_log::info;
pub(crate) use bevy_render::view::RenderLayers;

#[cfg(feature = "dev")]
use bevy_lunex::UiLunexDebugPlugin;
use bevy_lunex::{UiLunexPlugin, UiLunexPlugins, UiSourceCamera};
use bevy_tweening::TweeningPlugin;

use std::{fmt::Debug, marker::PhantomData};

use crate::{
    card::{Card, CardData},
    deck::{CardSource, Deck},
    events::{CardHover, CardOut, DrawToHand, UpdateCardOrigins},
    hand::Hand,
};

// Module declarations
mod card;
mod deck;
mod events;
mod hand;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::CardsPlugin;
    pub use crate::card::*;
    pub use crate::deck::*;
    pub use crate::events::*;
    pub use crate::hand::*;
}

#[derive(Default)]
pub struct CardsPlugin<T: Send + Sync + Clone + Debug + CardData + 'static>(pub PhantomData<T>);

impl<T: Send + Sync + Clone + Debug + CardData + 'static> Plugin for CardsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateCardOrigins>()
            .add_event::<CardHover>()
            .add_event::<CardOut>()
            .add_event::<DrawToHand>();

        app.add_plugins(events::plugin::<T>);

        if !app.is_plugin_added::<TweeningPlugin>() {
            app.add_plugins(TweeningPlugin);
        }

        // Ui
        if !app.is_plugin_added::<UiLunexPlugin>() {
            app.add_plugins(UiLunexPlugins);
        }
        // Ui - Debug
        #[cfg(feature = "dev")]
        if !app.is_plugin_added::<UiLunexDebugPlugin>() {
            app.add_plugins(UiLunexDebugPlugin::<21, 31>);
        }

        // Spawn card specific camera
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct CardsPluginSettings {
    pub card_tween_translate_duration: f32,
    pub card_tween_scale_duration: f32,
    pub card_tween_scale_magnitude: f32,
}

impl Default for CardsPluginSettings {
    fn default() -> Self {
        Self {
            card_tween_translate_duration: 1.0,
            card_tween_scale_duration: 1.0,
            card_tween_scale_magnitude: 1.1,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        UiSourceCamera::<20>,
        Transform::from_translation(Vec3::Z * 1000.0),
        RenderLayers::from_layers(&[20, 21]),
    ));
}
