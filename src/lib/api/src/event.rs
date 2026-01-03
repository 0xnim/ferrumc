//! Event system for mod communication.
//!
//! This module wraps Bevy's message system with a mod-friendly API.
//! Events can be subscribed to with priority ordering.

use bevy_ecs::prelude::*;

/// Marker trait for events that mods can listen to.
///
/// Events must also implement Bevy's `Event` trait (via `#[derive(Event)]`).
/// This trait marks types as part of the mod API.
pub trait Event: bevy_ecs::event::Event + Clone + Send + Sync + 'static {}

/// Priority for event handlers.
///
/// Lower priority values run first. Use `Monitor` for read-only
/// handlers that should run last and not modify the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum EventPriority {
    /// Runs first, can modify or cancel event
    Highest = 0,
    /// Runs early
    High = 1,
    /// Default priority
    #[default]
    Normal = 2,
    /// Runs late
    Low = 3,
    /// Runs very late
    Lowest = 4,
    /// Runs last, should not modify event (monitoring only)
    Monitor = 5,
}

impl EventPriority {
    /// Get all priorities in execution order.
    pub fn all() -> &'static [EventPriority] {
        &[
            EventPriority::Highest,
            EventPriority::High,
            EventPriority::Normal,
            EventPriority::Low,
            EventPriority::Lowest,
            EventPriority::Monitor,
        ]
    }
}

/// A handler function for an event type.
pub type EventHandler<E> = Box<dyn Fn(&mut E, &mut World) + Send + Sync>;

/// Trait for events that can be cancelled.
///
/// Implement this for events where handlers should be able to
/// prevent the default action from occurring.
pub trait CancellableEvent: Event {
    /// Check if the event has been cancelled.
    fn is_cancelled(&self) -> bool;

    /// Set the cancelled state.
    fn set_cancelled(&mut self, cancelled: bool);

    /// Cancel the event (shorthand for `set_cancelled(true)`).
    fn cancel(&mut self) {
        self.set_cancelled(true);
    }
}

/// Helper macro to implement Event for existing message types.
///
/// # Example
///
/// ```ignore
/// impl_event!(PlayerJoined, PlayerLeft, PlayerDamaged);
/// ```
#[macro_export]
macro_rules! impl_event {
    ($($event:ty),* $(,)?) => {
        $(
            impl $crate::event::Event for $event {}
        )*
    };
}

/// Type alias for event handler function wrapped in Arc.
pub type ArcEventHandler<E> = std::sync::Arc<dyn Fn(&mut E, &mut World) + Send + Sync>;

/// A stored event listener with priority information.
#[derive(Clone)]
pub struct EventListener<E: Event> {
    /// The handler function
    pub handler: ArcEventHandler<E>,
    /// Handler priority
    pub priority: EventPriority,
    /// Mod that registered this listener
    pub mod_id: &'static str,
}

impl<E: Event> EventListener<E> {
    /// Create a new event listener.
    pub fn new(
        priority: EventPriority,
        mod_id: &'static str,
        handler: impl Fn(&mut E, &mut World) + Send + Sync + 'static,
    ) -> Self {
        Self {
            handler: std::sync::Arc::new(handler),
            priority,
            mod_id,
        }
    }
}
