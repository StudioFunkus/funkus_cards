use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    card::Card,
    deck::{CardSource, Deck},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Hand<Deck>>();
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Hand<T: CardSource> {
    cards: VecDeque<(Card, Entity)>,
    card_limit: usize,
    source: T,
}

#[allow(dead_code)]
impl<T: CardSource> Hand<T> {
    pub fn draw_one(&mut self, commands: Commands) -> Result {
        if self.cards.len() < self.card_limit {
            if let Some(drawn_card) = self.source.draw_one() {
                let card_entity: Entity = spawn_card(commands, drawn_card.clone());
                self.cards.push_back((drawn_card, card_entity));
            };
        }

        Ok(())
    }
}

pub fn spawn_card(mut commands: Commands, card_component: Card) -> Entity {
    commands.spawn((Name::new("Card"), card_component)).id()
}
