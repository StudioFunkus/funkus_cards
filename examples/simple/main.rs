//! A simple example demonstrating how cards can be added to a scene
use funkus_cards::prelude::*;

use bevy::prelude::*;

const CARD_SIZE: Vec2 = Vec2 { x: 140.0, y: 190.0 };

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, CardsPlugin::<SimpleCard>::default()));

    // Spawn and populate hand and deck
    app.add_systems(
        Startup,
        (spawn_hand_and_deck, ApplyDeferred, populate_deck).chain(),
    );

    app.add_systems(Update, draw_cards.run_if(run_once));

    app.run()
}

#[derive(Clone, Debug, Default)]
struct SimpleCard {
    value: u8,
}

impl CardData for SimpleCard {
    type Output = SimpleCard;

    fn card_front(&self) -> impl Bundle {}
}

#[tracing::instrument(skip_all)]
fn spawn_hand_and_deck(mut commands: Commands, window_query: Query<&Window>) {
    info!("Started");

    // Window, for positioning hand
    let window = window_query.single().unwrap();

    // Deck itself
    let deck = commands
        .spawn((Deck::<SimpleCard>::default(), Name::new("Deck")))
        .id();

    // Then the hand, with the deck as its source.
    commands.spawn((
        Hand::<SimpleCard>::new(5, deck, CARD_SIZE, 5.0),
        Name::new("Hand"),
        Transform::from_xyz(0.0, -(window.height() / 2.0) + (CARD_SIZE.y / 2.0), 0.0),
    ));

    info!("Done");
}

#[tracing::instrument(skip_all)]
fn populate_deck(mut deck_query: Query<&mut Deck<SimpleCard>>) {
    info!("Started");

    let mut deck = deck_query.single_mut().unwrap();

    for index in 0 as u8..9 as u8 {
        deck.add_card(SimpleCard { value: index });
    }

    info!("Done");
}

#[tracing::instrument(skip_all)]
fn draw_cards(mut commands: Commands, hand_query: Query<Entity, With<Deck<SimpleCard>>>) {
    info!("Starting");

    let hand_entity = hand_query.single().unwrap();

    commands.trigger(DrawToHand {
        hand_entity,
        number_to_draw: 5,
    });

    info!("Done")
}
