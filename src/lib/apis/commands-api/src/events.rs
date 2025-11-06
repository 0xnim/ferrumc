//! Command response events

use bevy_ecs::prelude::*;
use ferrumc_text::TextComponent;

/// Event requesting a command response to be sent
///
/// **NOTE:** This is `pub(crate)` - plugins cannot create this directly.
/// Use `CommandsAPI::send_to_player()` or related methods instead.
#[derive(Event, Clone)]
pub struct SendCommandResponseEvent {
    pub(crate) receiver: Option<Entity>,
    pub(crate) message: TextComponent,
    pub(crate) actionbar: bool,
}
