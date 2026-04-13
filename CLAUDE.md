# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo run            # Run the game (uses dynamic_linking in dev)
cargo check          # Type-check without building
cargo build          # Build debug
cargo build --release
```

## Dependencies

- **bevy 0.16** with `dynamic_linking` feature
- **bevy_rapier2d 0.31** — physics (must stay in sync with bevy version; wrong version causes duplicate bevy in dep graph)
- **noise 0.9** — Perlin noise for ore distribution
- **rand 0.8** — map generation randomness

## Architecture

### Plugin tree

`GamePlugin` (`game.rs`) coordinates all sub-plugins:
- `ResourcePlugin` — asset loading, `LoadingProgress`, FOV material shader
- `MenuPlugin` — start/game-over/inventory/base menus, `MenuState` sub-state
- `MapPlugin` — procedural world generation, FOV algorithm, tile entities
- `PlayerPlugin` — player spawn, movement, drilling, collision/death
- `WorldBasePlugin` — base entity, collision trigger for base menu
- `CameraPlugin` — camera follow with boundary clamping
- `HUDPlugin` — health/fuel/inventory/depth UI
- `GameAnimationPlugin` — drill bob, camera shake on impact

### State machine

`GameState`: `Loading → MainMenu → Rendering → Playing ⇄ Menu → GameOver`

`MenuState` (sub-state active while in `Menu`): `None | Start | GameOver | Settings | Inventory | WorldBase`

### System execution order

Systems run in chained `GameSystems` sets on `Update`:
`Loading → Rendering → Movement → Camera → Physics → Collision → Animation → Ui`

> **Known issue**: chaining all sets in `Update` can conflict with Rapier's internal scheduling. The planned fix is to migrate physics-dependent systems to `FixedUpdate`.

### Map

100 × 500 grid, 32 px/tile. Generation: stochastic fill → cellular automata (4 passes) → Perlin noise ore distribution. FOV uses BFS with occlusion; revealed tiles persist (roguelike visibility).

### Player

Components: `Health`, `Fuel`, `Inventory`, `Currency`, `DrillState` (Idle/Flying/Drilling/Falling).
Physics: `RigidBody::Dynamic`, capsule collider, rotation locked, Rapier raycasts for falling detection.

## WIP: module refactoring (branch `update-bevy`)

`src/map.rs` and `src/player.rs` were **deleted** and replaced with empty `src/map/` and `src/player/` directories as part of a planned split into submodules. The directories currently have no `mod.rs` — **the project does not compile** until they are filled.

Planned submodules:
- `map/`: `mod.rs`, `components.rs`, `generation.rs`, `fov.rs`, `events.rs`
- `player/`: `mod.rs`, `components.rs`, `movement.rs`, `drilling.rs`

See `implementation_plan.md` for the full refactoring spec (written in Italian).
