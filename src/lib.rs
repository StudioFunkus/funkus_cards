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
pub(crate) use bevy_reflect::{GetTypeRegistration, Typed};
pub(crate) use bevy_render::view::RenderLayers;

#[cfg(feature = "dev")]
use bevy_lunex::UiLunexDebugPlugin;
use bevy_lunex::{UiLunexPlugin, UiLunexPlugins, UiSourceCamera};
use bevy_tweening::TweeningPlugin;

use std::{fmt::Debug, marker::PhantomData};

use crate::{card::*, deck::*, events::*, hand::*};

// Module declarations
mod card;
mod deck;
mod dev_tools;
mod events;
mod hand;
mod render;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::CardsPlugin;
    pub use crate::card::*;
    pub use crate::deck::*;
    #[cfg(feature = "dev")]
    pub use crate::dev_tools::*;
    pub use crate::events::*;
    pub use crate::hand::*;
    pub use crate::render::*;
}

#[derive(Default)]
pub struct CardsPlugin<
    T: Send + Sync + Clone + Debug + FromReflect + Typed + GetTypeRegistration + CardData + 'static,
>(pub PhantomData<T>);

impl<
    T: Send + Sync + Clone + Debug + FromReflect + Typed + GetTypeRegistration + CardData + 'static,
> Plugin for CardsPlugin<T>
{
    fn build(&self, app: &mut App) {
        // Internal plugins
        app.add_plugins((events::plugin::<T>, render::plugin));

        // Animations
        if !app.is_plugin_added::<TweeningPlugin>() {
            app.add_plugins(TweeningPlugin);
        }

        // Ui
        if !app.is_plugin_added::<UiLunexPlugin>() {
            app.add_plugins(UiLunexPlugins);
        }

        // Dev tools
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin::<T>);
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
