# Task 010 — Miglioramenti visivi menu e HUD (bottoni, inventario, stile)

> **ID**: `010`
> **Categoria**: HUD & Interfaccia
> **Priorità**: 🟢 P3
> **Stima**: ~2h
> **Assegnato a**: *(non assegnato)*
> **Sessione**: *(da assegnare)*

---

## 🎯 Obiettivo

Migliorare l'esperienza visiva della UI con tre modifiche:

1. **Feedback bottoni menu**: I bottoni non hanno feedback visivo su hover/press. Aggiungere cambio colore.
2. **Display inventario HUD**: L'inventario mostra `{:?}` (debug format) con virgolette extra. Usare la stringa formattata direttamente.
3. **Stile menu**: Menu tutto nero, text 20pt, bottoni senza stile. Aggiungere titoli 30pt, bottoni con sfondo+padding, pannelli con bordo.

---

## 📋 Acceptance Criteria

Un task è considerato **completato** quando:

- [ ] I bottoni nel menu cambiano colore su hover (grigio scuro) e press (grigio chiaro)
- [ ] Il display inventario in HUD mostra testo senza virgolette (es. `"iron x2, coal x1"` non `"\"iron x2, coal x1\""`)
- [ ] I titoli dei menu ("Drill McDrillface", "World base", "Game Over") sono a font_size 30 (invece di 20)
- [ ] I bottoni hanno `BackgroundColor` visibile + padding 8px + width 220px
- [ ] I pannelli menu hanno bordo grigio 2px
- [ ] `cargo check` passa senza errori
- [ ] Il gioco compila e la UI è visivamente migliorata

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/menu.rs` | Modifica: `init_menu` (bottoni + titoli), `MenuPlugin::build` (aggiunta sistema feedback), costanti colori |
| `src/hud.rs` | Modifica: `update_hud` riga ~207 (inventario text) |

---

## 🧩 Contesto Tecnico

### Menu bottoni (attuale)

In `src/menu.rs`, `init_menu` alla riga ~118, i bottoni sono spawned così:
```rust
popup.spawn((Button, NewGame)).with_children(|button| {
    button.spawn((
        Text::new("Start game"),
        font_style.clone(),
        TextColor(Color::WHITE),
    ));
});
```

**Problema**: Non hanno `BackgroundColor`, `Node` con padding/width, né feedback visivo.

### Inventario HUD (attuale)

In `src/hud.rs`, `update_hud` riga ~206:
```rust
*text_writer.text(inventory_text_entity, 1) = format!("{:?}", inventory.print_items());
```

**Problema**:
- `inventory.print_items()` già ritorna `String` formattata come `"iron x2, coal x1"`
- `format!("{:?}", ...)` aggiunge virgolette attorno: `"\"iron x2, coal x1\""`

### Bevy UI Components

- `Button`: marker component per bottoni interattivi
- `Interaction`: enum `None | Hovered | Pressed` (si legge con `Changed<Interaction>`)
- `BackgroundColor`: colore sfondo dell'elemento UI
- `Node`: styling dimensioni/padding/flex
- `BorderColor` + `border` in `Node`: per bordi UI
- `TextFont`: font_size, font, etc.

---

## 🔨 Implementazione Suggerita

### 1. Fix HUD inventario (1 riga in `src/hud.rs` riga ~206)

```rust
// Prima
*text_writer.text(inventory_text_entity, 1) = format!("{:?}", inventory.print_items());

// Dopo
*text_writer.text(inventory_text_entity, 1) = inventory.print_items().clone();
// oppure se print_items() ritorna String direttamente
*text_writer.text(inventory_text_entity, 1) = inventory.print_items();
```

Verifica in `src/player/components.rs` riga ~165 che `print_items()` ritorni `String`.

### 2. Menu bottoni styling + feedback (`src/menu.rs`)

**a) Aggiungere costanti colore all'inizio del file (dopo gli import, prima del plugin)**

```rust
const BTN_NORMAL: Color = Color::srgba(0.15, 0.15, 0.15, 0.8);
const BTN_HOVERED: Color = Color::srgba(0.3, 0.3, 0.3, 0.9);
const BTN_PRESSED: Color = Color::srgba(0.5, 0.5, 0.5, 1.0);

fn button_node() -> Node {
    Node {
        padding: UiRect::all(Val::Px(8.0)),
        width: Val::Px(220.0),
        justify_content: JustifyContent::Center,
        ..default()
    }
}
```

**b) Aggiungere titoli font style in `init_menu`**

Subito dopo la definizione di `font_style`:
```rust
let title_font_style = TextFont {
    font: font.clone(),
    font_size: 30.0,
    ..Default::default()
};
```

**c) Modificare spawn titoli** (riga ~101, ~135, ~198)

```rust
// Prima
popup.spawn((
    Text::new("Drill McDrillface"),
    font_style.clone(),
    TextColor(Color::WHITE),
));

