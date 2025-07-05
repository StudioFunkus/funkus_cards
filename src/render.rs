use bevy_lunex::{UiFetchFromCamera, UiLayoutRoot};

use crate::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_card_ui);
}

/// Spawns entities required for rendering cards.
///
/// The card specific camera shouldn't interfere with any other cameras
/// added by the user of this crate.
///
/// The Ui root will be used as the basis for rendering all card entities.
///
/// This is an opinionated and assumptive approach, which may require
/// refactoring at a later date to allow users of this crate to
/// define their own behaviours.
fn initialize_card_ui(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera2d,
        UiSourceCamera::<20>,
        Transform::from_translation(Vec3::Z * 1000.0),
        RenderLayers::from_layers(&[20, 21]),
    ));

    // Ui Root
    commands.spawn((CardUi, UiLayoutRoot::new_2d(), UiFetchFromCamera::<20>));
}

/// # CardUi
///
/// Marker component to identify the [`UiLayoutRoot`] used to render cards
#[derive(Component, Reflect)]
#[reflect(Component)]
struct CardUi;
