# Task Execution Queue

Questa è la coda di esecuzione dei task. I task sono ordinati per priorità di sviluppo.

## Come usare questa coda

- **Esecuzione**: prendere sempre il primo task `[ ]` dall'alto verso il basso
- **Riordinare**: spostare le righe per cambiare la priorità di esecuzione
- **Stato**: aggiornare il simbolo quando si inizia (`[/]`) o completa (`[x]`) un task
- **Delegare a un agente**: aprire il task file e seguire la sezione `🤖 Come delegare questo task`
- **Archiviare**: al completamento, `mv tasks/NNN.md tasks/done/` e aggiornare questa tabella

## Livelli di priorità

| Livello | Significato |
|---------|-------------|
| 🔴 P1   | Blocca altre feature o ha alto impatto sul gameplay |
| 🟡 P2   | Feature visibili ma non bloccanti |
| 🟢 P3   | Ottimizzazioni, pulizia, polish |

---

## 🤖 Delegare un task a un agente

### Antigravity (nuova conversazione)
> Apri una nuova chat e scrivi:
> *"Leggi `/Users/biagioliberto/dev/src/biagiolibe/black-quartz/tasks/NNN-nome.md` ed esegui il task."*

### Claude CLI (terminale)
```bash
cd /Users/biagioliberto/dev/src/biagiolibe/black-quartz
claude "$(cat tasks/NNN-nome.md)"$'\n\nEsegui questo task nel progetto corrente.'
```

---

## Coda attiva

| Stato | ID | Titolo | Priorità | Agente | Task File |
|-------|----|--------|----------|--------|-----------|
| `[ ]` | 003 | Sistema upgrade alla base (completare logica + limiti) | 🔴 P1 | — | [003](003-upgrade-system.md) |
| `[ ]` | 002 | Rigenerazione mappa a ogni nuova run | 🔴 P1 | — | [002](002-map-regen-roguelike.md) |
| `[ ]` | 005 | Visual feedback: lampeggio salute/carburante bassa + avviso 20% | 🔴 P1 | — | [005](005-low-resource-feedback.md) |
| `[ ]` | 008 | Animazione distruzione blocco (flash visivo) | 🟡 P2 | — | [008](008-tile-destroy-animation.md) |
| `[ ]` | 010 | Miglioramenti UI menu e HUD (bottoni, inventario, stile) | 🟢 P3 | — | [010](010-ui-improvements.md) |
| `[ ]` | 004 | Ottimizzazione FOV: skip se player fermo | 🟢 P3 | — | [004](004-fov-optimization.md) |
| `[ ]` | 009 | Ottimizzazione rendering tile | 🟢 P3 | — | [009](009-tile-render-optimization.md) |
| `[ ]` | 001 | Caricare proprietà tile da file `.ron` | 🟢 P3 | — | [001](001-tile-config-ron.md) |

---

## Task senza file dedicato (eseguibili direttamente)

Questi task sono semplici e non richiedono briefing per un agente:

| Stato | Descrizione | Priorità |
|-------|-------------|----------|
| `[ ]` | Rimuovere `pub use` inutilizzati in `map/mod.rs` e `player/mod.rs` | 🟢 P3 |
| `[ ]` | Rimuovere import `BlackQuartzCamera` inutilizzato in `player/mod.rs` | 🟢 P3 |
| `[ ]` | Rimuovere log `info!("Foving ...")` dal sistema FOV | 🟢 P3 |
| `[ ]` | Bilanciare `damage_factor` e `armor_resistance` (design + valori) | 🟡 P2 |
| `[ ]` | Bilanciare prezzi minerali in `EconomyConfig` (analisi + valori) | 🟡 P2 |
| `[ ]` | Aggiungere profilo `dist` in `Cargo.toml` | 🟢 P3 |
| `[ ]` | Verificare compilazione in modalità `release` senza `dynamic_linking` | 🟢 P3 |

---

## Completati

> I file di task completati sono archiviati in `tasks/done/`.

| Stato | ID | Titolo | Agente | File |
|-------|----|--------|--------|------|
| `[x]` | 006 | HUD: barra visiva carburante — già implementata in `hud.rs` | — | [006](done/006-hud-fuel-bar.md) |
| `[x]` | 007 | Camera smoothing — già implementato in `camera.rs` con `lerp` | — | [007](done/007-camera-smoothing.md) |
| `[x]` | Fix | Sprite integrità HUD: formula indice con `health.max` + clamp | Manuale | — |
| `[x]` | Fix | `armor_resistance` default: 1.0 → 0.0 in `PlayerAttributes` | Manuale | — |

---

*Aggiornato: Aprile 2026*