// Dopo
popup.spawn((
    Text::new("Drill McDrillface"),
    title_font_style.clone(),  // <- usare title invece di font_style
    TextColor(Color::WHITE),
));
```

**d) Modificare spawn bottoni** — aggiungere `button_node()` e `BackgroundColor`

```rust
// Prima
popup.spawn((Button, NewGame)).with_children(|button| {
    button.spawn((
        Text::new("Start game"),
        font_style.clone(),
        TextColor(Color::WHITE),
    ));
});

// Dopo
popup.spawn((Button, NewGame, button_node(), BackgroundColor(BTN_NORMAL))).with_children(|button| {
    button.spawn((
        Text::new("Start game"),
        font_style.clone(),
        TextColor(Color::WHITE),
    ));
});
```

Fare questo per **tutti i bottoni** (NewGame, QuitGame, Sell, Refill, Resume, UpgradeDrill, UpgradeSpeed, UpgradeTank, UpgradeArmor).

**e) Aggiungere pannelli con bordo** (riga ~130, ~192)

```rust
// Prima
parent
    .spawn((
        parent_node.clone(),
        BackgroundColor(Color::BLACK),
        Visibility::Hidden,
    ))

// Dopo
parent
    .spawn((
        {
            let mut node = parent_node.clone();
            node.border = UiRect::all(Val::Px(2.0));
            node
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.95)),
        BorderColor(Color::srgba(0.4, 0.4, 0.4, 1.0)),
        Visibility::Hidden,
    ))
```

Oppure, in modo più conciso, sovrascrivere i campi direttamente con un Builder pattern.

**f) Aggiungere sistema feedback visivo in `MenuPlugin::build`**

```rust
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (init_menu, handle_start_menu)
                .in_set(GameSystems::Rendering)
                .chain(),
        )
        .add_systems(OnEnter(MenuState::WorldBase), handle_base_menu)
        .add_systems(OnEnter(MenuState::GameOver), handle_gameover_menu)
        .add_systems(OnEnter(MenuState::Inventory), handle_inventory_menu)
        .add_systems(OnEnter(MenuState::Settings), handle_settings_menu)
        .add_systems(Update, (
            handle_button_interaction,
            button_visual_feedback,  // <- AGGIUNGERE QUESTO
        ).in_set(GameSystems::Ui))
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
    }
}
```

**g) Aggiungere sistema `button_visual_feedback` prima di `handle_button_interaction`**

```rust
fn button_visual_feedback(
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg) in buttons.iter_mut() {
        bg.0 = match interaction {
            Interaction::Pressed => BTN_PRESSED,
            Interaction::Hovered => BTN_HOVERED,
            Interaction::None => BTN_NORMAL,
        };
    }
}
```

---

## ⚠️ Vincoli e Attenzioni

- **Bevy 0.16**: `Button` è marker component; `Interaction` è enum con `Changed<Interaction>` per query reactive
- **Dynamic linking**: Progetto usa `dynamic_linking` feature in dev, non influenza i cambiamenti UI
- **Ordinamento sistemi**: `button_visual_feedback` va PRIMA di `handle_button_interaction` perché questo legge `Pressed`, mentre il feedback deve aggiornare il colore prima (usa `Changed<Interaction>`)
- **Inventario**: verificare che `print_items()` ritorni `String` (non `&String`), altrimenti usare `.clone()`
- Non rompere il design dei pannelli menu esistenti — il 50%×50% deve restare, solo aggiungere bordo

---

## 🔗 Dipendenze

- Dipende da: nessuno
- Blocca: nessuno

---

## 🤖 Come delegare questo task a un agente

### Opzione A — Antigravity (nuova conversazione)

Apri una nuova chat e scrivi:

```
Leggi il file `/Users/biagioliberto/dev/src/biagiolibe/black-quartz/tasks/010-ui-improvements.md`
ed esegui il task descritto. Il progetto è in `/Users/biagioliberto/dev/src/biagiolibe/black-quartz/`.
```

### Opzione B — Claude CLI (terminale)

```bash
cd /Users/biagioliberto/dev/src/biagiolibe/black-quartz
claude "$(cat tasks/010-ui-improvements.md)"$'\n\nEsegui questo task nel progetto corrente.'
```

### Dopo la delega

1. Aggiorna `Assegnato a` e `Sessione` in questo file
2. Cambia stato in `QUEUE.md`: `[ ]` → `[/]`
3. Al completamento: `mv tasks/010-ui-improvements.md tasks/done/` e aggiorna `QUEUE.md` con `[x]`
