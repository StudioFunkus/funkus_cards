//! A simple example demonstrating how cards can be added to a scene

use bevy::prelude::*;
use funkus_cards::prelude::*;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Window {
                title: "funkus_cards: simple".to_string(),
                resolution: (1920.0, 1080.0).into(),
                ..default()
            }
            .into(),
            ..default()
        }),
    );

    app.add_plugins(CardsPlugin);

    let rarity = CardRarities::Common(10);
    println!("{:?}", rarity.get());

    app.run()
}

#[derive(Debug)]
#[allow(dead_code)]
enum CardRarities {
    Common(usize),
    Uncommon(usize),
    Rare(usize),
    Epic(usize),
    Legendary(usize),
}

impl Default for CardRarities {
    fn default() -> Self {
        CardRarities::Common(10)
    }
}

impl Rarity for CardRarities {}
