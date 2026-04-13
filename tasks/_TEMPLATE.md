# Task Template — [Titolo del Task]

> **ID**: `XXX`
> **Categoria**: Architettura / Mappa / Giocatore / HUD / Audio / etc.
> **Priorità**: Alta / Media / Bassa
> **Stima**: ~1h / ~2h / mezza giornata
> **Assegnato a**: *(agente / nome)*

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
- Eventuali note specifiche sul progetto (es. "non usare `get_single`, usa `single`" in Bevy 0.16)

---

## 🔗 Dipendenze

- Dipende da: *(altri task ID o nessuno)*
- Blocca: *(altri task ID o nessuno)*
