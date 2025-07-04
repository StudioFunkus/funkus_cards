//! # Card
//!
//! Mark entities as cards with the [`Card`] component, then add
//! any required optional components.

use crate::*;

use std::fmt::Debug;

/// # CardData
///
/// Implement this trait on a struct to define your own CardType.
///
/// The methods [`CardData::card_top_scale`] and [`CardData::card_render_layer`]
/// provide default implementations.
///
/// ## Example
///
/// ```rust
/// #[derive(Default)]
/// pub struct MyCard {
///     pub suit: String,
///     pub value: u8,
///     pub card_back: Handle<Image>,
///     pub card_top: Handle<Image>,
/// }
///
/// impl CardData for MyCard {
///     type Output = MyCard;
///
///     fn name(&self) -> String {
///         format!("{} of {}", value, suit)
///     }
///
///     fn card_back(&self) -> Sprite {
///         Sprite {
///             image: self.card_back.clone(),
///             ..default()
///         }
///     }
///
///     fn card_top(&self) -> Sprite {
///         Sprite {
///             image: self.card_top.clone(),
///             ..default()
///         }
///     }
/// }
/// ```
pub trait CardData {
    type Output;

    fn card_front(&self) -> impl Bundle {}
    fn card_back(&self) -> impl Bundle {}
}

/// # Card
///
/// Marker component denoting a Card entity, where a struct that implements
/// [`CardData`] replaces the generic, T.
#[derive(Component, Reflect, Clone, Default, Debug)]
#[require(Name, Transform, Pickable {is_hoverable: true, should_block_lower: true})]
pub struct Card<T>
where
    T: Send + Sync + Clone + Debug + CardData + 'static,
{
    pub origin: Transform,
    pub data: T,
    pub size: Vec2,
}
