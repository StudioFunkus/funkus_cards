use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_reflect::{GetTypeRegistration, Typed};

use crate::*;

pub(super) fn plugin<T: CardData>(app: &mut App)
where
    T: Clone + Debug + FromReflect + Typed + GetTypeRegistration,
{
    // Egui plugin
    if !app.is_plugin_added::<EguiPlugin>() {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        });
    }

    // World inspector
    if !app.is_plugin_added::<WorldInspectorPlugin>() {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    // Debug plugin for BevyLunex
    if !app.is_plugin_added::<UiLunexDebugPlugin>() {
        app.add_plugins(UiLunexDebugPlugin::<21, 31>);
    };

    // Register appropriate types
    app.register_type::<Card<T>>();
}
