# Task 007 — Camera Smoothing (Interpolazione Lerp)

> **ID**: `007`
> **Categoria**: Animazioni & Camera
> **Priorità**: 🟡 P2
> **Stima**: ~30min
> **Assegnato a**: *(non assegnato)*

---

## 🎯 Obiettivo

> ⚠️ **Nota preliminare**: leggendo `camera.rs` si scopre che il lerp è **già implementato**:
> ```rust
> let t = (5.0_f32 * time.delta_secs()).min(1.0_f32);
> camera_pos.translation.x = camera_pos.translation.x.lerp(player_pos.x, t);
> ```
> Il task va quindi verificato prima di tutto. Se lo smoothing è già percepibile in gameplay,
> potrebbe bastare solo tweakare il coefficiente `5.0`.

Verificare e raffinare il sistema di smoothing della camera. Se già funzionante, migliorare
il coefficiente di interpolazione per un feeling più naturale.

---

## 📋 Acceptance Criteria

- [ ] La camera segue il player con un ritardo visibile (non snap immediato)
- [ ] Il movimento è fluido, senza scatti
- [ ] La camera non "lag" troppo (massimo ~0.2s di ritardo percepibile)
- [ ] Durante il `CameraShake` lo smoothing non interferisce con l'animazione di shake
- [ ] `cargo check` e `cargo run` passano senza errori

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/camera.rs` | Contiene `follow_player` — da modificare |
| `src/animation.rs` | Contiene `animate_camera` (shake) — da controllare per interferenze |

---

## 🧩 Contesto Tecnico

### Implementazione attuale di `follow_player`

```rust
fn follow_player(
    time: Res<Time>,
    query_player: Query<(&Transform, &DrillState), With<Player>>,
    mut query_camera: Query<
        (&mut Transform, &Projection),
        (With<BlackQuartzCamera>, Without<Player>),
    >,
    world_grid: Res<WorldGrid>,
) {
    if let Ok((player_transform, _drill_state)) = query_player.single() {
        let player_pos = player_transform.translation;
        if let Ok((mut camera_pos, camera)) = query_camera.single_mut() {
            if let Projection::Orthographic(ortho) = camera {
                let camera_area = ortho.area;
                let t = (5.0_f32 * time.delta_secs()).min(1.0_f32);  // ← lerp factor

                if player_pos.x + camera_area.max.x <= world_grid.map_area.max.x
                    && player_pos.x + camera_area.min.x >= world_grid.map_area.min.x
                {
                    camera_pos.translation.x = camera_pos.translation.x.lerp(player_pos.x, t);
                }

                if player_pos.y + camera_area.max.y <= world_grid.map_area.max.y
                    && player_pos.y + camera_area.min.y >= world_grid.map_area.min.y
                {
                    camera_pos.translation.y = camera_pos.translation.y.lerp(player_pos.y, t);
                }
            }
        }
    }
}
```

Il lerp usa `FloatExt::lerp` da Bevy (`use bevy::prelude::FloatExt`).

### Calcolo del fattore `t`

Con `delta_secs ≈ 0.016` (60fps):
- `t = 5.0 * 0.016 = 0.08` → la camera si avvicina dell'8% alla posizione target ogni frame
- Questo produce uno smoothing abbastanza forte, con leggero ritardo

Per un ritardo minore (camera più reattiva): aumentare il moltiplicatore (es. `8.0`).
Per un ritardo maggiore (camera più "cinematica"): diminuirlo (es. `3.0`).

---

## 🔨 Implementazione Suggerita

### Step 1: Verificare visivamente

Avviare il gioco e muovere il player velocemente. La camera dovrebbe seguire con un
leggero ritardo fluido. Se è già soddisfacente, il task è praticamente completato.

### Step 2 (se necessario): Aggiungere diverso smoothing per X e Y

Una camera può seguire orizzontalmente più velocemente che verticalmente (o viceversa)
per un feeling più naturale durante lo scavo verso il basso:

```rust
let t_x = (8.0_f32 * time.delta_secs()).min(1.0_f32);  // più reattivo
let t_y = (4.0_f32 * time.delta_secs()).min(1.0_f32);  // più morbido

camera_pos.translation.x = camera_pos.translation.x.lerp(player_pos.x, t_x);
camera_pos.translation.y = camera_pos.translation.y.lerp(player_pos.y, t_y);
```

### Step 3 (opzionale): Dead zone

Una dead zone permette alla camera di non muoversi finché il player non è uscito
da una certa area centrata sulla camera:

```rust
let threshold = 32.0; // pari a TILE_SIZE
let delta_x = (player_pos.x - camera_pos.translation.x).abs();
if delta_x > threshold {
    camera_pos.translation.x = camera_pos.translation.x.lerp(player_pos.x, t);
}
```

---

## ⚠️ Vincoli e Attenzioni

- Bevy 0.16: usare `single()` / `single_mut()`
- Verificare che il `CameraShake` in `animation.rs` non venga sovrascritto da questo sistema:
  `follow_player` modifica `camera_pos.translation`, `animate_camera` lo modifica anche lui.
  Se entrambi girano nello stesso frame, potrebbero interferire. Verificare l'ordine dei sistemi.
- Non modificare `animate_camera` — solo `follow_player`

---

## 🔗 Dipendenze

- Dipende da: nessuno
- Blocca: nessuno
- Correlato a: `animate_camera` in `animation.rs` — verificare compatibilità
