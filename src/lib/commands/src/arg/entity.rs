//! Entity and player argument types for commands.

use std::io::Write;

use bevy_ecs::entity::Entity;
use bevy_ecs::query::QueryData;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net_codec::encode::{errors::NetEncodeError, NetEncode, NetEncodeOpts};
use ferrumc_text::{ComponentBuilder, NamedColor};
use tokio::io::AsyncWrite;

use crate::arg::utils::parser_error;
use crate::arg::{CommandArgument, ParserResult};
use crate::ctx::CommandContext;
use crate::wrapper;
use crate::Suggestion;

use super::primitive::{PrimitiveArgument, PrimitiveArgumentFlags, PrimitiveArgumentType};

/// Flags for entity argument parsing.
#[derive(Clone, Debug, PartialEq)]
pub struct EntityArgumentFlags {
    pub single: bool,
    pub players_only: bool,
}

impl NetEncode for EntityArgumentFlags {
    fn encode<W: Write>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        let mut flags: u8 = 0;
        if self.single {
            flags |= 0x01;
        }
        if self.players_only {
            flags |= 0x02;
        }
        writer.write_all(&[flags])?;
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        use tokio::io::AsyncWriteExt;
        let mut flags: u8 = 0;
        if self.single {
            flags |= 0x01;
        }
        if self.players_only {
            flags |= 0x02;
        }
        writer.write_all(&[flags]).await?;
        Ok(())
    }
}

wrapper! {
    /// A player argument that resolves to a single online player entity.
    struct PlayerArgument(Entity);
}

impl CommandArgument for PlayerArgument {
    fn parse(ctx: &mut CommandContext<'_>) -> ParserResult<Self> {
        let username = ctx.input.read_string();
        
        if username.is_empty() {
            return Err(parser_error("player name must not be empty"));
        }

        #[derive(QueryData)]
        struct PlayerQuery {
            entity: Entity,
            identity: &'static PlayerIdentity,
        }

        let mut query = ctx.world.query::<PlayerQuery>();
        
        for player in query.iter(ctx.world) {
            if player.identity.username.eq_ignore_ascii_case(&username) {
                return Ok(PlayerArgument(player.entity));
            }
        }

        Err(Box::new(
            ComponentBuilder::text(format!("Player '{}' not found", username))
                .color(NamedColor::Red)
                .build(),
        ))
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::Entity,
            flags: Some(PrimitiveArgumentFlags::Entity(EntityArgumentFlags {
                single: true,
                players_only: true,
            })),
        }
    }

    fn suggest(ctx: &mut CommandContext<'_>) -> Vec<Suggestion> {
        let partial = ctx.input.peek_string();
        ctx.input.read_string();

        #[derive(QueryData)]
        struct PlayerQuery {
            identity: &'static PlayerIdentity,
        }

        let mut query = ctx.world.query::<PlayerQuery>();
        
        query
            .iter(ctx.world)
            .map(|player| Suggestion::of(&player.identity.username))
            .filter(|sug| {
                sug.content
                    .to_lowercase()
                    .starts_with(&partial.to_lowercase())
            })
            .collect()
    }
}

wrapper! {
    /// An entity argument that can target multiple entities using selectors.
    /// Currently only supports player names, but can be extended for @a, @e, etc.
    struct EntityArgument(Vec<Entity>);
}

impl CommandArgument for EntityArgument {
    fn parse(ctx: &mut CommandContext<'_>) -> ParserResult<Self> {
        let selector = ctx.input.read_string();
        
        if selector.is_empty() {
            return Err(parser_error("entity selector must not be empty"));
        }

        #[derive(QueryData)]
        struct PlayerQuery {
            entity: Entity,
            identity: &'static PlayerIdentity,
        }

        let mut query = ctx.world.query::<PlayerQuery>();
        
        match selector.as_str() {
            "@a" => {
                let entities = query.iter(ctx.world).map(|p| p.entity).collect();
                Ok(EntityArgument(entities))
            }
            "@p" => {
                if let Some(player) = query.iter(ctx.world).next() {
                    Ok(EntityArgument(vec![player.entity]))
                } else {
                    Err(Box::new(
                        ComponentBuilder::text("No players found")
                            .color(NamedColor::Red)
                            .build(),
                    ))
                }
            }
            username => {
                for player in query.iter(ctx.world) {
                    if player.identity.username.eq_ignore_ascii_case(username) {
                        return Ok(EntityArgument(vec![player.entity]));
                    }
                }
                Err(Box::new(
                    ComponentBuilder::text(format!("Entity '{}' not found", username))
                        .color(NamedColor::Red)
                        .build(),
                ))
            }
        }
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument {
            argument_type: PrimitiveArgumentType::Entity,
            flags: Some(PrimitiveArgumentFlags::Entity(EntityArgumentFlags {
                single: false,
                players_only: false,
            })),
        }
    }

    fn suggest(ctx: &mut CommandContext<'_>) -> Vec<Suggestion> {
        let partial = ctx.input.peek_string();
        ctx.input.read_string();

        let mut suggestions = vec![
            Suggestion::of("@a"),
            Suggestion::of("@p"),
        ];

        #[derive(QueryData)]
        struct PlayerQuery {
            identity: &'static PlayerIdentity,
        }

        let mut query = ctx.world.query::<PlayerQuery>();
        
        for player in query.iter(ctx.world) {
            suggestions.push(Suggestion::of(&player.identity.username));
        }

        suggestions
            .into_iter()
            .filter(|sug| {
                sug.content
                    .to_lowercase()
                    .starts_with(&partial.to_lowercase())
            })
            .collect()
    }
}
