# Task 009 — Ottimizzazione Rendering Tile

> **ID**: `009`
> **Categoria**: Mappa & Generazione
> **Priorità**: 🟢 P3
> **Stima**: ~1h
> **Assegnato a**: *(non assegnato)*

---

## 🎯 Obiettivo

Analizzare e ottimizzare il sistema di rendering dei tile per ridurre lavoro non necessario.

Il problema da investigare: esistono sistemi che iterano su tutti i tile ogni frame?
In una mappa di `100 × 500 = 50.000 tile`, qualsiasi iterazione non necessaria è costosa.

---

## 📋 Acceptance Criteria

- [ ] Identificare tutti i sistemi che iterano su tile ogni frame
- [ ] Ridurre o eliminare le iterazioni non necessarie
- [ ] Verificare che il FPS non degradi con la mappa completa
- [ ] `cargo check` e `cargo run` passano senza errori

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/map/fov.rs` | `update_fov_overlay` itera su `fov.visible_tiles` — verificare frequenza |
| `src/map/generation.rs` | `render_map` respawna tutti i tile — chiamata solo a inizio run |
| `src/hud.rs` | `update_hud` itera su query player — 1 solo elemento, ok |
| `src/player/drilling.rs` | `drill` usa query su singolo entity `world_grid.grid.get()` — ok |

---

## 🧩 Contesto Tecnico

### Sistemi che iterano sui tile

#### `update_fov_overlay` — potenzialmente costoso

```rust
pub fn update_fov_overlay(
    mut fov_query: Query<&mut FieldOfView, With<Player>>,
    mut query_tiles: Query<(&mut Sprite, &Tile), With<Tile>>,  // ← tutti i tile
    mut world_grid: ResMut<WorldGrid>,
) {
    if let Ok(mut fov) = fov_query.single_mut() {
        if fov.dirty {  // ← protetto da flag dirty, ok se task 004 è fatto prima
            fov.visible_tiles.iter().for_each(|(x, y)| {
                if !world_grid.revealed_tiles.contains(&(*x, *y)) {
                    if let Some(entity) = world_grid.grid.get(&(*x, *y)) {
                        let (mut sprite, tile) = query_tiles.get_mut(*entity).unwrap();
                        // aggiorna colore
                    }
                }
            });
        }
    }
}
```

La query `Query<(&mut Sprite, &Tile), With<Tile>>` include tutti i tile, ma l'accesso
effettivo è solo per entity specifiche tramite `query_tiles.get_mut(*entity)`.
Bevy ottimizza questo — non è un loop su tutti i tile, ma una lookup per entity.
**Probabilmente non è un problema reale.**

#### `collision_detection` in `drilling.rs`

```rust
tiles: Query<&Transform, With<Tile>>,
```

Questa query include tutti i tile ma viene usata solo per lookup entity-specifica.
Bevy non itera su tutti automaticamente. **Probabilmente non è un problema.**

### Vero potenziale problema: `update_fov` ricalcola il BFS ogni frame

Vedi task 004 — questo è il vero collo di bottiglia. Se task 004 è già completato,
il BFS viene skippato quando il player è fermo, risolvendo il problema principale.

---

## 🔨 Implementazione Suggerita

### Step 1: Profilazione (prima di ottimizzare)

Aggiungere al `GamePlugin` il plugin di diagnostica:

```rust
// src/game.rs
#[cfg(debug_assertions)]
{
    use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
    app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
}
```

Questo era già presente in `game.rs` ma commentato — decommentarlo temporaneamente e
verificare il FPS durante il gioco con la mappa completa.

### Step 2: Verificare se il problema esiste

Se FPS > 45 con la mappa completa → nessun problema reale, il task è completato.
Se FPS < 30 → procedere con l'analisi.

### Step 3 (se necessario): Cull dei tile non visibili

Se l'overhead è reale, considerare di disabilitare la visibilità (`Visibility::Hidden`)
dei tile molto lontani dal player (oltre 2x il viewport). Questo riduce il lavoro del renderer.

```rust
// Sistema di culling — eseguito ogni N frame, non ogni frame
fn cull_distant_tiles(
    player_query: Query<&Transform, With<Player>>,
    mut tile_query: Query<(&Transform, &mut Visibility), With<Tile>>,
    world_grid: Res<WorldGrid>,
) {
    // ...solo se FPS è un problema
}
```

---

## ⚠️ Vincoli e Attenzioni

- **Non ottimizzare prematuramente**: fare prima la profilazione
- Il plugin di diagnostica va rimosso (o lasciato solo in `#[cfg(debug_assertions)]`) prima di release
- Task 004 (FOV optimization) va fatto prima — risolve il caso più probabile

---

## 🔗 Dipendenze

- Dipende da: task 004 (FOV optimization) — fatto prima riduce la necessità di questo task
- Blocca: nessuno
- Può essere eseguito in parallelo con qualsiasi altro task
