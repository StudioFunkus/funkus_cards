use crate::*;

use bevy_tweening::{
    Animator, Tween,
    lens::{TransformPositionLens, TransformScaleLens},
};

use std::time::Duration;

pub(super) fn plugin<T>(app: &mut App)
where
    T: Clone + Send + Sync + CardData + 'static,
{
    app.add_event::<UpdateCardOrigins>()
        .add_event::<CardHover>()
        .add_event::<CardOut>()
        .add_event::<DrawToHand>();

    app.add_observer(update_card_origins::<T>)
        .add_observer(draw_to_hand::<T>)
        .add_observer(card_hover::<T>)
        .add_observer(card_out::<T>);

    app.add_systems(Update, cleanup_animators::<Transform>);
}

#[derive(Event)]
pub struct CardHover {
    pub entity: Entity,
}

impl From<Pointer<Over>> for CardHover {
    fn from(event: Pointer<Over>) -> Self {
        CardHover {
            entity: event.target,
        }
    }
}

pub fn card_hover<T>(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    mut cards_query: Query<&mut Card<T>, With<Transform>>,
    plugin_settings: Res<CardsPluginSettings>,
) where
    T: Send + Sync + CardData + 'static,
{
    if let Ok(card) = cards_query.get_mut(trigger.target()) {
        let scale_tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs_f32(plugin_settings.card_tween_scale_duration),
            TransformScaleLens {
                start: card.origin.scale,
                end: card.origin.scale * plugin_settings.card_tween_scale_magnitude,
            },
        );

        commands
            .entity(trigger.target())
            .with_child(Animator::new(scale_tween).with_target(trigger.target()));
    }
}

#[derive(Event)]
pub struct CardOut {
    pub entity: Entity,
}

impl From<Pointer<Out>> for CardOut {
    fn from(event: Pointer<Out>) -> Self {
        CardOut {
            entity: event.target,
        }
    }
}

pub fn card_out<T>(
    trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    mut cards_query: Query<&mut Card<T>, With<Transform>>,
    plugin_settings: Res<CardsPluginSettings>,
) where
    T: Send + Sync + CardData + 'static,
{
    if let Ok(card) = cards_query.get_mut(trigger.target()) {
        let scale_tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs_f32(plugin_settings.card_tween_scale_duration),
            TransformScaleLens {
                start: card.origin.scale * plugin_settings.card_tween_scale_magnitude,
                end: card.origin.scale,
            },
        );

        commands
            .entity(trigger.target())
            .with_child(Animator::new(scale_tween).with_target(trigger.target()));
    }
}

pub fn spawn_cards<T>(
    mut commands: Commands,
    cards_to_spawn: Vec<T>,
    card_size: Vec2,
) -> Vec<(Entity, T)>
where
    T: Send + Sync + Clone + CardData + 'static,
{
    let mut spawned_cards: Vec<(Entity, T)> = Vec::new();

    for card_type in cards_to_spawn {
        // Build and spawn the card component
        let card_component = Card {
            data: card_type.clone(),
            origin: Transform::default(),
            size: card_size,
        };
        let card_entity = commands.spawn(card_component).id();

        // Insert the card_front from the card_type
        commands.entity(card_entity).insert(card_type.card_front());

        spawned_cards.push((card_entity, card_type));
    }

    spawned_cards
}

#[derive(Event)]
pub struct DrawToHand {
    pub hand_entity: Entity,
    pub number_to_draw: usize,
}

pub fn draw_to_hand<T>(
    trigger: Trigger<DrawToHand>,
    mut commands: Commands,
    mut hands_query: Query<&mut Hand<T>>,
    mut decks_query: Query<&mut Deck<T>>,
) where
    T: Clone + Send + Sync + CardData + 'static,
{
    let mut hand = hands_query.get_mut(trigger.hand_entity).unwrap();
    let mut deck = decks_query.get_mut(hand.source).unwrap();
    let mut number_to_draw = trigger.number_to_draw.clone();

    if (trigger.number_to_draw + hand.get_card_count()) > hand.card_limit {
        number_to_draw = hand.card_limit - hand.get_card_count();
    }
    if number_to_draw > deck.get_card_count() {
        number_to_draw = deck.get_card_count();
    }

    let drawn_cards = deck.draw_n(number_to_draw);

    // Spawn cards and add to hand
    let spawned_cards = spawn_cards(
        commands.reborrow(),
        drawn_cards.into_iter().collect(),
        hand.card_size,
    );
    hand.add_cards(spawned_cards.clone());

    // Make the cards children of the hand
    commands.entity(trigger.hand_entity).add_children(
        spawned_cards
            .iter()
            .map(|(entity, _)| *entity)
            .collect::<Vec<Entity>>()
            .as_slice(),
    );

    // Then trigger an update of card origins for that hand
    commands.trigger(UpdateCardOrigins(trigger.hand_entity));
}

#[derive(Event)]
pub struct UpdateCardOrigins(pub Entity);

pub fn update_card_origins<T>(
    trigger: Trigger<UpdateCardOrigins>,
    mut commands: Commands,
    mut hand_query: Query<&mut Hand<T>>,
    mut card_query: Query<(Entity, &mut Card<T>, &Transform)>,
    plugin_settings: Res<CardsPluginSettings>,
) where
    T: Clone + Send + Sync + CardData + 'static,
{
    info!("Update card origins for: {:?}", trigger.0);
    let mut hand = hand_query.get_mut(trigger.0).unwrap();
    let cards_count = hand.get_card_count();

    let card_size = hand.card_size.clone();
    let card_spacing = hand.card_spacing.clone();

    for (index, (entity, _)) in hand.cards.iter_mut().enumerate() {
        let (card_entity, mut card, card_transform) = card_query.get_mut(*entity).unwrap();

        let starting_origin = card.origin.translation;
        let mut new_origin = starting_origin;
        new_origin.x = (index as f32 * (card_size.x + card_spacing))
            - ((cards_count as f32 - 1.0) / 2.0 * (card_size.x + card_spacing));

        card.origin.translation = new_origin;

        let translation_tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs_f32(plugin_settings.card_tween_translate_duration),
            TransformPositionLens {
                start: card_transform.translation,
                end: card.origin.translation,
            },
        );

        commands
            .entity(card_entity)
            .with_child(Animator::new(translation_tween).with_target(card_entity));
    }
}

fn cleanup_animators<T>(mut commands: Commands, animator_query: Query<(Entity, &Animator<T>)>)
where
    T: Component,
{
    for (entity, animator) in animator_query {
        if animator.tweenable().progress() == 1.0 {
            commands.entity(entity).despawn();
        }
    }
}
