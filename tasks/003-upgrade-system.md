# Task 003 — Sistema di Upgrade alla Base

> **ID**: `003`
> **Categoria**: Giocatore & Meccaniche
> **Priorità**: Alta
> **Stima**: ~2h
> **Assegnato a**: *(non assegnato)*

---

## 🎯 Obiettivo

Implementare la logica completa degli upgrade acquistabili alla base (World Base).
I bottoni UI esistono già in `menu.rs`, i costi sono già definiti in `EconomyConfig`,
e la logica di acquisto è **parzialmente implementata** in `handle_button_interaction`.

Il task consiste nel:
1. Completare e correggere la logica degli upgrade (verifica saldi, applicazione degli effetti)
2. Aggiungere feedback visivo sull'acquisto (testo di conferma o disabilitazione bottone)
3. Aggiungere un limite massimo agli upgrade (evitare valori infiniti)
4. Verificare che gli upgrade persistano per tutta la durata della sessione

---

## 📋 Acceptance Criteria

- [ ] Tutti e 4 gli upgrade funzionano: Drill, Speed, Tank, Armor
- [ ] Se la valuta è insufficiente, l'acquisto non viene eseguito (già implementato, verificare)
- [ ] Ogni upgrade ha un massimo applicabile (es. drill power max 5.0, armor max 0.8)
- [ ] Il testo del bottone mostra il livello corrente dell'upgrade (es. "Upgrade Drill [Lv.2] (80c)")
- [ ] `cargo check` e `cargo run` passano senza errori

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/menu.rs` | Contiene `init_menu`, `handle_button_interaction`, `MenuButton` — da modificare |
| `src/game.rs` | Contiene `EconomyConfig` con i costi degli upgrade — da estendere con limiti |
| `src/player/components.rs` | Contiene `PlayerAttributes`, `Fuel`, `Currency` — da consultare |

---

## 🧩 Contesto Tecnico

### Stato attuale degli upgrade in `menu.rs`

La logica di acquisto **è già parzialmente implementata** in `handle_button_interaction`:

```rust
UpgradeDrill => {
    if let Ok((_, _, mut currency, mut attributes)) = player.single_mut() {
        if currency.amount >= economy.upgrade_drill_cost {
            currency.amount -= economy.upgrade_drill_cost;
            attributes.drill_power += 0.5;
            info!("Upgraded drill power to {:.1}", attributes.drill_power);
        }
    }
}
UpgradeSpeed => {
    // ground_speed_factor += 20.0, flying_speed_factor += 20.0
}
UpgradeTank => {
    // fuel.max += 50.0
}
UpgradeArmor => {
    // armor_resistance += 0.1
}
```

### Bottoni esistenti in `init_menu`

```rust
popup.spawn((Button, UpgradeDrill)).with_children(|button| {
    button.spawn((Text::new("Upgrade Drill (80c)"), ...));
});
popup.spawn((Button, UpgradeSpeed)).with_children(|button| {
    button.spawn((Text::new("Upgrade Speed (60c)"), ...));
});
popup.spawn((Button, UpgradeTank)).with_children(|button| {
    button.spawn((Text::new("Upgrade Tank (100c)"), ...));
});
popup.spawn((Button, UpgradeArmor)).with_children(|button| {
    button.spawn((Text::new("Upgrade Armor (70c)"), ...));
});
```

### PlayerAttributes (valori default)

```rust
pub struct PlayerAttributes {
    pub drill_power: f32,          // default: 1.0
    pub damage_factor: f32,        // default: 0.05 (da deprecare)
    pub armor_resistance: f32,     // default: 0.0  → riduce i danni da impatto
    pub ground_speed_factor: f32,  // default: 200.0
    pub flying_speed_factor: f32,  // default: 200.0
    pub fuel_efficiency: f32,      // default: 0.3  → consumo carburante per frame
}
```

### EconomyConfig (valori default)

```rust
pub struct EconomyConfig {
    pub fuel_price_per_unit: u32,   // 2
    pub fuel_refill_amount: f32,    // 100.0
    pub upgrade_drill_cost: u32,    // 80
    pub upgrade_speed_cost: u32,    // 60
    pub upgrade_tank_cost: u32,     // 100
    pub upgrade_armor_cost: u32,    // 70
}
```

### Fuel (valori default)

```rust
pub struct Fuel {
    pub current: f32,   // default: 100.0
    pub max: f32,       // default: 100.0 → aumenta con UpgradeTank
}
```

---

## 🔨 Implementazione Suggerita

### Step 1: Aggiungere limiti massimi agli upgrade in `EconomyConfig`

```rust
// src/game.rs
pub struct EconomyConfig {
    // ...esistenti...
    pub max_drill_power: f32,       // es. 5.0
    pub max_speed_factor: f32,      // es. 400.0
    pub max_tank_capacity: f32,     // es. 300.0
    pub max_armor_resistance: f32,  // es. 0.8 (80% riduzione danni)
}

