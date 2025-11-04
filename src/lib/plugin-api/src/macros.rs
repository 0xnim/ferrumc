//! Helper macros for plugin development

/// Register multiple events at once
///
/// # Example
///
/// ```rust,no_run
/// use ferrumc_plugin_api::register_events;
/// # use ferrumc_plugin_api::PluginContext;
/// # use bevy_ecs::prelude::Event;
/// # #[derive(Event)] struct EventA;
/// # #[derive(Event)] struct EventB;
/// # fn example(ctx: &mut PluginContext) {
/// register_events!(ctx,
///     EventA,
///     EventB,
/// );
/// # }
/// ```
#[macro_export]
macro_rules! register_events {
    ($ctx:expr, $($event:ty),* $(,)?) => {
        $(
            $ctx.register_event::<$event>();
        )*
    };
}
