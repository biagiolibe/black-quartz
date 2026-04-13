# Task Template — [Titolo del Task]

> **ID**: `XXX`
> **Categoria**: Architettura / Mappa / Giocatore / HUD / Audio / etc.
> **Priorità**: 🔴 P1 / 🟡 P2 / 🟢 P3
> **Stima**: ~1h / ~2h / mezza giornata
> **Assegnato a**: *(Antigravity / Claude CLI / non assegnato)*
> **Sessione**: *(ID conversazione o descrizione sessione, es. "Antigravity conv. 9d6957d8")*

---

## 🎯 Obiettivo

Descrizione chiara e concisa di cosa deve essere fatto e perché.

---

## 📋 Acceptance Criteria

Un task è considerato **completato** quando:

- [ ] Criterio 1
- [ ] Criterio 2
- [ ] Criterio 3 (es. `cargo check` passa senza errori)

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/...` | Descrizione del file e cosa toccherà |

---

## 🧩 Contesto Tecnico

Spiega in dettaglio il contesto necessario per eseguire il task **senza dover leggere il resto del codebase**.

- Quali struct/enum/eventi sono coinvolti?
- Qual è il comportamento attuale?
- Qual è il comportamento desiderato?

---

## 🔨 Implementazione Suggerita

Passi concreti o pseudo-codice per guidare l'implementazione.

```rust
// Esempio di codice o struttura da creare/modificare
```

---

## ⚠️ Vincoli e Attenzioni

- Non rompere la compilazione (il progetto deve restare compilabile a ogni step)
- Non introdurre dipendenze esterne senza approvazione
- Bevy 0.16: usare `single()` / `single_mut()` (non `get_single`), `write()` (non `send()`)
- Eventuali note specifiche sul progetto

---

## 🔗 Dipendenze

- Dipende da: *(altri task ID o nessuno)*
- Blocca: *(altri task ID o nessuno)*

---

## 🤖 Come delegare questo task a un agente

### Opzione A — Antigravity (nuova conversazione)

Apri una nuova chat e scrivi come primo messaggio:

```
Leggi il file `/Users/biagioliberto/dev/src/biagiolibe/black-quartz/tasks/XXX-nome-task.md`
ed esegui il task descritto. Il progetto è in `/Users/biagioliberto/dev/src/biagiolibe/black-quartz/`.
```

### Opzione B — Claude CLI (terminale)

```bash
cd /Users/biagioliberto/dev/src/biagiolibe/black-quartz
claude "$(cat tasks/XXX-nome-task.md)"$'\n\nEsegui questo task nel progetto corrente.'
```

### Dopo la delega

1. Aggiorna `Assegnato a` e `Sessione` in questo file
2. Cambia stato in `QUEUE.md`: `[ ]` → `[/]`
3. Al completamento: `mv tasks/XXX-nome-task.md tasks/done/` e aggiorna `QUEUE.md`
