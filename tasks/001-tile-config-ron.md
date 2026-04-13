# Task 001 — Caricare le proprietà dei tile da file `.ron`

> **ID**: `001`
> **Categoria**: Mappa & Generazione
> **Priorità**: Media
> **Stima**: ~2h
> **Assegnato a**: *(non assegnato)*

---

## 🎯 Obiettivo

Attualmente le proprietà dei tile (integrità, durezza, sprite index, loot) sono hardcoded nella funzione `get_tile_to_render` in `src/map/generation.rs`.

L'obiettivo è spostare questa configurazione in un file esterno `assets/tiles.ron` caricato come asset Bevy, in modo che le proprietà possano essere modificate senza ricompilare.

---

## 📋 Acceptance Criteria

Un task è considerato **completato** quando:

- [ ] Esiste un file `assets/tiles.ron` con le proprietà di tutti i `TileType`
- [ ] Esiste una struct `TileConfig` (o simile) che Bevy carica come asset
- [ ] `get_tile_to_render` legge i dati da `TileConfig` invece che da costanti hardcoded
- [ ] `cargo check` e `cargo run` passano senza errori

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/map/generation.rs` | Contiene `get_tile_to_render`, da modificare |
| `src/map/components.rs` | Contiene `TileType`, `Tile`, `Drilling` — da consultare |
| `src/resource.rs` | Contiene il sistema di caricamento asset (`GameAssets`) — da estendere |
| `assets/tiles.ron` | **Da creare** — configurazione dei tile in formato RON |
| `Cargo.toml` | Verificare che `bevy` abbia la feature `ron` abilitata (di solito inclusa di default) |

---

## 🧩 Contesto Tecnico

### TileType attuale

```rust
// src/map/components.rs
pub enum TileType { Empty, Solid, Sand, Iron, Copper, Gold, Crystal }
```

### Struttura `Tile` e `Drilling`

```rust
pub struct Tile {
    pub tile_type: TileType,
    pub drilling: Drilling,
}

pub struct Drilling {
    pub integrity: f32,
    pub hardness: f32,
}
```

### Funzione da refactorare

```rust
// src/map/generation.rs
fn get_tile_to_render(tile_type: &TileType) -> (Tile, usize) {
    match tile_type {
        Solid => (Tile { tile_type: Solid, drilling: Drilling { integrity: 1.0, hardness: 0.5 } }, 0),
        Sand  => (Tile { ... }, 1),
        // etc.
    }
}
```

L'indice `usize` è l'indice nel TextureAtlas dello sprite.

### Sistema asset attuale

`GameAssets` in `src/resource.rs` carica texture e font. Va esteso per caricare anche `TileConfigAsset`.

---

## 🔨 Implementazione Suggerita

### 1. Definire la struct di config

```rust
// src/map/components.rs (o nuovo file src/map/config.rs)
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize)]
pub struct TileConfigAsset {
    pub tiles: Vec<TileEntry>,
}

#[derive(Deserialize)]
pub struct TileEntry {
    pub tile_type: String,  // "Solid", "Sand", etc.
    pub sprite_index: usize,
    pub integrity: f32,
    pub hardness: f32,
}
```

### 2. Creare `assets/tiles.ron`

```ron
(
  tiles: [
    ( tile_type: "Solid",   sprite_index: 0,  integrity: 1.0, hardness: 0.5  ),
    ( tile_type: "Sand",    sprite_index: 1,  integrity: 0.1, hardness: 0.05 ),
    ( tile_type: "Iron",    sprite_index: 2,  integrity: 0.6, hardness: 0.3  ),
    ( tile_type: "Copper",  sprite_index: 3,  integrity: 0.4, hardness: 0.2  ),
    ( tile_type: "Gold",    sprite_index: 4,  integrity: 0.5, hardness: 0.2  ),
    ( tile_type: "Crystal", sprite_index: 5,  integrity: 0.3, hardness: 0.1  ),
  ]
)
```

### 3. Registrare l'asset loader

```rust
// src/resource.rs o src/map/mod.rs
app.init_asset::<TileConfigAsset>()
   .init_asset_loader::<bevy::asset::ron::RonAssetPlugin<TileConfigAsset>>();
```

### 4. Aggiungere l'handle a `GameAssets`

```rust
pub struct GameAssets {
    // ...esistenti...
    pub tile_config: Handle<TileConfigAsset>,
}
```

### 5. Usare la config in `get_tile_to_render`

```rust
fn get_tile_to_render(tile_type: &TileType, config: &TileConfigAsset) -> (Tile, usize) {
    let entry = config.tiles.iter()
        .find(|e| e.tile_type == tile_type.to_string())
        .expect("tile config missing");
    (
        Tile { tile_type: *tile_type, drilling: Drilling { integrity: entry.integrity, hardness: entry.hardness } },
        entry.sprite_index,
    )
}
```

---

## ⚠️ Vincoli e Attenzioni

- Usare **Bevy 0.16** e le sue API (`single()` non `get_single()`)
- Non usare `dynamic_linking` per i test — verificare `release` build alla fine
- Il file `.ron` deve essere incluso nella cartella `assets/` perché Bevy lo carichi correttamente a runtime
- Aggiungere `serde` come dipendenza se non già presente (`serde = { version = "1", features = ["derive"] }`)

---

## 🔗 Dipendenze

- Dipende da: nessuno
- Blocca: nessuno
