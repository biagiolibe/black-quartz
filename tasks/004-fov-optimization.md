# Task 004 — Ottimizzazione FOV: Skip se il Player non si è Mosso

> **ID**: `004`
> **Categoria**: Field of View
> **Priorità**: 🟢 P3
> **Stima**: ~30min
> **Assegnato a**: *(non assegnato)*

---

## 🎯 Obiettivo

Il sistema FOV (`update_fov` + `update_fov_overlay`) viene eseguito ogni frame in `FixedUpdate`.
Quando il player è fermo, il FOV viene ricalcolato inutilmente: le celle visibili non cambiano.

L'obiettivo è aggiungere un check sulla posizione del player: se non è cambiata rispetto al
frame precedente, il sistema skippa il ricalcolo.

---

## 📋 Acceptance Criteria

- [ ] Quando il player è fermo, `update_fov` non rielabora il BFS
- [ ] Quando il player si muove, il FOV si aggiorna normalmente
- [ ] Il flag `fov.dirty` funziona ancora correttamente
- [ ] Il log `info!("Foving ...")` viene rimosso
- [ ] `cargo check` e `cargo run` passano senza errori

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/map/fov.rs` | Contiene `update_fov` e `update_fov_overlay` — da modificare |
| `src/player/components.rs` | Contiene `FieldOfView` — da estendere con `last_position` |
| `src/map/mod.rs` | Registra i sistemi FOV |

---

## 🧩 Contesto Tecnico

### Sistema FOV attuale

```rust
// src/map/fov.rs
pub fn update_fov(
    mut player_query: Query<(&Transform, Mut<FieldOfView>), With<Player>>,
    world_grid: ResMut<WorldGrid>,
) {
    if let Ok((player_transform, mut fov)) = player_query.single_mut() {
        let player_pos = IVec2::from(world_to_grid_position(
            player_transform.translation.truncate(),
        ));

        // BFS eseguito sempre, anche se player_pos non è cambiata
        let mut queue = VecDeque::new();
        queue.push_back((player_pos, 0));
        // ...
        fov.dirty = true;  // sempre true
    }
}
```

### Componente `FieldOfView`

```rust
// src/player/components.rs
pub struct FieldOfView {
    pub visible_tiles: HashSet<(i32, i32)>,
    pub radius: i32,
    pub dirty: bool,
}
```

### Problema specifico

Il log `info!("Foving ...")` in `update_fov_overlay` viene stampato ogni volta che una cella
diventa visibile. Dal log di gioco si vede che viene emesso decine di volte per ogni spostamento,
il che è normale; ma il fatto che `dirty` sia sempre `true` significa che anche fermo il sistema
lavora inutilmente.

---

## 🔨 Implementazione Suggerita

### Step 1: Aggiungere `last_position` a `FieldOfView`

```rust
// src/player/components.rs
pub struct FieldOfView {
    pub visible_tiles: HashSet<(i32, i32)>,
    pub radius: i32,
    pub dirty: bool,
    pub last_position: Option<IVec2>,  // nuova
}

impl Default for FieldOfView {
    fn default() -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: 10,
            dirty: false,
            last_position: None,  // nuova
        }
    }
}
```

### Step 2: Aggiungere il check in `update_fov`

```rust
pub fn update_fov(
    mut player_query: Query<(&Transform, Mut<FieldOfView>), With<Player>>,
    world_grid: ResMut<WorldGrid>,
) {
    if let Ok((player_transform, mut fov)) = player_query.single_mut() {
        let player_pos = IVec2::from(world_to_grid_position(
            player_transform.translation.truncate(),
        ));

        // Skip se il player non si è mosso
        if fov.last_position == Some(player_pos) {
            return;
        }
        fov.last_position = Some(player_pos);

        // ... resto del BFS invariato ...
        fov.dirty = true;
    }
}
```

### Step 3: Rimuovere il log `info!("Foving ...")`

```rust
// src/map/fov.rs, in update_fov_overlay
// Rimuovere questa riga:
info!("Foving {}x{}", x, y);
```

---

## ⚠️ Vincoli e Attenzioni

- Bevy 0.16: usare `single_mut()` (non `get_single_mut()`)
- Non modificare la logica del BFS — solo aggiungere il check all'inizio
- `dirty` deve restare `false` quando il player è fermo (già gestito se si fa `return` prima)

---

## 🔗 Dipendenze

- Dipende da: nessuno
- Blocca: nessuno
- Può essere eseguito in parallelo con qualsiasi altro task
