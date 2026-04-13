# Black Quartz — Development Plan

## Come funziona questo file

```
PROPOSTE  →  (revisione)  →  BACKLOG  →  (sviluppo)  →  COMPLETATI
```

| Simbolo | Significato |
|---------|-------------|
| `[ ]`   | Task nel backlog, da fare |
| `[/]`   | In corso (assegnato o in sviluppo) |
| `[x]`   | Completato |
| `[-]`   | Annullato o scartato |
| `[?]`   | Proposta — in attesa di valutazione |

### Task files per agenti
Ogni task approvato e complesso può avere un file dedicato in `tasks/<id>-nome-task.md`.
Quel file contiene tutto il contesto necessario affinché un agente possa eseguirlo in autonomia,
senza bisogno di leggere l'intero codebase.

Formato del riferimento: `→ tasks/<id>.md`

---

## 🗂️ SEZIONE 1 — PROPOSTE (da valutare)

> Le voci `[?]` qui sotto sono idee non ancora approvate.
> Prima di spostarle nel backlog, vanno discusse e confermate.

### Architettura

- `[?]` **A1** — Valutare se unificare `world_base.rs` nel modulo `map` *(rimandare: dipende da crescita del file)*
- `[?]` **A3** — Passare parametri ai sistemi tramite risorse config invece di costanti hardcoded *(rimandare: abbinare al task upgrade player)*

### Mappa & Generazione

- `[?]` **M3** — Aggiungere varianti visive per `Solid` (pattern cracked, stratificato) *(rimandare: dipende dagli asset grafici)*

### Giocatore & Meccaniche

*(nessuna proposta attiva)*

### HUD & Interfaccia

- `[?]` **H2** — Aggiungere indicatore di direzione/bussola *(rimandare: bassa priorità)*
- `[?]` **H4** — Aggiungere tooltip sugli item nell'inventario *(rimandare: richiede schermata inventario dedicata)*

### Animazioni & Camera

- `[?]` **AN1** — Aggiungere particelle di terra/polvere durante lo scavo *(rimandare: serve crate esterno o sistema custom)*

### Audio *(rimandare a dopo le feature core)*

- `[?]` **AU1** — Integrare `bevy_audio`
- `[?]` **AU2** — Aggiungere musica di sottofondo in loop
- `[?]` **AU3** — Aggiungere effetti sonori: scavo, impatto, raccolta item, acquisto
- `[?]` **AU4** — Variare i suoni in base al tipo di blocco scavato

### Economia

- `[?]` **E1** — Persistere la valuta tra le sessioni (salvataggio su file) *(rimandare: richiede design del formato di salvataggio)*
- `[?]` **E2** — Mostrare il prezzo degli upgrade nella UI della base *(rimandare: dipende da G1 - sistema upgrade)*

### Build & Deploy

- `[?]` **B2** — Creare uno script di packaging per macOS (`.app` bundle) *(rimandare: quando il gioco è più completo)*
- `[?]` **B3** — Valutare target WASM *(rimandare: limitazioni tecniche con Rapier + dynamic_linking)*

---

## 🔵 SEZIONE 2 — BACKLOG (approvati, da fare)

### 🏗️ Architettura

- `[ ]` Rimuovere i `pub use` inutilizzati in `map/mod.rs` e `player/mod.rs`
- `[ ]` Rimuovere import `crate::BlackQuartzCamera` inutilizzato in `player/mod.rs`
- `[ ]` Pulire `prelude.rs`: solo re-export espliciti, nessun wildcard non necessario

### 🗺️ Mappa & Generazione

- `[ ]` **M1** — Rigenerazione casuale della mappa a ogni nuova run (stile roguelike) → `tasks/002-map-regen-roguelike.md`
- `[ ]` **M2** — Ottimizzare il rendering dei tile: verificare e rimuovere ricalcoli inutili ogni frame
- `[ ]` Caricare le proprietà dei tile (durezza, loot, sprite index) da file `.ron` esterno → `tasks/001-tile-config-ron.md`

### 🤖 Giocatore & Meccaniche

- `[ ]` Bilanciare `damage_factor` e `armor_resistance` (deprecare `damage_factor` in favore di `armor_resistance`)
- `[ ]` **G1** — Sistema di upgrade acquistabili alla base (drill power, tank size, armor) → `tasks/003-upgrade-system.md`
- `[ ]` **G2/G3** — Effetto visivo lampeggiante quando salute e/o carburante sono bassi

### 👁️ Field of View

