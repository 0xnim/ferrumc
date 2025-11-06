//! Position argument types for commands.

use crate::arg::utils::parser_error;
use crate::arg::{CommandArgument, ParserResult};
use crate::ctx::CommandContext;

use super::primitive::{PrimitiveArgument, PrimitiveArgumentType};

#[derive(Debug, Clone, PartialEq)]
pub enum CoordinateType {
    Absolute(f64),
    Relative(f64),
    Local(f64),
}

impl CoordinateType {
    fn parse(s: &str) -> Result<Self, String> {
        if s.is_empty() {
            return Err("coordinate cannot be empty".to_string());
        }

        if let Some(relative) = s.strip_prefix('~') {
            if relative.is_empty() {
                Ok(CoordinateType::Relative(0.0))
            } else {
                relative
                    .parse::<f64>()
                    .map(CoordinateType::Relative)
                    .map_err(|_| format!("invalid relative coordinate: {}", s))
            }
        } else if let Some(local) = s.strip_prefix('^') {
            if local.is_empty() {
                Ok(CoordinateType::Local(0.0))
            } else {
                local
                    .parse::<f64>()
                    .map(CoordinateType::Local)
                    .map_err(|_| format!("invalid local coordinate: {}", s))
            }
        } else {
            s.parse::<f64>()
                .map(CoordinateType::Absolute)
                .map_err(|_| format!("invalid absolute coordinate: {}", s))
        }
    }
}

/// A 3D position with double precision (x, y, z).
/// Supports absolute, relative (~), and local (^) coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct Vec3Argument {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl CommandArgument for Vec3Argument {
    fn parse(ctx: &mut CommandContext<'_>) -> ParserResult<Self> {
        let x_str = ctx.input.read_string();
        let y_str = ctx.input.read_string();
        let z_str = ctx.input.read_string();

        let x = CoordinateType::parse(&x_str).map_err(|e| parser_error(&e))?;
        let y = CoordinateType::parse(&y_str).map_err(|e| parser_error(&e))?;
        let z = CoordinateType::parse(&z_str).map_err(|e| parser_error(&e))?;

        let (x_val, y_val, z_val) = match (&x, &y, &z) {
            (CoordinateType::Absolute(x), CoordinateType::Absolute(y), CoordinateType::Absolute(z)) => (*x, *y, *z),
            (CoordinateType::Relative(x), CoordinateType::Relative(y), CoordinateType::Relative(z)) => (*x, *y, *z),
            _ => {
                return Err(parser_error(
                    "mixed coordinate types not yet supported - use all absolute, all relative, or all local",
                ))
            }
        };

        Ok(Vec3Argument { x: x_val, y: y_val, z: z_val })
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::Vec3,
            flags: None,
        }
    }
}

/// A 2D position with double precision (x, z).
/// Supports absolute, relative (~), and local (^) coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct Vec2Argument {
    pub x: f64,
    pub z: f64,
}

impl CommandArgument for Vec2Argument {
    fn parse(ctx: &mut CommandContext<'_>) -> ParserResult<Self> {
        let x_str = ctx.input.read_string();
        let z_str = ctx.input.read_string();

        let x = CoordinateType::parse(&x_str).map_err(|e| parser_error(&e))?;
        let z = CoordinateType::parse(&z_str).map_err(|e| parser_error(&e))?;

        let (x_val, z_val) = match (&x, &z) {
            (CoordinateType::Absolute(x), CoordinateType::Absolute(z)) => (*x, *z),
            (CoordinateType::Relative(x), CoordinateType::Relative(z)) => (*x, *z),
            _ => {
                return Err(parser_error(
                    "mixed coordinate types not yet supported - use all absolute or all relative",
                ))
            }
        };

        Ok(Vec2Argument { x: x_val, z: z_val })
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::Vec2,
            flags: None,
        }
    }
}

/// A block position with integer coordinates (x, y, z).
/// Supports absolute, relative (~), and local (^) coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockPosArgument {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl CommandArgument for BlockPosArgument {
    fn parse(ctx: &mut CommandContext<'_>) -> ParserResult<Self> {
        let x_str = ctx.input.read_string();
        let y_str = ctx.input.read_string();
        let z_str = ctx.input.read_string();

        let parse_block_coord = |s: &str| -> Result<i32, String> {
            if s.is_empty() {
                return Err("coordinate cannot be empty".to_string());
            }

            if let Some(relative) = s.strip_prefix('~') {
                if relative.is_empty() {
                    Ok(0)
                } else {
                    relative
                        .parse::<i32>()
                        .map_err(|_| format!("invalid relative coordinate: {}", s))
                }
            } else if let Some(local) = s.strip_prefix('^') {
                if local.is_empty() {
                    Ok(0)
                } else {
                    local
                        .parse::<i32>()
                        .map_err(|_| format!("invalid local coordinate: {}", s))
                }
            } else {
                s.parse::<i32>()
                    .map_err(|_| format!("invalid absolute coordinate: {}", s))
            }
        };

        let x = parse_block_coord(&x_str).map_err(|e| parser_error(&e))?;
        let y = parse_block_coord(&y_str).map_err(|e| parser_error(&e))?;
        let z = parse_block_coord(&z_str).map_err(|e| parser_error(&e))?;

        Ok(BlockPosArgument { x, y, z })
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::BlockPos,
            flags: None,
        }
    }
}

/// A column position with integer coordinates (x, z).
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnPosArgument {
    pub x: i32,
    pub z: i32,
}

impl CommandArgument for ColumnPosArgument {
    fn parse(ctx: &mut CommandContext<'_>) -> ParserResult<Self> {
        let x_str = ctx.input.read_string();
        let z_str = ctx.input.read_string();

        let parse_col_coord = |s: &str| -> Result<i32, String> {
            if s.is_empty() {
                return Err("coordinate cannot be empty".to_string());
            }

            if let Some(relative) = s.strip_prefix('~') {
                if relative.is_empty() {
                    Ok(0)
                } else {
                    relative
                        .parse::<i32>()
                        .map_err(|_| format!("invalid relative coordinate: {}", s))
                }
            } else {
                s.parse::<i32>()
                    .map_err(|_| format!("invalid absolute coordinate: {}", s))
            }
        };

        let x = parse_col_coord(&x_str).map_err(|e| parser_error(&e))?;
        let z = parse_col_coord(&z_str).map_err(|e| parser_error(&e))?;

        Ok(ColumnPosArgument { x, z })
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::ColumnPos,
            flags: None,
        }
    }
}
