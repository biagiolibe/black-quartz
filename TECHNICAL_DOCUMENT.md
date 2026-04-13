# Black Quartz - Technical Design Document

Questo documento tecnico descrive l'architettura, i componenti principali e le meccaniche di base del gioco **Black Quartz**, un videogioco 2D sviluppato in **Rust** utilizzando il motore **Bevy** e il motore fisico **Bevy Rapier**.

## 1. Panoramica del Progetto
**Black Quartz** è un gioco di escavazione e gestione risorse in cui il giocatore comanda una "Drilling Machine" (trivella). L'obiettivo principale sembra essere l'esplorazione del sottosuolo, la raccolta di minerali preziosi tramite scavo, la gestione di risorse essenziali (carburante e salute del veicolo) e il potenziamento/commercio in un'economia in-game.

### Stack Tecnologico
- **Linguaggio**: Rust (edizione 2021)
- **Game Engine**: Bevy (0.17)
  - Modelli di programmazione: ECS (Entity-Component-System)
  - Finestre e Rendering standard di Bevy
- **Fisica**: `bevy_rapier2d` (0.25.0) per la collisione e dinamica dei corpi rigidi (gravità, urti)
- **Generazione Procedurale**: `noise` (0.9.0) e `rand` per la generazione della mappa e distribuzione dei minerali.

---

## 2. Architettura di Sistema (Game State & Systems)
Il flusso di gioco è gestito tramite gli state machine di Bevy, configurati principalmente in `game.rs`.

**Stati del Gioco (`GameState`)**:
- `Loading`: Caricamento degli asset e preparazione iniziale.
- `MainMenu`: Menu principale.
- `Rendering`: Fase di generazione o configurazione visiva iniziale (es. mappa).
- `Playing`: Ciclo di gioco attivo in cui il giocatore può muoversi ed esplorare.
- `Menu`: Menu in-game (es. inventario, pausa).
- `GameOver`: Fine della partita, raggiunta ad esempio in caso di esaurimento salute o carburante.

**Game Systems**:
Il plugin principale divide l'aggiornamento nelle seguenti pipeline:
`Loading` → `Rendering` → `Movement` → `Camera` → `Physics` → `Collision` → `Animation` → `Ui`.

---

## 3. Entità Principale: Il Giocatore (`player.rs`)

Il giocatore è rappresentato da un'entità complessa che raggruppa numerosi componenti essenziali.

### 3.1. Componenti del Giocatore
La navicella (Drilling Machine) fa uso dei seguenti componenti principali:
- **`Health` & `Fuel`**: Tracciano rispettivamente la vitalità della trivella e il livello di carburante. Muoversi consuma carburante, gli impatti riducono la salute.
- **`Inventory` & `Currency`**: L'inventario a slot memorizza i materiali scavati (con nome, quantità e valore). `Currency` tiene traccia della valuta in-game.
- **`DrillState`**: Una macchina a stati per le animazioni e le logiche della trivella (`Idle`, `Flying`, `Drilling`, `Falling`).
- **`PlayerAttributes`**: Determinano le prestazioni del veicolo:
  - `drill_power`: Velocità di scavo.
  - `armor_resistance` / `damage_factor`: Resistenza ai danni da caduta.
  - `ground_speed_factor` / `flying_speed_factor`: Velocità di movimento.
  - `fuel_efficiency`: Quoziente di consumo del carburante.
- **Fisica Rapier**: Usa `RigidBody::Dynamic`, `Collider::capsule_y` e `Velocity`. La rotazione è bloccata (`LockedAxes::ROTATION_LOCKED`) e c'è una certa applicazione di smorzamento lineare e angolare (`Damping`), modificata dinamicamente in base a se il player è `Idle` (maggiore attrito) oppure in volo.

