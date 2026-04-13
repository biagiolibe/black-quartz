# Task 002 — Rigenerazione casuale della mappa a ogni nuova run

> **ID**: `002`
> **Categoria**: Mappa & Generazione
> **Priorità**: Alta
> **Stima**: ~1h
> **Assegnato a**: *(non assegnato)*

---

## 🎯 Obiettivo

Il gioco è strutturato come roguelike: ogni nuova partita deve generare una mappa diversa.
Attualmente la mappa viene inizializzata una volta sola (`initialize_world_grid`) durante lo stato
`GameState::Rendering`, ma se il giocatore sceglie "New Game" la mappa non viene rigenerata —
viene solo re-renderizzata (`render_map`) con i tile precedenti.

L'obiettivo è fare in modo che **ogni volta che si entra nello stato `GameState::Rendering`**,
la mappa venga rigenerata da zero con un seed casuale diverso.

---

## 📋 Acceptance Criteria

- [ ] Premendo "Start game" o "Restart game" la mappa è sempre diversa (seed casuale a ogni run)
- [ ] I `revealed_tiles` (FOV) vengono azzerati a ogni nuova run
- [ ] La `WorldGrid` viene reinizializzata prima del rendering
- [ ] Il player viene riposizionato correttamente alla partenza
- [ ] `cargo check` e `cargo run` passano senza errori

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/map/generation.rs` | Contiene `initialize_world_grid` e `render_map` — entrambe da modificare |
| `src/map/components.rs` | Contiene `WorldGrid` con `tiles`, `grid`, `revealed_tiles` |
| `src/map/mod.rs` | Registra i sistemi del modulo mappa |
| `src/resource.rs` | Contiene `LoadingProgress` — il flag `rendering_map` controlla il rendering |
| `src/menu.rs` | Il pulsante `NewGame` resetta i flag di `LoadingProgress` e porta a `GameState::Rendering` |
| `src/player/movement.rs` | Contiene `spawn_player` — va verificato il riposizionamento |

---

## 🧩 Contesto Tecnico

### Flusso attuale al "New Game"

In `menu.rs`, quando si preme "New Game":
```rust
NewGame => {
    loading_progress.rendering_map = false;
    loading_progress.spawning_player = false;
    loading_progress.spawning_base = false;
    next_state.set(GameState::Rendering);
    next_menu_state.set(MenuState::None);
}
```

Questo triggera `OnEnter(GameState::Rendering)` che esegue:
1. `initialize_world_grid` — ma solo se non c'è già una `WorldGrid` in memoria
2. `render_map` — despawna i tile esistenti e respawna i nuovi

### Problema

`initialize_world_grid` usa `commands.insert_resource(WorldGrid {...})`.
Se `WorldGrid` esiste già come risorsa (dalla partita precedente), il comportamento dipende
da come Bevy gestisce `insert_resource` su una risorsa già esistente (la sovrascrive, ma
solo se il sistema viene eseguito di nuovo).

Il punto critico è che `initialize_world_grid` è registrato `OnEnter(GameState::Rendering)`,
quindi **viene rieseguito** ad ogni re-entry nello stato — ma va verificato che:
1. I `revealed_tiles` siano azzerati (attualmente sono solo inizializzati con `HashSet::new()` nella prima run)
2. Il `HashMap` del `grid` sia azzerato
3. Il seed del Perlin Noise sia diverso (attualmente usa `rand::thread_rng().gen()` — già casuale ✅)

### WorldGrid

```rust
// src/map/components.rs
pub struct WorldGrid {
    pub grid: HashMap<(i32, i32), Entity>,       // mapping pos → entity tile
    pub revealed_tiles: HashSet<(i32, i32)>,     // FOV memory
    pub tiles: Vec<Vec<TileType>>,               // matrice dei tipi
    pub map_area: Rect,                          // bounds fisici
}
```

### render_map

```rust
// src/map/generation.rs — già fa il cleanup prima di rispawnare
pub fn render_map(...) {
    for tile_entity in tile_query.iter() {
        commands.entity(tile_entity).despawn();  // ✅ despawna i vecchi tile
    }
    // poi spawna i nuovi...
    world_grid.grid.insert((x, y), entity);     // ✅ reinserisce nella grid
}
```

---

## 🔨 Implementazione Suggerita

### Verifica: `initialize_world_grid` viene rieseguito?

Prima di fare modifiche, verificare se il sistema viene effettivamente rieseguito.
Aggiungere temporaneamente un `info!` e fare un test manuale con "Restart".

Se viene rieseguito correttamente, il problema si riduce solo ai `revealed_tiles`.

### Fix principale: azzerare `revealed_tiles` in `initialize_world_grid`

La funzione già crea `WorldGrid` da zero con `HashSet::new()`, quindi i `revealed_tiles`
sono già azzerati ad ogni chiamata. **Verificare se questo è sufficiente.**

### Fix aggiuntivo (se necessario): reset esplicito prima del rendering

Se `initialize_world_grid` non viene rieseguito (perché Bevy schedula `OnEnter` solo alla
prima entrata nello stato), aggiungere un sistema di reset esplicito:

```rust
// In map/mod.rs o map/generation.rs
pub fn reset_world_grid(mut commands: Commands) {
    // Rimuove la risorsa esistente, così initialize_world_grid la ricrea
    commands.remove_resource::<WorldGrid>();
}
```

E registrarlo **prima** di `initialize_world_grid`:

```rust
// In map/mod.rs
app.add_systems(
    OnEnter(GameState::Rendering),
    (reset_world_grid, initialize_world_grid, render_map, setup_borders)
        .chain()
        .in_set(GameSystems::Rendering),
);
```

### Verificare anche il riposizionamento del player

In `spawn_player` (in `src/player/movement.rs`), verificare che il player sia spawnato
sempre alla posizione iniziale (probabilmente `Vec3::new(0.0, 0.0, 1.0)` o simile).
Se il player viene rispawnato con `despawn` + nuovo `spawn`, è già corretto.

---

## ⚠️ Vincoli e Attenzioni

- Usare **Bevy 0.16** (`single()` non `get_single()`, `write()` non `send()` per EventWriter)
- Non modificare le costanti `GRID_WIDTH`, `GRID_HEIGHT`, `TILE_SIZE` — definite in `map/components.rs`
- Il seed del Perlin Noise in `distribute_materials` usa già `rand::thread_rng().gen()` — produce già mappe diverse senza modifiche
- Verificare che il FOV overlay venga azzerato visivamente (i tile dovrebbero tornare neri)
- Non dimenticare di verificare che `loading_progress.rendering_map` sia resettato prima di entrare in `Rendering` (già fatto in `menu.rs`)

---

## 🔗 Dipendenze

- Dipende da: nessuno
- Blocca: nessuno
- Correlato a: task 001 (tile config RON) — possono essere eseguiti in parallelo