impl Default for EconomyConfig {
    fn default() -> Self {
        EconomyConfig {
            // ...esistenti...
            max_drill_power: 5.0,
            max_speed_factor: 400.0,
            max_tank_capacity: 300.0,
            max_armor_resistance: 0.8,
        }
    }
}
```

### Step 2: Aggiornare la logica in `handle_button_interaction`

Aggiungere il check sul limite massimo per ogni upgrade:

```rust
UpgradeDrill => {
    if let Ok((_, _, mut currency, mut attributes)) = player.single_mut() {
        if currency.amount >= economy.upgrade_drill_cost
            && attributes.drill_power < economy.max_drill_power
        {
            currency.amount -= economy.upgrade_drill_cost;
            attributes.drill_power = (attributes.drill_power + 0.5).min(economy.max_drill_power);
            info!("Upgraded drill power to {:.1}", attributes.drill_power);
        }
    }
}
// ...stesso pattern per Speed, Tank, Armor
```

### Step 3: Aggiornare il testo dei bottoni dinamicamente

Aggiungere un sistema `update_upgrade_buttons` che aggiorna il testo in base al livello:

```rust
// Aggiungere marker component per identificare i bottoni upgrade
#[derive(Component)]
pub struct UpgradeButtonText(pub MenuButton);  // o usare il MenuButton già esistente

// Sistema di aggiornamento
fn update_upgrade_buttons(
    player: Query<&PlayerAttributes, With<Player>>,
    mut text_query: Query<(&mut Text, &Parent)>,
    button_query: Query<&MenuButton>,
) {
    if let Ok(attrs) = player.single() {
        for (mut text, parent) in text_query.iter_mut() {
            if let Ok(button) = button_query.get(parent.get()) {
                match button {
                    MenuButton::UpgradeDrill => {
                        // Calcola il livello: (drill_power - 1.0) / 0.5
                        let level = ((attrs.drill_power - 1.0) / 0.5) as u32;
                        *text = Text::new(format!("Upgrade Drill [Lv.{}] (80c)", level));
                    }
                    // ...altri bottoni...
                    _ => {}
                }
            }
        }
    }
}
```

> ⚠️ Nota: in Bevy 0.16 `Text` viene aggiornato tramite `TextUiWriter` o sostituendo il componente `Text` direttamente. Verificare l'approccio corretto controllando come `update_hud` in `hud.rs` aggiorna i testi.

### Step 4: Registrare il sistema

```rust
// src/menu.rs, nel Plugin build
app.add_systems(
    Update,
    update_upgrade_buttons.run_if(in_state(GameState::Menu)),
);
```

---

## ⚠️ Vincoli e Attenzioni

- **Bevy 0.16**: usare `single()` (non `get_single()`), `single_mut()` (non `get_single_mut()`)
- Il sistema `handle_button_interaction` usa già `player.single_mut()` con il pattern `(Inventory, Fuel, Currency, PlayerAttributes)` — mantenere la stessa firma
- Non modificare i valori di default di `PlayerAttributes` — gli upgrade si applicano durante la sessione
- Gli upgrade si **azzerano** a ogni nuova partita (il player viene rispawnato con `Default`)
- Il costo del fuel è in `fuel_price_per_unit` (crediti per unità), non in `fuel_refill_amount` (importo fisso per rifornimento completo)
- **Non** aggiungere dipendenze esterne

---

## 🔗 Dipendenze

- Dipende da: nessuno (logica già parzialmente presente)
- Blocca: `E2` (mostrare prezzi upgrade in UI — questo task già li mostra)
- Correlato a: task 001, task 002 — possono essere eseguiti in parallelo
