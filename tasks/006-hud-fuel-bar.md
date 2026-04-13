# Task 006 — HUD: Barra Visiva Carburante (Progress Bar)

> **ID**: `006`
> **Categoria**: HUD & Interfaccia
> **Priorità**: 🟡 P2
> **Stima**: ~30min
> **Assegnato a**: *(non assegnato)*

---

## 🎯 Obiettivo

> ⚠️ **Nota preliminare**: leggendo `hud.rs` si scopre che la **fuel bar esiste già** nel codice
> (componente `HudFuelBar`, nodo figlio con `BackgroundColor` arancione).
> Il task va quindi verificato prima di tutto: se la bar è già funzionante visivamente,
> potrebbe bastare solo rifinirla (dimensioni, posizione, colore condizionale).

Verificare e completare la barra visiva del carburante nell'HUD:
- Se già funzionante: rifinire il layout (dimensione, posizione relativa all'icona salute)
- Se non visibile: debuggare perché non appare e correggerla

---

## 📋 Acceptance Criteria

- [ ] La barra carburante è visibile nell'HUD durante il gameplay
- [ ] La larghezza della barra riflette il rapporto `fuel.current / fuel.max`
- [ ] La barra si riduce man mano che il carburante scende
- [ ] La barra si ricarica dopo il rifornimento alla base
- [ ] Il colore cambia (es. arancione → rosso) quando il carburante è sotto il 20%
- [ ] `cargo check` e `cargo run` passano senza errori

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/hud.rs` | Contiene `init_hud`, `update_hud`, `HudFuelBar` — principale |
| `src/player/components.rs` | Contiene `Fuel` — da consultare |

---

## 🧩 Contesto Tecnico

### Stato attuale (già implementato in hud.rs)

```rust
// init_hud: la struttura già esiste
hud_children
    .spawn((
        Node {
            width: Val::Px(100.0),
            height: Val::Px(16.0),
            margin: UiRect::left(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),  // sfondo grigio scuro
    ))
    .with_child((
        Node {
            width: Val::Percent(100.0),  // inizia piena
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(1.0, 0.6, 0.0)),  // arancione
        HudFuelBar,
    ));

// update_hud: già aggiornata
if let Ok(mut fuel_bar_node) = hud_fuel_bar.single_mut() {
    let fuel = player_stats.2;
    let fuel_percentage = (fuel.current / fuel.max * 100.0).clamp(0.0, 100.0);
    fuel_bar_node.width = Val::Percent(fuel_percentage);
}
```

### Cosa manca

1. **Colore condizionale**: quando carburante < 20%, il colore dovrebbe cambiare a rosso
2. **Layout**: la barra è posizionata in `FlexDirection::Row` accanto agli altri elementi,
   ma potrebbe non avere le dimensioni giuste visivamente
3. **Etichetta**: non c'è testo sopra/sotto la barra che dica "FUEL"

---

## 🔨 Implementazione Suggerita

### Step 1: Verificare la visibilità

Avviare il gioco e controllare se la barra appare nell'HUD in alto a sinistra.
Se non appare, probabilmente il problema è nel layout Node (dimensioni o posizione).

### Step 2: Aggiungere colore condizionale

```rust
// src/hud.rs, in update_hud — aggiungere query
mut hud_fuel_bar_bg: Query<&mut BackgroundColor, With<HudFuelBar>>,

// poi in update_hud
if let Ok(mut fuel_bar_node) = hud_fuel_bar.single_mut() {
    let fuel = player_stats.2;
    let fuel_percentage = (fuel.current / fuel.max * 100.0).clamp(0.0, 100.0);
    fuel_bar_node.width = Val::Percent(fuel_percentage);

    // Aggiungere colore condizionale
    if let Ok(mut bar_color) = hud_fuel_bar_bg.single_mut() {
        bar_color.0 = if fuel_percentage <= 20.0 {
            Color::srgb(1.0, 0.1, 0.1)  // rosso
        } else {
            Color::srgb(1.0, 0.6, 0.0)  // arancione
        };
    }
}
```

### Step 3 (opzionale): Migliorare il layout

Se la barra è troppo piccola o mal posizionata, aggiustare in `init_hud`:
```rust
// Aumentare dimensioni per maggiore visibilità
Node {
    width: Val::Px(150.0),   // era 100
    height: Val::Px(20.0),   // era 16
    margin: UiRect::left(Val::Px(10.0)),
    ..default()
}
```

---

## ⚠️ Vincoli e Attenzioni

- Bevy 0.16: usare `single_mut()` (non `get_single_mut()`)
- La query `hud_fuel_bar` aggiorna il `Node` (dimensione), `hud_fuel_bar_bg` aggiorna il `BackgroundColor` — sono due query separate perché è un nodo figlio
- Non modificare `init_hud` se non strettamente necessario — la struttura è già corretta

---

## 🔗 Dipendenze

- Dipende da: nessuno (la struttura base è già implementata)
- Blocca: nessuno
- Correlato a: task 005 (feedback risorse basse) — il cambio colore è lo stesso concetto
