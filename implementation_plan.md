# Implementazione Architetturale - Black Quartz

Questo documento descrive il piano di refactoring per migliorare l'architettura di **Black Quartz**, rendendola più idiomatica e scalabile secondo le best practice di Bevy.

## User Review Required

> [!WARNING]
> Questo refactoring comporterà la creazione di nuove cartelle, file e la riorganizzazione massiccia della base di codice (`map.rs` e `player.rs`).
> Controlla i passaggi proposti di seguito e, se ti sembrano corretti, approva il piano.

## Proposed Changes

---

### File e Cartelle Generali

Riorganizzeremo i "God Modules" creando directory dedicate. In `src/main.rs`, le importazioni `mod map;` e `mod player;` punteranno automaticamente a `src/map/mod.rs` e `src/player/mod.rs`. Esigiamo che questi ultimi ri-esportino le funzioni in modo che la `prelude` non debba essere modificata massivamente.

---

### Game Systems e Scheduler (game.rs)

Eviteremo i chain massicci su `Update` che forzano un ordine rigido, potenzialmente in conflitto con Rapier, e gestiremo meglio i Set.

#### [MODIFY] [game.rs](file:///Users/biagioliberto/dev/src/biagiolibe/black-quartz/src/game.rs)
- Rimuoveremo il blocco `.configure_sets(Update, (GameSystems::Loading, GameSystems::Rendering, GameSystems::Movement, ...).chain())`.
- Lasceremo solo `.configure_sets(OnEnter(GameState::Loading), ...)` se strettamente necessario, oppure li rimuoveremo del tutto, delegando le dipendenze locali ai plugin (tramite `.after()` o `.before()` dove strettamente richiesto).

---

### Riorganizzazione di Map

Separeremo la logica della generazione della mappa e FOV in moduli distinti all'interno di `src/map/`.

#### [DELETE] [map.rs](file:///Users/biagioliberto/dev/src/biagiolibe/black-quartz/src/map.rs)
Il file monolitico verrà cancellato in favore della nuova cartella.

#### [NEW] [map/mod.rs](file:///Users/biagioliberto/dev/src/biagiolibe/black-quartz/src/map/mod.rs)
- Conterrà il `MapPlugin`.
- Dichiarazione dei sottomoduli: `pub mod components; pub mod generation; pub mod fov; pub mod events;`.
- Riappropriazione asincrona e registrazione degli eventi come `DrillHitEvent` e `LootPickupEvent`.

#### [NEW] [map/components.rs](file:///Users/biagioliberto/dev/src/biagiolibe/black-quartz/src/map/components.rs)
- Conterrà `TileType`, `Tile`, `WorldGrid`, `Drilling`, e `TILE_SIZE`, `GRID_WIDTH`, `GRID_HEIGHT`.

#### [NEW] [map/events.rs](file:///Users/biagioliberto/dev/src/biagiolibe/black-quartz/src/map/events.rs)
- Implementazione dell'evento per disaccoppiare lo scavo: `pub struct DrillHitEvent { pub target: Entity, pub damage: f32, pub miner_entity: Entity }`.
- Implementazione dell'evento per i drop: `pub struct LootPickupEvent { pub item: Item, pub player_entity: Entity }`.

#### [NEW] [map/generation.rs](file:///Users/biagioliberto/dev/src/biagiolibe/black-quartz/src/map/generation.rs)
- Ospiterà `initialize_world_grid`, `distribute_materials`, `simulation`, `count_solid_neighbors`, e `setup_borders`.
- Includerà anche il sistema per reagire al `DrillHitEvent`. Quando questo evento emette danni, qui decrementeiamo l'integrità del blocco. Se distrutto, spawnerà un `LootPickupEvent`. Mettiamo questo sistema nel ciclo `FixedUpdate`.

#### [NEW] [map/fov.rs](file:///Users/biagioliberto/dev/src/biagiolibe/black-quartz/src/map/fov.rs)
- Sposteremo qui `update_fov` e `update_fov_overlay`.

---

### Riorganizzazione di Player

Separeremo componenti, logica di movimento e logica di collisione/danno. Sposteremo inoltre le logiche critiche su `FixedUpdate`.

#### [DELETE] [player.rs](file:///Users/biagioliberto/dev/src/biagiolibe/black-quartz/src/player.rs)
Il file monolitico verrà cancellato in favore della nuova cartella.

#### [NEW] [player/mod.rs](file:///Users/biagioliberto/dev/src/biagiolibe/black-quartz/src/player/mod.rs)
- Conterrà `PlayerPlugin`.
- Istruirà Bevy di lanciare i sistemi di `movement.rs` e `drilling.rs` in **`FixedUpdate`** invece che `Update`, in modo da slegarli dagli FPS.

#### [NEW] [player/components.rs](file:///Users/biagioliberto/dev/src/biagiolibe/black-quartz/src/player/components.rs)
- Sposteremo `Player`, `Health`, `Fuel`, `DrillState`, `PlayerAttributes`, `PlayerDirection`, `FieldOfView`, `Item`, `Inventory` e `Currency`.

#### [NEW] [player/movement.rs](file:///Users/biagioliberto/dev/src/biagiolibe/black-quartz/src/player/movement.rs)
- Trasferiremo `move_player`, `falling_detection`, e `update_player_direction`.
- Usa `FixedUpdate` (e quindi `Res<Time<Fixed>>` invece di `Res<Time>` quando misura delta time).

#### [NEW] [player/drilling.rs](file:///Users/biagioliberto/dev/src/biagiolibe/black-quartz/src/player/drilling.rs)
- Modificheremo `drill` per **NON** alterare più `Tile` direttamente ma per mandare un evento `DrillHitEvent` a `map`.
- Implementeremo un sistema per catturare `LootPickupEvent` ed aggiungere automaticamente le risorse all'inventario, mantenendolo slegato dalla mappa.
- Conterrà `collision_detection` e `death_detection`.

## Open Questions

- Nel gioco l'inventario è parte del giocatore (`Component`); se credi che convenga renderlo una risorsa (`Resource`) per facilitare l'accesso dall'HUD e salvarne lo stato a parte, possiamo valutare di farlo. Per ora il piano prevede di mantenerlo come Componente.
- Ci sono altri eventi di impatto (ad esempio suoni o particelle) che desideri agganciare in futuro ai nuovi `DrillHitEvent` o `LootPickupEvent`?

## Verification Plan

### Manual Verification
- Dopo il refactoring non ci devono essere errori di sintassi cargo / ruggine.
- Avviare il gioco con `cargo run`: il giocatore deve muoversi, volare e scavare esattamente come prima.
- La generazione del mondo e i minerali non devono presentare alterazioni.
- Consumi di carburante e integrità blocco scavato non devono fluttuare in base a cali di FPS grazie all'adesione a `FixedUpdate`.
