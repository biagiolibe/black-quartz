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

Components: `Health`, `Fuel`, `Inventory`, `Currency`, `DrillState` (Idle/Flying/Drilling/Falling), `PlayerAttributes` (drill_power, armor_resistance=0.0, ground_speed_factor, flying_speed_factor, fuel_efficiency).
Physics: `RigidBody::Dynamic`, capsule collider, rotation locked, Rapier raycasts for falling detection.

## Module structure

`main.rs` defines an inline `prelude` module with wildcard re-exports from all submodules. All files use `use crate::prelude::*` to access cross-module types.

**Submodules**:
- `src/map/`: `components.rs` (Tile, WorldGrid, constants, helper functions), `generation.rs` (procedural generation), `fov.rs` (field of view algorithm)
- `src/player/`: `components.rs` (Health, Fuel, Inventory, Currency, PlayerAttributes, DrillState), `movement.rs` (keyboard input, velocity), `drilling.rs` (drill system, collision, impact)
- `src/animation.rs` — sprite animation and camera shake
- `src/camera.rs` — camera follow with lerp smoothing
- `src/hud.rs` — UI health/fuel/inventory/depth display
- `src/menu.rs` — start/pause/game-over menus
- `src/resource.rs` — asset loading and TextureAtlas definitions
- `src/world_base.rs` — base entity and collision trigger

---

## Bevy 0.16 API conventions

- **Queries**: `query.single()` / `query.single_mut()` — NOT `get_single()` (renamed in 0.16)
- **Events**: `events.write(event)` — NOT `events.send()` (renamed in 0.16)
- **Reactivity**: `Changed<T>` in query filters for systems that only run when T was modified
- **UI text**: Use `TextUiWriter` to modify text content and spans in HUD systems
- **TextureAtlas**: Use `ImageNode::from_atlas_image(texture, atlas)` for sprite selection

---

## Development workflow

The project uses the **MERIDIAN** system (`MERIDIAN/WORKFLOW_GUIDE.md`) for structured task management:

| File | Purpose |
|------|---------|
| `DEVELOPMENT_PLAN.md` | Feature backlog (proposals → approved → completed) organized by category |
| `tasks/QUEUE.md` | Execution queue with priority levels (🔴 P1 / 🟡 P2 / 🟢 P3) |
| `tasks/NNN-name.md` | Detailed task briefings with context for agent delegation |
| `tasks/_TEMPLATE.md` | Template for creating new task files |
| `tasks/done/` | Archive of completed task files |

When starting a new feature: (1) add to `DEVELOPMENT_PLAN.md`, (2) create a task file if complex, (3) add to `QUEUE.md` with priority, (4) delegate to an agent via task file.
