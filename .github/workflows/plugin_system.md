# Plugin System CI Checklist

This checklist ensures the plugin system stays healthy.

## Build Checks

- [ ] `cargo check --package ferrumc-plugin-api` - Plugin API compiles
- [ ] `cargo check --package ferrumc-plugin-hello` - Example plugin compiles
- [ ] `cargo check --package ferrumc` - Server compiles with plugins

## Test Checks

- [ ] `cargo test --package ferrumc-plugin-api` - Plugin API tests pass
- [ ] `cargo test --package ferrumc-plugin-hello` - Example plugin tests pass

## Lint Checks

- [ ] `cargo clippy --package ferrumc-plugin-api -- -Dwarnings` - No clippy warnings
- [ ] `cargo clippy --package ferrumc-plugin-hello -- -Dwarnings` - No clippy warnings

## Documentation

- [ ] `cargo doc --package ferrumc-plugin-api` - Docs build successfully
- [ ] All public APIs have doc comments
- [ ] Examples compile (marked with `no_run` or `ignore`)

## Integration

- [ ] Server starts successfully with plugin system enabled
- [ ] Plugin initialization logs appear
- [ ] No runtime errors related to plugins