- `[ ]` Ottimizzare aggiornamento FOV: skippa se il giocatore non si è mosso
- `[ ]` Rimuovere log di debug `info!("Foving ...")` dal sistema FOV

### 🖥️ HUD & Interfaccia

- `[ ]` Collegare correttamente `Inventory` all'HUD (mostrare lista item, non solo count)
- `[ ]` **H1** — Aggiungere barra visiva per il carburante (stile progress bar)
- `[ ]` **H3** — Mostrare avviso visivo quando carburante scende sotto il 20% *(abbinato a G3)*

### 🎬 Animazioni & Camera

- `[ ]` **AN2** — Animare la distruzione del blocco (breve flash o variazione sprite)
- `[ ]` **AN3** — Smoothing della camera: interpolazione su più frame invece di snap diretto

### 💰 Economia

- `[ ]` **E3** — Bilanciare i prezzi dei minerali in `EconomyConfig` (analisi e proposta valori)

### 🧪 Qualità & Debug

- `[ ]` Rimuovere tutti i log `info!` di debug non necessari in produzione
- `[ ]` Scrivere test unitari per `world_to_grid_position` e `world_grid_position_to_idx`
- `[ ]` Scrivere test per la logica di distribuzione materiali

### 📦 Build & Deploy

- `[ ]` Verificare la compilazione in modalità `release` senza `dynamic_linking`
- `[ ]` **B1** — Aggiungere profilo `dist` in `Cargo.toml` ottimizzato per dimensione finale

---

## 🟡 SEZIONE 3 — IN CORSO

> Sposta qui i task quando inizi a lavorarci. Aggiungi chi lo sta sviluppando se utile.

*(nessuno al momento)*

---

## ✅ SEZIONE 4 — COMPLETATI

### 🏗️ Architettura

- `[x]` Separare il codice in moduli (`map`, `player`, `animation`, `camera`, `hud`, `menu`)
- `[x]` Introdurre un sistema di eventi (`DrillHitEvent`, `LootPickupEvent`, `TileDestroyedEvent`)
- `[x]` Aggiornare a Bevy 0.16 e bevy_rapier2d 0.31
- `[x]` Configurare `GameSystems` come `SystemSet` con ordinamento esplicito (`.chain()`)
- `[x]` Spostare `world_to_grid_position` e `world_grid_position_to_idx` in `map/components.rs`
- `[x]` Spostare costanti (`FILL_PROBABILITY`, `SIMULATION_STEPS`) in `map/components.rs`
- `[-]` **A2** — Bus centralizzato per gli eventi tra plugin *(scartato: complessità non giustificata)*

### 🗺️ Mappa & Generazione

- `[x]` Implementare generazione con Automi Cellulari
- `[x]` Distribuire materiali con Perlin Noise in base alla profondità
- `[x]` Generare bordi fisici della mappa con Rapier (`setup_borders`)
- `[x]` Implementare `handle_tile_destroyed` tramite evento `TileDestroyedEvent`

### 🤖 Giocatore & Meccaniche

- `[x]` Implementare movimento con tasti direzionali
- `[x]` Implementare scavo con riduzione integrità del tile
- `[x]` Implementare danno da collisione ad alta velocità (`collision_detection`)
- `[x]` Implementare rilevamento caduta con `ShapeCast` (`falling_detection`)
- `[x]` Implementare consumo carburante durante il movimento
- `[x]` Collegare `PlayerImpactEvent` al `CameraShake`
- `[x]` **G4** — Morte del player per esaurimento carburante *(già implementata in `death_detection`)*

### 👁️ Field of View

- `[x]` Implementare algoritmo BFS per il FOV sotterraneo
- `[x]` Memorizzare le celle visitate in `revealed_tiles`
- `[x]` Aggiornare l'overlay grafico (`FovOverlay`) in base alla visibilità

### 💰 Economia & Base

- `[x]` Implementare inventario con slot e capacità massima
- `[x]` Implementare vendita inventario alla base (`sell_all_inventory`)
- `[x]` Implementare rifornimento carburante alla base (`refill_tank`)
- `[x]` Definire `EconomyConfig` come risorsa configurabile

### 🎬 Animazioni & Camera

- `[x]` Animare l'idle della trivella con TextureAtlas
- `[x]` Implementare `CameraShake` con timer e intensità
- `[x]` Aggiungere sistema `handle_camera_shake` collegato a `PlayerImpactEvent`

---

*Ultimo aggiornamento: Aprile 2026*