### 3.2. Meccaniche di Movimento e Scavo
- **Movimento**: Utilizza le frecce direzionali. Un movimento orizzontale imposta la velocità orizzontale, uno verticale abilita il volo (`Flying`) con il consumo aumentato del carburante.
- **Gravity & Falling**: Un sistema inietta "ShapeCasting" verso il basso per determinare se il giocatore sta cadendo verso il vuoto (`DrillState::Falling`).
- **Danno da Impatto**: Rileva le collisioni (`CollisionEvent`). Se la trivella urta il suolo ad alta velocità (superiore a 300.0) e non sta scavando, subisce danni proporzionali alla velocità d'impatto. Causa anche un effetto di scuotimento della telecamera (`CameraShake`).
- **Scavo (`drill`)**: Quando si spinge contro un blocco e si è a contatto, lo stato diventa `Drilling`. La vita (integrità) del blocco decremezza calcolata da `drill_power * delta_time * (1 - hardness)`. Quando a zero, il blocco scompare e rilascia l'item corrispondente nell'inventario del giocatore.

---

## 4. Generazione Mappa e Field Of View (FOV)

Questo capitolo esplora in modo approfondito due dei sistemi più complessi di Black Quartz: la generazione del mondo di gioco sotterraneo e l'algoritmo di calcolo della visibilità (Fog of War) contenuti in `map.rs`.

### 4.1. Generazione Procedurale della Mappa
La mappa di Black Quartz (`GRID_WIDTH = 100`, `GRID_HEIGHT = 500`, `TILE_SIZE = 32.0`) viene creata all'avvio nel sistema `initialize_world_grid` tramite una pipeline articolata in tre fasi: un riempimento stocastico, una simulazione con Automi Cellulari e infine un texturing spaziale guidato dal Perlin Noise e dalla profondità.

**Fase 1: Inizializzazione Stocastica**
Viene creata una grande matrice bidimensionale. Inizia iterando su ogni cella e determinando il suo stato iniziale tramite pura casualità (`rand`):
- Se il valore casuale generato è inferiore al `FILL_PROBABILITY` (`0.55`), la cella diviene di tipo `Solid`.
- Altrimenti rimane `Empty`.

**Fase 2: Automi Cellulari (Smoothing)**
Per agglomerare il rumore iniziale in formazioni naturali e grotte, il gioco passa la matrice ad una funzione `simulation` per `SIMULATION_STEPS` iterazioni (di default 4 volte).
Il sistema iterativo si basa sulle seguenti regole di controllo spaziale (vicinato 3x3):
1. **Regola dell'esaurimento**: `(Solid, n) se n < 3 => Empty`
   *Se una cella di roccia è isolata, frana e diventa vuoto.*
2. **Regola della formazione**: `(Empty, n) se n > 4 => Solid`
   *Se uno spazio vuoto è circondato da almeno 5 pareti solidi, viene "riempito" di roccia chiudendo piccoli tunnel isolati.*

**Fase 3: Distribuzione Procedurale dei Minerali (Perlin Noise)**
Conclusa la rimodellazione spaziale, la funzione `distribute_materials` trasforma le celle generiche `Solid` nei vari tile di gioco. Utilizza il **Rumore Perlin** interpolato alla **profondità fisica (Y) del giocatore**. A causa del sistema di coordinate, l'indice `0` (array) rappresenta il fondo, mentre `>= 400` è lo strato di *superficie*:
- **In superficie (y > 400)**: Il rumore definisce il 70% come `Solid` neutro, il resto si divide in Rame (`Copper`) e Ferro (`Iron`) oltre a della sabbia morbida (`Sand`).
- **Nello strato mediano (y 100-400)**: Rame e sabbia scompaiono e iniziano i giacimenti d'oro (`Gold`), resi accessibili solo nelle soglie di rumore oltre il 90esimo percentile.
- **In profondità estreme (y < 100)**: Scompaiono i minerali poveri, ma la fascia oltre il 90esimo percentile abilita l'allocazione casuale e stesa dei rarissimi Cristalli (`Crystal`).

