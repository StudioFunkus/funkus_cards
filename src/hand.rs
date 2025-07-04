use crate::*;

use std::{collections::VecDeque, fmt::Debug};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Name, Transform, Visibility::Visible)]
pub struct Hand<T>
where
    T: Send + Sync + Clone + Debug + CardData + 'static,
{
    pub cards: VecDeque<(Entity, T)>,
    pub card_limit: usize,
    pub source: Entity,
    pub card_size: Vec2,
    pub card_spacing: f32,
}

impl<T> Hand<T>
where
    T: Send + Sync + Clone + Debug + CardData + 'static,
{
    pub fn new(card_limit: usize, source: Entity, card_size: Vec2, card_spacing: f32) -> Hand<T> {
        Hand {
            cards: VecDeque::new(),
            card_limit,
            source,
            card_size,
            card_spacing,
        }
    }

    pub fn get_card_count(&self) -> usize {
        self.cards.len()
    }

    pub fn add_card(&mut self, card_entity: Entity, card: T) {
        self.cards.push_back((card_entity, card));
    }

    pub fn add_cards(&mut self, cards: Vec<(Entity, T)>) {
        self.cards.extend(cards);
    }

    pub fn as_vec(&self) -> Vec<(usize, &(Entity, T))> {
        self.cards.iter().enumerate().collect()
    }

    pub fn despawn_card_in_hand(&mut self, mut commands: Commands, entity_to_despawn: Entity) {
        for (index, (entity, _card)) in self.cards.clone().iter().enumerate() {
            if *entity == entity_to_despawn {
                commands.entity(entity_to_despawn).despawn();
                self.cards.remove(index);
                break;
            }
        }

        commands.trigger(UpdateCardOrigins(self.source));
    }

    pub fn empty_hand(&mut self, mut commands: Commands) {
        let cards: Vec<(Entity, T)> = self.cards.drain(..).collect();

        for (entity, _card) in cards {
            commands.entity(entity).despawn();
        }
    }
}
