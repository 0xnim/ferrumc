//! Command builder for the fluent API.
//!
//! This module provides a builder pattern for creating commands
//! that can be registered via the mod API.

use std::sync::Arc;

use crate::{
    arg::{builders::ArgumentBuilder, CommandArgumentNode},
    handler::{CommandHandler, CommandResult},
    ctx::CommandContext,
    Command,
};

/// A command built via the builder API.
///
/// This contains all the information needed to register and execute
/// a command, including the handler function.
pub struct BuiltCommand {
    /// The underlying command structure (name + args).
    pub command: Arc<Command>,

    /// Optional description of what the command does.
    pub description: Option<String>,

    /// Required permission to execute this command.
    pub permission: Option<String>,

    /// Whether this command requires a player sender.
    pub requires_player: bool,

    /// The handler function to execute.
    pub handler: Arc<dyn CommandHandler>,
}

/// Builder for constructing commands with a fluent API.
///
/// # Example
///
/// ```ignore
/// use ferrumc_commands::{CommandBuilder, arg::builders::FloatArg};
///
/// let command = CommandBuilder::new("teleport")
///     .description("Teleport to coordinates")
///     .permission("server.teleport")
///     .requires_player()
///     .arg(FloatArg::new("x"))
///     .arg(FloatArg::new("y"))
///     .arg(FloatArg::new("z"))
///     .handler(|ctx| {
///         let x = ctx.arg::<Float>("x")?;
///         let y = ctx.arg::<Float>("y")?;
///         let z = ctx.arg::<Float>("z")?;
///         ctx.sender.send_message(
///             format!("Teleporting to {}, {}, {}", *x, *y, *z).into(),
///             false
///         );
///         Ok(())
///     });
/// ```
pub struct CommandBuilder {
    name: String,
    description: Option<String>,
    permission: Option<String>,
    requires_player: bool,
    args: Vec<CommandArgumentNode>,
    handler: Option<Arc<dyn CommandHandler>>,
}

impl CommandBuilder {
    /// Create a new command builder with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            permission: None,
            requires_player: false,
            args: Vec::new(),
            handler: None,
        }
    }

    /// Set the description of the command.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set the required permission for this command.
    pub fn permission(mut self, perm: impl Into<String>) -> Self {
        self.permission = Some(perm.into());
        self
    }

    /// Mark this command as requiring a player sender.
    pub fn requires_player(mut self) -> Self {
        self.requires_player = true;
        self
    }

    /// Add an argument to the command.
    pub fn arg<A: ArgumentBuilder>(mut self, arg: A) -> Self {
        self.args.push(arg.build());
        self
    }

    /// Set the handler function for this command.
    ///
    /// The handler receives a `CommandContext` and should return `Ok(())`
    /// on success or `Err(Box<TextComponent>)` with an error message.
    pub fn handler<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut CommandContext) -> CommandResult + Send + Sync + 'static,
    {
        self.handler = Some(Arc::new(f));
        self
    }

    /// Build the command.
    ///
    /// # Panics
    ///
    /// Panics if no handler was set.
    pub fn build(self) -> BuiltCommand {
        let handler = self
            .handler
            .expect("CommandBuilder::build() called without setting a handler");

        // The Command struct uses &'static str for name, but we have a String.
        // We need to leak it to get a static lifetime. This is acceptable since
        // commands are registered once and live for the server lifetime.
        let name: &'static str = Box::leak(self.name.into_boxed_str());

        BuiltCommand {
            command: Arc::new(Command {
                name,
                args: self.args,
            }),
            description: self.description,
            permission: self.permission,
            requires_player: self.requires_player,
            handler,
        }
    }
}

impl Command {
    /// Create a new command builder with the given name.
    ///
    /// This is a convenience method equivalent to `CommandBuilder::new(name)`.
    pub fn builder(name: impl Into<String>) -> CommandBuilder {
        CommandBuilder::new(name)
    }
}
