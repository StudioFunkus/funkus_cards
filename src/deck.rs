use crate::*;

use std::collections::VecDeque;
use std::fmt::Debug;

pub trait CardSource<T>
where
    T: Send + Sync + Clone + Debug + CardData + 'static,
{
    fn get_card_count(&self) -> usize;
    fn draw_one(&mut self) -> Option<T>;
    fn draw_n(&mut self, n: usize) -> VecDeque<T>;

    fn add_card(&mut self, card: T);
    fn add_cards(&mut self, cards: Vec<T>);
}

#[derive(Component, Reflect, Debug, Default, Clone)]
#[reflect(Component)]
#[require(Name)]
pub struct Deck<T>
where
    T: Send + Sync + Clone + Debug + CardData + 'static,
{
    cards: VecDeque<T>,
}

impl<T> CardSource<T> for Deck<T>
where
    T: Send + Sync + Clone + Debug + CardData + 'static,
{
    fn draw_one(&mut self) -> Option<T> {
        self.cards.pop_front()
    }

    fn draw_n(&mut self, n: usize) -> VecDeque<T> {
        if self.cards.len() >= n {
            self.cards.drain(..n).collect()
        } else {
            self.cards.drain(..).collect()
        }
    }

    fn add_card(&mut self, card: T) {
        self.cards.push_back(card);
    }

    fn add_cards(&mut self, cards: Vec<T>) {
        self.cards.extend(cards);
    }

    fn get_card_count(&self) -> usize {
        self.cards.len()
    }
}
