# Task 008 — Animazione Distruzione Blocco (Flash Visivo)

> **ID**: `008`
> **Categoria**: Animazioni & Camera
> **Priorità**: 🟡 P2
> **Stima**: ~1h
> **Assegnato a**: *(non assegnato)*

---

## 🎯 Obiettivo

Quando un blocco viene distrutto (integrità ≤ 0), attualmente viene semplicemente `despawn`-ato
in modo istantaneo. L'obiettivo è aggiungere un breve effetto visivo prima che sparisca:
un flash di colore bianco/giallo per ~0.1-0.15 secondi.

Questa modifica migliora notevolmente il feedback visivo dello scavo.

---

## 📋 Acceptance Criteria

- [ ] Quando un blocco raggiunge integrità 0, emette un flash bianco per ~0.1s prima di sparire
- [ ] Il flash non blocca il gameplay (il blocco non blocca più il player durante il flash)
- [ ] `TileDestroyedEvent` viene emesso nello stesso momento attuale (non ritardato)
- [ ] Il despawn avviene dopo il flash (delay di 0.1-0.15s)
- [ ] `cargo check` e `cargo run` passano senza errori

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/player/drilling.rs` | Contiene `drill` — dove viene emesso `TileDestroyedEvent` |
| `src/map/generation.rs` | Contiene `handle_tile_destroyed` — dove avviene il despawn |
| `src/map/components.rs` | Contiene `TileDestroyedEvent`, `Tile` |
| `src/animation.rs` | Aggiungere qui il sistema di flash |

---

## 🧩 Contesto Tecnico

### Flusso attuale della distruzione

```
drill() → tile.integrity <= 0 → TileDestroyedEvent emesso
    → handle_tile_destroyed() → commands.entity(event.entity).despawn() (immediato)
```

### Flusso target

```
drill() → tile.integrity <= 0 → TileDestroyedEvent emesso
    → flash_tile() → applica flash color, inserisce DestroyTimer sul tile
    → (0.1s dopo) → handle_tile_destroyed() → despawn effettivo
```

### Componente marker per il flash

```rust
#[derive(Component)]
pub struct TileFlash {
    pub timer: Timer,
}
```

---

## 🔨 Implementazione Suggerita

### Approccio semplificato (consigliato)

Il modo più pulito è **non modificare `handle_tile_destroyed`** ma aggiungere un sistema
intermedio che intercetta `TileDestroyedEvent`, applica il flash, e poi triggera il despawn
con un componente timer.

### Step 1: Aggiungere `TileFlash` component

```rust
// src/map/components.rs
#[derive(Component)]
pub struct TileFlash {
    pub timer: Timer,
}
```

### Step 2: Sistema `start_tile_flash`

```rust
// src/animation.rs
fn start_tile_flash(
    mut events: EventReader<TileDestroyedEvent>,
    mut commands: Commands,
    mut query_tiles: Query<&mut Sprite, With<Tile>>,
) {
    for event in events.read() {
        if let Ok(mut sprite) = query_tiles.get_mut(event.entity) {
            sprite.color = Color::srgb(2.0, 2.0, 1.0);  // flash giallo brillante (HDR-like)
            commands.entity(event.entity).insert(TileFlash {
                timer: Timer::new(Duration::from_secs_f32(0.12), TimerMode::Once),
            });
        }
    }
}
```

### Step 3: Sistema `tick_tile_flash`

```rust
fn tick_tile_flash(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut TileFlash, &mut Sprite)>,
    mut world_grid: ResMut<WorldGrid>,
) {
    for (entity, mut flash, mut sprite) in query.iter_mut() {
        flash.timer.tick(time.delta());

        // Interpolazione dal giallo brillante al bianco durante il flash
        let progress = flash.timer.fraction();
        sprite.color = Color::srgb(
            2.0 - progress,
            2.0 - progress,
            1.0 + progress * 0.5,
        );

        if flash.timer.finished() {
            commands.entity(entity).despawn();
            // Aggiorna la world grid
            // Nota: qui serve la posizione del tile. Aggiungere un campo `position` a TileFlash,
            // oppure leggere dal Transform.
        }
    }
}
```

### Problema: la world_grid va aggiornata al despawn

`handle_tile_destroyed` aggiorna `world_grid.grid` e `world_grid.tiles`.
Se il despawn è ritardato, bisogna garantire che la world_grid venga aggiornata
**subito** (al momento dell'evento), non dopo il flash — altrimenti il player
potrebbe collegarsi a entità che non hanno ancora subito il despawn fisico ma la cui
posizione è già rimossa dalla grid.

**Soluzione**: aggiornare `world_grid` in `start_tile_flash` (o mantenere `handle_tile_destroyed`
per la grid) e fare solo il **despawn visivo** ritardato.

```rust
// In start_tile_flash: aggiorna subito la world_grid
world_grid.grid.remove(&event.position);
let grid_id = world_grid_position_to_idx(event.position);
world_grid.tiles[grid_id.1][grid_id.0] = TileType::Empty;
// NON fare commands.entity(event.entity).despawn() -- lo fa tick_tile_flash dopo il flash
```

### Step 4: Registrare i sistemi

```rust
// src/animation.rs, in GameAnimationPlugin::build
app.add_systems(Update, start_tile_flash.in_set(Animation))
   .add_systems(Update, tick_tile_flash.in_set(Animation));
```

### Step 5: Rimuovere o modificare `handle_tile_destroyed`

Se `start_tile_flash` gestisce il despawn ritardato, `handle_tile_destroyed` deve essere
rimosso o limitato all'aggiornamento della `WorldGrid` (senza despawn).

---

## ⚠️ Vincoli e Attenzioni

- Bevy 0.16: usare `single()` / `single_mut()`
- **Rischio race condition**: `TileDestroyedEvent` viene letto sia da `handle_tile_destroyed`
  che da `start_tile_flash`. Verificare che non ci sia doppia lettura (in Bevy 0.16 gli eventi
  vengono consumati alla lettura con `EventReader`).
- Il Collider del tile deve essere rimosso subito (non dopo il flash), altrimenti il player
  continua a collidere con un tile "fantasma". Usare `commands.entity(entity).remove::<Collider>()`
  in `start_tile_flash`.
- Usare `Duration` da `std::time::Duration`

---

## 🔗 Dipendenze

- Dipende da: nessuno
- Blocca: nessuno
- Modifica: `handle_tile_destroyed` in `map/generation.rs` — coordinarsi se eseguito
  in parallelo con task 002
