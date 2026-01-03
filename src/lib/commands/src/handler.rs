//! Command handler traits and types for the builder API.

use ferrumc_text::TextComponent;

use crate::ctx::CommandContext;

/// Result type for command execution.
pub type CommandResult = Result<(), Box<TextComponent>>;

/// Trait for command execution handlers.
///
/// This trait is automatically implemented for closures that take
/// `&mut CommandContext` and return `CommandResult`.
pub trait CommandHandler: Send + Sync + 'static {
    /// Execute the command with the given context.
    fn execute(&self, ctx: &mut CommandContext) -> CommandResult;
}

/// Blanket implementation for closures.
impl<F> CommandHandler for F
where
    F: Fn(&mut CommandContext) -> CommandResult + Send + Sync + 'static,
{
    fn execute(&self, ctx: &mut CommandContext) -> CommandResult {
        self(ctx)
    }
}
