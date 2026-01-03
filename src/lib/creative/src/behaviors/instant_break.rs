//! Instant block breaking behavior for creative mode.

use ferrumc_api::behavior::{BlockBehavior, BlockContext, Handling};

/// Behavior that allows instant block breaking in creative mode.
///
/// When attached globally (to "*"), this behavior checks if the player
/// breaking the block has the `CreativeMode` component and returns
/// a hardness of 0.0 for instant breaking.
pub struct InstantBreakBehavior;

impl BlockBehavior for InstantBreakBehavior {
    fn get_hardness(&self, ctx: &BlockContext) -> Option<f32> {
        // Check if the player breaking the block is in creative mode
        if ctx.is_player_creative() {
            // Return 0 hardness for instant breaking
            Some(0.0)
        } else {
            // Pass to next behavior in chain
            None
        }
    }

    fn get_drops(&self, ctx: &BlockContext) -> Option<Vec<ferrumc_api::behavior::ItemStack>> {
        // In creative mode, blocks don't drop items
        if ctx.is_player_creative() {
            Some(Vec::new()) // Empty drops
        } else {
            None // Let other behaviors handle drops
        }
    }

    fn on_broken(&self, ctx: &mut BlockContext) -> Handling {
        // In creative mode, breaking is instant - just pass through
        // The hardness of 0 handles the actual instant breaking
        if ctx.is_player_creative() {
            Handling::Pass // Allow break but other behaviors can still run
        } else {
            Handling::Pass
        }
    }
}
