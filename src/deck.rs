use std::collections::VecDeque;

use bevy::prelude::*;

use crate::card::Card;

pub(super) fn plugin(app: &mut App) {
    info!("Adding deck plugin");

    app.register_type::<Deck>();
}

#[allow(dead_code)]
pub trait CardSource {
    fn draw_one(&mut self) -> Option<Card>;
    fn draw_n(&mut self, n: usize) -> VecDeque<Card>;
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl CardSource for Deck {
    fn draw_one(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    fn draw_n(&mut self, n: usize) -> VecDeque<Card> {
        if self.cards.len() >= n {
            self.cards.drain(..n).collect()
        } else {
            self.cards.drain(..).collect()
        }
    }
}