### 4.2. Caratteristiche dei Materiali (`TileType`)
Ogni blocco ha determinati attributi fisici che si relazionano col giocatore:
- **Sand**: Fragilissima (Integrità: 0.1, Durezza: 0.05). Nessun loot.
- **Copper** (Rame): Integrità 0.4. Dà un oggetto `Copper` dal valore di 5 crediti.
- **Iron** (Ferro): Integrità elevata 0.6. Valore 10 crediti.
- **Gold** (Oro): Durezza moderata (0.2). Valutazione preziosa: 25 crediti.
- **Crystal** (Cristallo): Fragile ma rarissimo (spesso in profondità estreme). Valore molto alto: 50 crediti.

### 4.3. Visibilità: Field of View (FOV) Algorithm 
Una volta istanziato (con Rapier per le pareti), lo scenario sotterraneo è coperto dalla Nebbia di Guerra (Fog of War). Il sistema di FOV di Black Quartz calcola e memorizza le porzioni esplorata.

**Algoritmo Breadth-First-Search (Raycasting Sotterraneo)**
Il gioco usa un algoritmo iterativo in ampiezza (BFS tramite una Coda `VecDeque`) per svelare l'orizzonte, implementato in `update_fov`:
1. Inizia dalla posizione del giocatore e si propaga con Hop circolari.
2. Esplora nei limiti forniti da un raggio visivo della navicella (`fov.radius`).
3. **Occlusione della Vista**: Se la cella analizzata non è Vuota (incontra un muro), viene memorizzata come visibile, ed il BFS **blocca la sua propagazione** in quella direzione. Questo simula illuminazione limitata agli ostacoli anziché passare attraverso pareti non spaccate.

**Svelamento Grafico dell'Ambiente (`update_fov_overlay`)**
- Quando il BFS segna nuovi blocchi (`fov.dirty`), il gestore preleva gli Sprite ECS di questi mattoni opacizzati/neri.
- Inietta colore (`Color::WHITE`) e salva la posizione raggiunta in un sistema di *Memoria Visiva* (`revealed_tiles`). Di conseguenza le aree visitate restano scoperte globalmente permanentemente favorendo l'esplorazione persistente in puro stile roguelike scava/esplora.
- I bordi massicci ed irraggiungibili del mondo possiedono grossi Collider che tengono il veicolo dentro i limiti dell'area generata.

---

## 5. Sistemi Ausiliari

- **HUD/Interfaccia (hud.rs)** (Non approfondito ma indicato in GameSystems::Ui): L'interfaccia deve mostrare salute e fuel attuali, oltre potenziale inventario e valute, legati ai rispettivi component ECS.
- **Economia (`EconomyConfig`)**: Risorsa configurabile che indica i prezzi predefiniti di base. Ad esempio, il `fuel_price_per_unit` base è 2 crediti, definendo una necessità per il giocatore di estrarre per potersi comprare il carburante necessario per continuare.
- **Animazioni e Camera (`animation.rs`, `camera.rs`)**: Vengono fatte le animazioni basate sull'indice del TextureAtlas e la camera segue costantemente il giocatore con piccoli aggiustamenti e "shake" algoritmici quando ci sono forti collisioni.
- **Menu e Interfacce utente**: Sono previsti strati per i Menu principali e condizioni GameOver se terminano la salute o il carburante.

## Conclusioni
L'architettura del progeto sfrutta molto bene le caratteristiche dell'ECS di Bevy separando completamente i dati logici (es. `Fuel`, `DrillState`) dai sistemi d'iterazione (es `move_player`, `drill`). La generazione procedurale e il sistema a fisica Rapier indicano che il comparto dinamico e tattile esplorativo rappresenta il cuore pulsante del gioco, portando avanti un'ispirazione ai colossi del genere ma sfruttando un design pattern moderno e puramente in Rust.
