//! Argument builders for the command builder API.
//!
//! These provide a fluent interface for defining command arguments.

use crate::{ctx::CommandContext, Suggestion};

use super::{
    primitive::{PrimitiveArgument, PrimitiveArgumentType},
    CommandArgumentNode,
};

/// Trait for argument builders used in the command builder API.
pub trait ArgumentBuilder {
    /// Build this argument into a `CommandArgumentNode`.
    fn build(self) -> CommandArgumentNode;
}

// ============================================================================
// Integer Argument
// ============================================================================

/// Builder for integer arguments.
pub struct IntArg {
    name: String,
    required: bool,
    min: Option<i32>,
    max: Option<i32>,
}

impl IntArg {
    /// Create a new integer argument with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            required: true,
            min: None,
            max: None,
        }
    }

    /// Make this argument optional.
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }

    /// Set the minimum value.
    pub fn min(mut self, min: i32) -> Self {
        self.min = Some(min);
        self
    }

    /// Set the maximum value.
    pub fn max(mut self, max: i32) -> Self {
        self.max = Some(max);
        self
    }

    /// Set a range for the value.
    pub fn range(mut self, min: i32, max: i32) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }
}

impl ArgumentBuilder for IntArg {
    fn build(self) -> CommandArgumentNode {
        CommandArgumentNode {
            name: self.name,
            required: self.required,
            primitive: PrimitiveArgument::int(self.min, self.max),
            suggester: default_suggester,
        }
    }
}

// ============================================================================
// Long Argument
// ============================================================================

/// Builder for long integer arguments.
pub struct LongArg {
    name: String,
    required: bool,
    min: Option<i64>,
    max: Option<i64>,
}

impl LongArg {
    /// Create a new long argument with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            required: true,
            min: None,
            max: None,
        }
    }

    /// Make this argument optional.
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }

    /// Set the minimum value.
    pub fn min(mut self, min: i64) -> Self {
        self.min = Some(min);
        self
    }

    /// Set the maximum value.
    pub fn max(mut self, max: i64) -> Self {
        self.max = Some(max);
        self
    }

    /// Set a range for the value.
    pub fn range(mut self, min: i64, max: i64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }
}

impl ArgumentBuilder for LongArg {
    fn build(self) -> CommandArgumentNode {
        CommandArgumentNode {
            name: self.name,
            required: self.required,
            primitive: PrimitiveArgument::long(self.min, self.max),
            suggester: default_suggester,
        }
    }
}

// ============================================================================
// Float Argument
// ============================================================================

/// Builder for float arguments.
pub struct FloatArg {
    name: String,
    required: bool,
    min: Option<f32>,
    max: Option<f32>,
}

impl FloatArg {
    /// Create a new float argument with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            required: true,
            min: None,
            max: None,
        }
    }

    /// Make this argument optional.
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }

    /// Set the minimum value.
    pub fn min(mut self, min: f32) -> Self {
        self.min = Some(min);
        self
    }

    /// Set the maximum value.
    pub fn max(mut self, max: f32) -> Self {
        self.max = Some(max);
        self
    }

    /// Set a range for the value.
    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }
}

impl ArgumentBuilder for FloatArg {
    fn build(self) -> CommandArgumentNode {
        CommandArgumentNode {
            name: self.name,
            required: self.required,
            primitive: PrimitiveArgument::float(self.min, self.max),
            suggester: default_suggester,
        }
    }
}

// ============================================================================
// Boolean Argument
// ============================================================================

/// Builder for boolean arguments.
pub struct BoolArg {
    name: String,
    required: bool,
}

impl BoolArg {
    /// Create a new boolean argument with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            required: true,
        }
    }

    /// Make this argument optional.
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

impl ArgumentBuilder for BoolArg {
    fn build(self) -> CommandArgumentNode {
        CommandArgumentNode {
            name: self.name,
            required: self.required,
            primitive: PrimitiveArgument::bool(),
            suggester: bool_suggester,
        }
    }
}

fn bool_suggester(ctx: &mut CommandContext) -> Vec<Suggestion> {
    let input = ctx.input.peek_string().to_lowercase();
    ctx.input.read_string();

    let mut suggestions = Vec::new();
    if "true".starts_with(&input) {
        suggestions.push(Suggestion::of("true"));
    }
    if "false".starts_with(&input) {
        suggestions.push(Suggestion::of("false"));
    }
    suggestions
}

// ============================================================================
// String Arguments
// ============================================================================

/// Builder for single-word string arguments.
pub struct WordArg {
    name: String,
    required: bool,
}

impl WordArg {
    /// Create a new word argument with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            required: true,
        }
    }

    /// Make this argument optional.
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

impl ArgumentBuilder for WordArg {
    fn build(self) -> CommandArgumentNode {
        CommandArgumentNode {
            name: self.name,
            required: self.required,
            primitive: PrimitiveArgument::word(),
            suggester: default_suggester,
        }
    }
}

/// Builder for quotable string arguments.
pub struct StringArg {
    name: String,
    required: bool,
}

impl StringArg {
    /// Create a new string argument with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            required: true,
        }
    }

    /// Make this argument optional.
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

impl ArgumentBuilder for StringArg {
    fn build(self) -> CommandArgumentNode {
        CommandArgumentNode {
            name: self.name,
            required: self.required,
            primitive: PrimitiveArgument::quotable(),
            suggester: default_suggester,
        }
    }
}

/// Builder for greedy string arguments (consumes rest of input).
pub struct GreedyStringArg {
    name: String,
}

impl GreedyStringArg {
    /// Create a new greedy string argument with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl ArgumentBuilder for GreedyStringArg {
    fn build(self) -> CommandArgumentNode {
        CommandArgumentNode {
            name: self.name,
            required: true, // Greedy args are always required (they consume everything)
            primitive: PrimitiveArgument::greedy(),
            suggester: default_suggester,
        }
    }
}

// ============================================================================
// GameMode Argument
// ============================================================================

/// Builder for gamemode arguments.
pub struct GameModeArg {
    name: String,
    required: bool,
}

impl GameModeArg {
    /// Create a new gamemode argument with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            required: true,
        }
    }

    /// Make this argument optional.
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

impl ArgumentBuilder for GameModeArg {
    fn build(self) -> CommandArgumentNode {
        CommandArgumentNode {
            name: self.name,
            required: self.required,
            primitive: PrimitiveArgument {
                argument_type: PrimitiveArgumentType::GameMode,
                flags: None,
            },
            suggester: gamemode_suggester,
        }
    }
}

fn gamemode_suggester(ctx: &mut CommandContext) -> Vec<Suggestion> {
    let input = ctx.input.peek_string().to_lowercase();
    ctx.input.read_string();

    let modes = ["survival", "creative", "adventure", "spectator"];
    modes
        .iter()
        .filter(|m| m.starts_with(&input))
        .map(|m| Suggestion::of(*m))
        .collect()
}

// ============================================================================
// Default Suggester
// ============================================================================

fn default_suggester(ctx: &mut CommandContext) -> Vec<Suggestion> {
    ctx.input.read_string();
    vec![]
}
