# FerrumC Development Guide for AI Agents

## 📚 Documentation

**For comprehensive architectural overview, see [ARCHITECTURE.md](ARCHITECTURE.md)**

The ARCHITECTURE.md file contains in-depth documentation about:
- Complete project structure and workspace layout
- Detailed system documentation (ECS, networking, world generation, storage, etc.)
- Data flow diagrams and architecture patterns
- Performance optimizations and design decisions
- Development reference and code examples

**For plugin development (implementing Minecraft features), see:**
- **[NEXT_STEPS.md](NEXT_STEPS.md)** - ⭐ START HERE - Complete guide for next developer
- **[PLUGIN_ARCHITECTURE.md](PLUGIN_ARCHITECTURE.md)** - Comprehensive plugin architecture (NEW!)
- [PLUGIN_VS_CORE.md](PLUGIN_VS_CORE.md) - Core vs APIs vs Plugins (critical!)
- [PLUGIN_QUICKSTART.md](PLUGIN_QUICKSTART.md) - Quick start guide for creating plugins

**For feature tracking, see [FEATURES.md](FEATURES.md)**

This guide (AGENTS.md) focuses on quick-reference commands and coding conventions for AI agents.

## Commands
- **Build**: `cargo build --release` (dev: `cargo build`)
- **Check**: `cargo check` (faster than build, for type checking)
- **Test all**: `cargo test`
- **Test single**: `cargo test <test_name>` (e.g., `cargo test test_to_string`)
- **Test package**: `cargo test -p <package_name>` (e.g., `cargo test -p ferrumc-text`)
- **Lint**: `cargo clippy --all-targets -- -Dwarnings` (CI enforced)
- **Format**: `cargo fmt --all -- --check` (to check) or `cargo fmt --all` (to fix)
- **Audit**: `cargo audit` (security check, CI enforced)

## Architecture
FerrumC is a Minecraft 1.21.8 server in Rust using a workspace layout with 40+ crates:
- **src/bin**: Main binary (bootstrap only, ~500 lines)
- **src/lib**: Core libraries organized by three layers:
  - **Core Infrastructure** (handles all I/O):
    - **net**: Networking (TCP, packets, encryption)
    - **world/world_gen**: World storage and terrain generation
    - **storage**: K/V database backend (uses `heed`)
    - **core**: ECS components and base types
    - **adapters**: NBT, Anvil parsers
    - **utils**: General utilities, logging, profiling
  - **Domain APIs** (bridge between core and plugins):
    - **apis/animation-api**: Animation events, traits, types
    - **apis/block-api**: Block events, traits, types
    - **apis/chat-api**: Chat events, traits, types
    - More domain APIs as needed
  - **Plugins** (gameplay logic only):
    - **plugins/core/animations**: Animation logic
    - **plugins/core/blocks**: Block placement/breaking rules
    - **plugins/core/chat**: Chat formatting
    - More plugins as needed

**Key principle:** Core handles I/O (packets, database), APIs define contracts, Plugins implement game logic.

## Code Style & Conventions
- **Error Handling**: Each crate has its own `thiserror`-based error types in `errors.rs`. Use `expect()` with detailed messages instead of `unwrap()`. Include `error!()` logs with `expect()`.
- **Dependencies**: Add all deps to workspace `Cargo.toml`, never to individual crate manifests
- **Imports**: Use `crate::*` for internal imports in tests (see src/lib/text/src/tests.rs)
- **Testing**: Mark data-dump/generation-only tests with `#[ignore]`. Use `#[cfg(test)]` for test-only code. Add tests for new features and bug fixes.
- **Lints**: Use `#[expect(lint)]` instead of `#[allow(lint)]` for intentional lint suppressions
- **Cloning**: Avoid `.clone()` except where necessary (startup, config loading is acceptable)
- **Paths**: Never use absolute paths. Use `get_root_path()` function instead of `../` chains
- **Documentation**: Add doc strings to public items. See https://doc.rust-lang.org/nightly/rustdoc/
- **Unsafe**: Allowed but must be well-documented with clear safety justifications
- **Formatting**: Auto-format on save recommended. Clippy and rustfmt are strictly enforced in CI
