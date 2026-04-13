# Task 005 — Visual Feedback: Lampeggio Salute/Carburante Bassa + Avviso HUD

> **ID**: `005`
> **Categoria**: Giocatore & HUD
> **Priorità**: 🔴 P1
> **Stima**: ~1.5h
> **Assegnato a**: *(non assegnato)*

---

## 🎯 Obiettivo

Dare al giocatore un feedback visivo chiaro quando le risorse critiche sono basse:

1. **Lampeggio del player sprite** quando la salute è ≤ 30%
2. **Lampeggio del player sprite** (diverso) quando il carburante è ≤ 20%
3. **Testo HUD** del carburante che cambia colore quando scende sotto il 20%

Queste feature migliorano notevolmente la leggibilità del gameplay (il giocatore sa quando
è in pericolo senza guardare continuamente l'HUD).

---

## 📋 Acceptance Criteria

- [ ] Quando `health.current / health.max <= 0.3`, il player sprite lampeggia in rosso
- [ ] Quando `fuel.current / fuel.max <= 0.2`, il player sprite lampeggia in arancione
- [ ] Il lampeggio è un ciclo di colore (es. bianco → colorato → bianco ogni 0.5s)
- [ ] Il testo del carburante nell'HUD diventa rosso quando sotto il 20%
- [ ] Quando le risorse tornano sopra la soglia, il lampeggio si ferma
- [ ] `cargo check` e `cargo run` passano senza errori

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/player/mod.rs` | Contiene `update_player_on_state_changes` — aggiungere qui o sistema separato |
| `src/player/components.rs` | Contiene `Health`, `Fuel`, `Player` |
| `src/animation.rs` | Contiene il loop di animazione — aggiungere `blink_animation` qui |
| `src/hud.rs` | Contiene `update_hud` e `HudFuelText` — modificare colore testo |
| `src/game.rs` | Contiene `GameSystems` — per registrare il sistema nel set corretto (`Animation`) |

---

## 🧩 Contesto Tecnico

### Sprite del player

Il player ha un `Sprite` component con un campo `.color: Color`.
Di default è `Color::WHITE`. Per fare lampeggio si può:
- Modificare `.color` ciclicamente tra `Color::WHITE` e il colore target ogni N secondi
- Usare un timer dedicato

### Sistema di animazione esistente

In `src/animation.rs` esiste già:
- `animate_drilling`: anima il TextureAtlas del player
- `animate_camera`: muove la camera con shake
- `handle_camera_shake`: gestisce l'effetto shake tramite `CameraShake` component

Il pattern da seguire è lo stesso: aggiungere un sistema `blink_player` che modifica
`Sprite.color` in base al rapporto `health/max` e `fuel/max`.

### HUD esistente

In `hud.rs`, il testo del carburante è:
```rust
hud_children.spawn((
    Text::new("Fuel: "),
    font_style.clone(),
    TextColor(Color::WHITE),  // ← da cambiare dinamicamente
    TextLayout::new_with_justify(Left),
    HudFuelText,
));
```

In `update_hud`:
```rust
if let Ok(fuel_text_entity) = hud_fuel_text.single() {
    let fuel = player_stats.2;
    *text_writer.text(fuel_text_entity, 1) = format!("{}", fuel.current.trunc());
}
```

`TextUiWriter` permette di aggiornare il testo ma non il colore direttamente.
Per il colore usare la query su `TextColor` component.

---

## 🔨 Implementazione Suggerita

### Step 1: Aggiungere un `BlinkTimer` component

```rust
// src/animation.rs (o src/player/components.rs)
#[derive(Component)]
pub struct BlinkTimer {
    pub timer: Timer,
    pub phase: bool,  // true = colore target, false = bianco
}
```

### Step 2: Sistema `blink_player`

```rust
// src/animation.rs
fn blink_player(
    time: Res<Time>,
    mut player_query: Query<(&Health, &Fuel, &mut Sprite, Option<&mut BlinkTimer>), With<Player>>,
    mut commands: Commands,
) {
    if let Ok((health, fuel, mut sprite, blink_timer)) = player_query.single_mut() {
        let health_ratio = health.current / health.max;
        let fuel_ratio = fuel.current / fuel.max;

        let blink_color = if health_ratio <= 0.3 {
            Some(Color::srgb(1.0, 0.2, 0.2))  // rosso
        } else if fuel_ratio <= 0.2 {
            Some(Color::srgb(1.0, 0.5, 0.0))  // arancione
        } else {
            None  // nessun lampeggio
        };

        if let Some(color) = blink_color {
            if let Some(mut timer) = blink_timer {
                timer.timer.tick(time.delta());
                if timer.timer.just_finished() {
                    timer.phase = !timer.phase;
                    timer.timer.reset();
                }
                sprite.color = if timer.phase { color } else { Color::WHITE };
            } else {
                // Inserisce il timer al primo frame in cui si attiva
                let entity = player_query.single().unwrap(); // si risolve diversamente
                // Nota: non si può usare commands qui se si è già in borrow su player_query
                // Soluzione: aggiungere BlinkTimer al player in spawn e disabilitarlo/abilitarlo
            }
        } else {
            // Risorse ok: rimuovi lampeggio
            sprite.color = Color::WHITE;
        }
    }
}
```

> ⚠️ **Problema con Commands + Query**: non si può usare `commands.entity()` e fare borrow
> della stessa query contemporaneamente. La soluzione più semplice è **aggiungere `BlinkTimer`
> direttamente nel bundle di spawn del player** con timer inizialmente paused/inattivo,
> e abilitarlo/disabilitarlo invece di inserirlo/rimuoverlo.

### Alternativa più semplice (consigliata): nessun BlinkTimer

Usare solo `time.elapsed_secs()` per calcolare il lampeggio:

```rust
fn blink_player(
    time: Res<Time>,
    mut player_query: Query<(&Health, &Fuel, &mut Sprite), With<Player>>,
) {
    if let Ok((health, fuel, mut sprite)) = player_query.single_mut() {
        let health_ratio = health.current / health.max;
        let fuel_ratio = fuel.current / fuel.max;

        // Lampeggio basato su sin(time) — oscilla tra 0 e 1 ogni ~0.5s
        let blink = (time.elapsed_secs() * 6.0).sin() > 0.0;

        sprite.color = if health_ratio <= 0.3 && blink {
            Color::srgb(1.0, 0.3, 0.3)  // rosso
        } else if fuel_ratio <= 0.2 && blink {
            Color::srgb(1.0, 0.6, 0.0)  // arancione
        } else {
            Color::WHITE
        };
    }
}
```

### Step 3: Cambiare colore testo HUD carburante

```rust
// src/hud.rs, in update_hud — aggiungere query per TextColor
mut hud_fuel_color: Query<&mut TextColor, With<HudFuelText>>,

// poi in update_hud:
if let Ok(mut fuel_text_color) = hud_fuel_color.single_mut() {
    let fuel_ratio = fuel.current / fuel.max;
    fuel_text_color.0 = if fuel_ratio <= 0.2 {
        Color::srgb(1.0, 0.2, 0.0)
    } else {
        Color::WHITE
    };
}
```

### Step 4: Registrare il sistema

```rust
// src/animation.rs, in GameAnimationPlugin::build
app.add_systems(Update, blink_player.in_set(Animation));
```

---

## ⚠️ Vincoli e Attenzioni

- Bevy 0.16: usare `single()` / `single_mut()`
- Il sistema va in `GameSystems::Animation` (già ordinato nell'update chain)
- **Non** modificare `Sprite.color` nei sistemi di drilling/movement — solo in `blink_player`
- La soglia salute è `<= 0.3` (30%), la soglia carburante è `<= 0.2` (20%)
- Il lampeggio deve fermarsi immediatamente quando le risorse tornano sopra soglia

---

## 🔗 Dipendenze

- Dipende da: nessuno
- Blocca: nessuno
- Abbinato a: H3 (avviso HUD carburante 20%) — possono essere nello stesso PR
