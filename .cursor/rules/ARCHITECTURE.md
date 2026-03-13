# ARCHITECTURE.md - Sandbox Exploration Game

## Philosophy

**Minimalist Maximalist:** Only as complex as needed, but capable of anything the game requires.

Three rules:
1. **One job, done well** — Systems don't overlap
2. **Explicit over implicit** — No magic
3. **Data down, events up** — Clear hierarchy

---

## Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Framework | React 18 + Vite | Fast dev, UI components |
| 3D Engine | Babylon.js 7.x | Rendering, scene management |
| State | Zustand | UI state bridge |
| Port | 3000 | Dev server |

---

## Core Architecture

### 1. Single Game Instance

One `Game` class owns everything. No singletons. Zustand is ONLY for React UI reads.

```
Game
├── clock           # Timing
├── eventBus        # State change messaging
├── input           # Controllers (1-4) + keyboard
├── scene           # Babylon scene
├── entities        # All game objects
├── collision       # Spatial hash + queries
├── areas           # Area management
├── debug           # All debug tools
└── systems         # Gameplay logic
    ├── player
    ├── resource
    ├── animal
    ├── boat
    ├── shopkeeper
    ├── storyEvent
    └── ...
```

### 2. Entity Model

Entities are plain objects with optional typed data. No component indirection.

```typescript
interface Entity {
  id: string
  type: EntityType
  position: Vector3
  rotation: number
  mesh: Mesh
  area: AreaType          // Which area this entity belongs to
  
  // Type-specific data (only one populated per entity)
  player?: PlayerData
  resource?: ResourceData
  animal?: AnimalData
  boat?: BoatData
  shopkeeper?: ShopkeeperData
  structure?: StructureData
  projectile?: ProjectileData
}

type EntityType = 
  | 'player'
  | 'resource'      // Trees, cactus, crystals, stone, cloud trees, palm trees
  | 'animal'        // Chickens, pigs, cows, bats
  | 'boat'
  | 'shopkeeper'
  | 'structure'     // Boat house, catapult, teleporter portal, tunnels
  | 'stump'
  | 'sapling'
  | 'projectile'    // Coconuts thrown at bats
  | 'spider'
  | 'cloudPlatform'

type AreaType = 
  | 'spring' | 'summer' | 'fall' | 'winter'  // Starting areas
  | 'farm'
  | 'beach' | 'desert'  // Combined as beach/desert
  | 'mountain'
  | 'volcano'
  | 'ethereal'
  | 'river'         // River channels
  | 'parkour'       // Cloud parkour area

interface PlayerData {
  playerIndex: 0 | 1 | 2 | 3      // Which player (supports 1-4)
  currentArea: AreaType
  inventory: ResourceInventory
  capacity: number
  tools: PlayerTools
  hasBoatHouse: boolean
  hasCactusArmor: boolean
  hasTeleporter: boolean
}

interface ResourceInventory {
  wood: number
  meat: number
  cactus: number
  coconuts: number
  stone: number
  crystal: number
  clouds: number
  hay: number                     // Bought at farm, used for breeding
  saplings: SaplingInventory
}

interface SaplingInventory {
  treeSaplings: number
  cactusSeed: number
  babyCrystal: number
  etherealSapling: number
}

interface PlayerTools {
  axeTier: 'wood' | 'stone' | 'iron' | 'steel'
  swordTier: 'wood' | 'stone' | 'iron' | 'steel'
  hasMachete: boolean
  hasChisel: boolean
  hasPickaxe: boolean
  hasEtherealAxe: boolean
}

interface ResourceData {
  resourceType: ResourceType
  variant: number               // 1, 2, or 3 (affects yield)
  size: 'small' | 'medium' | 'large'
  harvestProgress: number
  isRotten: boolean             // Trees only
  isPalmTree: boolean           // Shake for coconuts instead of chop
}

type ResourceType = 'tree' | 'cactus' | 'crystal' | 'stone' | 'cloudTree' | 'palmTree'

interface AnimalData {
  animalType: 'chicken' | 'pig' | 'cow' | 'bat'
  state: AnimalState
  fedHay: boolean               // For breeding
  isRelaxing: boolean           // Beach animals on chairs
  targetPlayer: string | null   // Bats chasing player
}

type AnimalState = 'idle' | 'wandering' | 'fleeing' | 'breeding' | 'chasing' | 'relaxing'

interface BoatData {
  passengers: string[]          // Player IDs on boat
  riverChannel: number          // Which of 4 river channels (0-3)
  riverPosition: number         // 0 = seasonal area, 1 = farm
  isSinking: boolean
  sinkTimer: number
  isStopped: boolean            // Collision with another boat
}

interface ShopkeeperData {
  shopkeeperType: ShopkeeperType
  isAlive: boolean
  dialogueState: string
}

type ShopkeeperType = 
  | 'seasonal'      // Generic for all 4 seasons
  | 'farm'          // Original farm keeper
  | 'farmNephew'    // Replacement after stampede
  | 'beach'
  | 'mountain'
  | 'volcano'
  | 'volcanoBat'    // Replacement after eruption
  | 'cloudBear'

interface StructureData {
  structureType: StructureType
  isBuilt: boolean
  owner: number | null          // Player index who built it
}

type StructureType = 
  | 'boatHouse'
  | 'catapult'
  | 'teleporterPortal'
  | 'tunnel'
  | 'woodenBarrier'             // Blocks beach access until stampede
  | 'spiderWeb'                 // Blocks mountain access until cactus armor
  | 'wallCrack'                 // Mineable exit from volcano
```

### 3. Area System

**NO LOAD ZONES** - The entire world exists as one seamless, connected space. All areas are loaded simultaneously and visible from neighboring areas. Travel between areas is purely physical (walking, boating, catapulting, teleporting) with zero loading screens.

The world is divided into distinct areas with their own rules, but these are logical zones, not separate scenes.

```typescript
interface AreaDefinition {
  id: AreaType
  displayName: string
  currency: ResourceType
  secondaryCurrency?: ResourceType
  toolRequired: ToolType
  fogColor: number
  ambientColor: number
  groundTexture: string
  spawnableResources: ResourceType[]
  spawnableAnimals: AnimalType[]
}

const AREAS: Record<AreaType, AreaDefinition> = {
  spring: {
    id: 'spring',
    displayName: 'Spring Grove',
    currency: 'wood',
    toolRequired: 'axe',
    fogColor: 0x90EE90,
    // ...
  },
  farm: {
    id: 'farm',
    displayName: 'Animal Farm',
    currency: 'meat',
    toolRequired: 'sword',
    fogColor: 0xFFD700,
    spawnableAnimals: ['chicken', 'pig', 'cow'],
    // ...
  },
  volcano: {
    id: 'volcano',
    displayName: 'Volcano Crater',
    currency: 'crystal',
    toolRequired: 'pickaxe',
    fogColor: 0xFF4500,
    spawnableAnimals: ['bat'],
    // ...
  },
  // ... etc
}
```

### 4. Single Delta Time

One `worldTimeScale` controls game speed. When paused, everything freezes except debug camera.

```typescript
GameClock {
  rawDelta: number          // Actual frame time (capped at 100ms)
  delta: number             // rawDelta * timeScale (0 when paused)
  timeScale: number         // Default 1.0, debug controls this
  isPaused: boolean         // Stops delta entirely
  
  // Unscaled delta for UI/debug that must always run
  uiDelta: number           // Always equals rawDelta
}
```

### 5. Collision System

Spatial hash rebuilt once per frame. Systems query it during their updates.

```typescript
CollisionSystem {
  // Called once at start of frame
  rebuildHash(entities: Entity[])
  
  // Called by movement code
  queryMove(entity: Entity, desiredPos: Vector3): CollisionResult
  
  // Returns resolved position that doesn't overlap
  resolve(entity: Entity, desiredPos: Vector3, hits: Entity[]): Vector3
}
```

**Collision Rules:**
| Entity | Blocks | Blocked By |
|--------|--------|------------|
| Player | Players | Players, Resources, Animals, Structures, Cliffs |
| Animal | Animals | Animals, Resources, Structures, Cliffs, Water |
| Boat | Boats | Boats, River banks |
| Resource | All | — (static) |
| Structure | All | — (static) |

### 6. Event Bus

Only for discrete state changes with multiple listeners. Never for per-frame data.

```typescript
// Good
eventBus.emit('resource:harvested', { resourceId, playerId, yield, area })
eventBus.emit('animal:slaughtered', { animalId, playerId, meatAmount })
eventBus.emit('story:stampede:triggered', { triggerPlayerId })
eventBus.emit('story:eruption:triggered', { triggerPlayerId })
eventBus.emit('area:entered', { playerId, fromArea, toArea })
eventBus.emit('boat:collision', { boat1Id, boat2Id })

// Bad - use direct calls instead
eventBus.emit('player:moved', position)  // ❌ Per-frame
eventBus.emit('boat:drifting', position) // ❌ Per-frame
```

### 7. Update Order

```typescript
Game.update(rawDelta) {
  // 1. Timing
  clock.update(rawDelta)
  
  // 2. Input
  input.update()
  
  // 3. Debug (can modify clock, handle save/load)
  debug.update(clock)
  
  // 4. Collision hash rebuild
  collision.rebuildHash(entities.all())
  
  // 5. Gameplay (skipped when paused)
  if (!clock.isPaused) {
    // Player & interaction
    systems.player.update(clock.delta)
    systems.interaction.update(clock.delta)
    
    // World simulation
    systems.resource.update(clock.delta)
    systems.animal.update(clock.delta)
    systems.boat.update(clock.delta)
    systems.river.update(clock.delta)
    
    // NPCs & Story
    systems.shopkeeper.update(clock.delta)
    systems.storyEvent.update(clock.delta)
    
    // Special areas
    systems.cloudParkour.update(clock.delta)
    systems.lava.update(clock.delta)
    
    // Structures
    systems.planting.update(clock.delta)
    systems.catapult.update(clock.delta)
    systems.teleporter.update(clock.delta)
  }
  
  // 6. Camera (always runs for Free Cam)
  systems.camera.update(clock.isPaused ? clock.uiDelta : clock.delta)
  
  // 7. Cutscene (can override camera, freeze gameplay)
  systems.cutscene.update(clock.uiDelta)
  
  // 8. Render
  scene.render()
  
  // 9. UI sync
  syncToZustand()
}
```

---

## Folder Structure

```
src/
├── main.tsx                    # React entry point
├── App.tsx                     # React root, renders game canvas + UI
├── Game.ts                     # Master game class
├── Constants.ts                # ALL game values (no magic numbers)
├── Types.ts                    # TypeScript interfaces
│
├── core/                       # Engine fundamentals
│   ├── Clock.ts                # GameClock
│   ├── EventBus.ts             # Pub/sub
│   ├── Input.ts                # Gamepad (1-4) + keyboard
│   ├── Collision.ts            # Spatial hash + queries
│   └── Utils.ts                # Math, ID generation
│
├── systems/                    # Gameplay logic
│   ├── PlayerSystem.ts         # Movement, harvesting, tool usage
│   ├── InteractionSystem.ts    # E-to-interact, context actions
│   ├── ResourceSystem.ts       # Trees, cactus, crystals, harvesting
│   ├── AnimalSystem.ts         # All animal AI, breeding, slaughtering
│   ├── BoatSystem.ts           # River travel, drifting, collisions
│   ├── RiverSystem.ts          # Water flow, direction changes
│   ├── ShopkeeperSystem.ts     # NPC shopkeepers, dialogue
│   ├── ShopSystem.ts           # Trading, purchases
│   ├── PlantingSystem.ts       # Saplings, growth, elixir
│   ├── StoryEventSystem.ts     # Stampede, eruption triggers
│   ├── CutsceneSystem.ts       # Full-screen cutscene playback
│   ├── CloudParkourSystem.ts   # Moving/rotating/disappearing clouds
│   ├── CatapultSystem.ts       # Launch mechanics
│   ├── TeleporterSystem.ts     # Fast travel
│   ├── LavaSystem.ts           # Lava pools, water-to-stone
│   ├── BatSystem.ts            # Volcano hazard
│   ├── SpiderSystem.ts         # Tunnel blocker
│   └── CameraSystem.ts         # All camera modes
│
├── entities/                   # Entity management
│   ├── EntityManager.ts        # Create, destroy, query
│   └── EntityFactory.ts        # Spawn helpers with correct data
│
├── world/                      # World setup
│   ├── AreaManager.ts          # Area definitions, transitions
│   ├── WorldGenerator.ts       # Full world generation
│   ├── SeasonalArea.ts         # 4 seasonal starting zones
│   ├── FarmArea.ts             # Central animal farm
│   ├── BeachDesertArea.ts      # Beach/desert zone
│   ├── MountainArea.ts         # Mountain zone
│   ├── VolcanoArea.ts          # Volcano interior
│   ├── EtherealArea.ts         # Ethereal realm
│   └── RiverGenerator.ts       # River channels between areas
│
├── rendering/                  # Graphics
│   ├── Scene.ts                # Babylon setup, splitscreen (1-4)
│   ├── N64Pipeline.ts          # Post-process, palette, fog
│   └── procedural/             # All procedural assets
│       ├── Geometry.ts         # Vertex-by-vertex mesh building
│       ├── Textures.ts         # Pixel-by-pixel textures
│       ├── Palettes.ts         # 16-color palettes per area
│       ├── Animation.ts        # Segmented limb keyframes
│       └── Models.ts           # All model definitions
│
├── audio/                      # Sound
│   └── Audio.ts                # Sound effects, ambient per area
│
├── debug/                      # Debug infrastructure
│   ├── Debug.ts                # Master debug manager
│   ├── SaveState.ts            # LocalStorage save/load
│   ├── TimeControl.ts          # Speed controls
│   ├── FreeCam.ts              # Free camera controller
│   └── DevVis.ts               # Collision boxes, debug lines
│
└── ui/                         # React components
    ├── store.ts                # Zustand store
    ├── GameUI.tsx              # Container for all UI
    ├── HUD.tsx                 # Per-player HUD (resources, tool, area)
    ├── ShopMenu.tsx            # Shop interface
    ├── Dialogue.tsx            # Shopkeeper text
    ├── MainMenu.tsx            # Player count select
    ├── PauseMenu.tsx           # Pause overlay
    ├── AreaTransition.tsx      # Area name popup
    ├── ControlsBar.tsx         # Bottom debug strip
    ├── Notifications.tsx       # Toast messages
    ├── RadialMenu.tsx          # Camera mode selector
    ├── SpawnMenu.tsx           # Debug entity spawner
    └── DebugOverlay.tsx        # FPS, position, entity info
```

---

## Build Order

Systems at the top have no dependencies. Each level depends on levels above.

```
Level 0: Foundation
├── Constants.ts
├── Types.ts
└── Utils.ts

Level 1: Core
├── Clock.ts
├── EventBus.ts
├── Input.ts (1-4 controllers)
└── Collision.ts

Level 2: Rendering Base
├── Palettes.ts
├── Geometry.ts
├── Textures.ts
└── Scene.ts (Babylon setup, splitscreen 1-4)

Level 3: Assets
├── Animation.ts
├── Models.ts (all procedural models)
└── N64Pipeline.ts

Level 4: Entities
├── EntityManager.ts
└── EntityFactory.ts

Level 5: Areas
├── AreaManager.ts
├── SeasonalArea.ts
├── FarmArea.ts
├── BeachDesertArea.ts
├── MountainArea.ts
├── VolcanoArea.ts
├── EtherealArea.ts
└── RiverGenerator.ts

Level 6: World
└── WorldGenerator.ts (combines all areas)

Level 7: Core Systems
├── PlayerSystem.ts
├── InteractionSystem.ts
├── ResourceSystem.ts
├── CameraSystem.ts
└── RiverSystem.ts

Level 8: Entity Systems
├── AnimalSystem.ts
├── BoatSystem.ts
├── PlantingSystem.ts
└── ShopkeeperSystem.ts

Level 9: Special Systems
├── BatSystem.ts
├── SpiderSystem.ts
├── LavaSystem.ts
├── CatapultSystem.ts
├── TeleporterSystem.ts
└── CloudParkourSystem.ts

Level 10: Shop & Economy
└── ShopSystem.ts

Level 11: Story
├── StoryEventSystem.ts
└── CutsceneSystem.ts

Level 12: Debug
├── SaveState.ts
├── TimeControl.ts
├── FreeCam.ts
├── DevVis.ts
└── Debug.ts (coordinator)

Level 13: Audio
└── Audio.ts

Level 14: UI
├── store.ts (Zustand)
└── All React components

Level 15: Game
└── Game.ts (orchestrates everything)
```

---

## Animal AI State Machine

Animals are the most dynamic entities. They use a state machine.

```
┌─────────────────────────────────────────────────────────┐
│              FARM ANIMAL STATES (Chicken/Pig/Cow)        │
└─────────────────────────────────────────────────────────┘

    ┌──────────┐
    │  IDLE    │ ← Start here
    └────┬─────┘
         │ Random chance OR player nearby
         ▼
    ┌──────────┐     ┌──────────┐
    │ WANDERING│ ←──→│ FLEEING  │ ← Player too close
    └────┬─────┘     └──────────┘
         │ Fed hay + partner nearby
         ▼
    ┌──────────┐
    │ BREEDING │ ← Two animals of same type, both fed
    └────┬─────┘
         │ Timer complete
         ▼
    (Spawn baby, back to IDLE)


┌─────────────────────────────────────────────────────────┐
│              BAT STATES (Volcano hazard)                 │
└─────────────────────────────────────────────────────────┘

    ┌──────────┐
    │  IDLE    │ ← Hanging on ceiling
    └────┬─────┘
         │ Player looks at bat
         ▼
    ┌──────────┐
    │ CHASING  │ ← Flying toward player
    └────┬─────┘
         │ Reaches player OR player leaves volcano OR hit by coconut
         ▼
    (Steal crystals / Die / Return to IDLE)


┌─────────────────────────────────────────────────────────┐
│              BEACH ANIMAL STATES (Post-stampede)         │
└─────────────────────────────────────────────────────────┘

    ┌──────────┐
    │ RELAXING │ ← On beach chair, sunglasses, pina colada
    └──────────┘
    (Never moves, purely decorative)
```

**Animal Data:**
```typescript
interface AnimalData {
  animalType: 'chicken' | 'pig' | 'cow' | 'bat'
  state: AnimalState
  fedHay: boolean
  isRelaxing: boolean
  targetPlayer: string | null
  wanderTarget: Vector3 | null
  stateTimer: number
}
```

---

## Boat System

Boats are player-controlled water vehicles. They do NOT drift with river current.

```
┌─────────────────────────────────────────────────────────┐
│                    BOAT LIFECYCLE                        │
└─────────────────────────────────────────────────────────┘

    ┌──────────┐
    │ FLOATING │ ← Player places boat in water (left hand item)
    └────┬─────┘
         │ Player right-clicks (feed button) to enter
         ▼
    ┌──────────┐
    │ OCCUPIED │ ← Player steering with W + look direction
    └────┬─────┘
         │ Collision with other boat
         ▼
    ┌──────────┐
    │ STOPPED  │ ← Both boats stopped (easier jumping)
    └────┬─────┘
         │ Player jumps out (A button)
         ▼
    ┌──────────┐
    │ SINKING  │ ← 5 second timer starts when player exits
    └────┬─────┘
         │ Timer expires
         ▼
    (Boat destroyed)
```

**Boat Controls:**
```typescript
// Enter boat: Right-click (feed button) when near a floating boat
// Steer: W key moves forward in camera look direction
// Exit: Jump button (A / spacebar)

interface BoatData {
  state: 'floating' | 'occupied' | 'sinking'
  occupantId: string | null      // Player ID currently in boat
  sinkTimer: number              // Countdown after player exits (5 seconds)
}
```

**Note:** River flow affects PLAYERS swimming in water (they get pushed by current), but boats stay stationary until the player steers them.

---

## Story Event System

Two major story events drive progression.

### Stampede Event

```
TRIGGER: Player attempts to buy GUN at Farm shop

┌─────────────────────────────────────────────────────────┐
│                  STAMPEDE SEQUENCE                       │
└─────────────────────────────────────────────────────────┘

1. Shopkeeper: "Sure, let me just get that for ya..."
2. Animals detect gun purchase attempt
3. CUTSCENE STARTS (full screen)
4. Animals STAMPEDE
5. Shopkeeper runs toward beach tunnel
6. Shopkeeper tries to break wooden barrier
7. Animals trample shopkeeper (becomes floor texture)
8. Animals break barrier (beach access unlocked)
9. CUTSCENE ENDS
10. Some animals spawn on beach chairs (relaxing state)
11. Start 5-minute timer for nephew shopkeeper

STATE CHANGES:
- storyFlags.stampedeTriggered = true
- storyFlags.beachAccessUnlocked = true
- farmShopkeeper.isAlive = false
- Spawn relaxing animals on beach
- Remove wooden barrier entity
- Start nephew spawn timer
```

### Eruption Event

```
TRIGGER: Player buys CRYSTAL MAGNET at Volcano shop

┌─────────────────────────────────────────────────────────┐
│                  ERUPTION SEQUENCE                       │
└─────────────────────────────────────────────────────────┘

1. Volcano shakes (screen shake)
2. Shopkeeper: "OH NO, WE'VE DISTURBED THE VOLCANO'S PEACE"
3. CUTSCENE STARTS
4. Volcano erupts
5. Shop hut launches into sky (player on it)
6. Player + shopkeeper land on cloud parkour start
7. Shopkeeper attempts parkour, falls off
8. CUTSCENE ENDS
9. Player in cloud parkour area
10. One bat becomes new volcano shopkeeper

STATE CHANGES:
- storyFlags.eruptionTriggered = true
- storyFlags.parkourAccessUnlocked = true
- volcanoShopkeeper.isAlive = false
- Remove all bats except one
- Convert remaining bat to shopkeeper
- Teleport player to parkour start
- Enable cloud parkour system
```

---

## Cloud Parkour System

Special platforming section between volcano and ethereal realm.

```typescript
interface CloudPlatformData {
  platformType: 'static' | 'disappearing' | 'moving' | 'rotating'
  
  // Disappearing clouds
  visibleTime: number
  hiddenTime: number
  currentPhase: 'visible' | 'hidden'
  phaseTimer: number
  
  // Moving clouds
  movePath: Vector3[]
  moveSpeed: number
  currentPathIndex: number
  
  // Rotating clouds
  rotationSpeed: number
}

function updateCloudParkour(entities: Entity[], delta: number) {
  for (const cloud of entities.filter(e => e.type === 'cloudPlatform')) {
    switch (cloud.platformType) {
      case 'disappearing':
        cloud.phaseTimer -= delta
        if (cloud.phaseTimer <= 0) {
          cloud.currentPhase = cloud.currentPhase === 'visible' ? 'hidden' : 'visible'
          cloud.phaseTimer = cloud.currentPhase === 'visible' ? cloud.visibleTime : cloud.hiddenTime
          cloud.mesh.isVisible = cloud.currentPhase === 'visible'
        }
        break
        
      case 'moving':
        // Move along path
        const target = cloud.movePath[cloud.currentPathIndex]
        const dir = target.subtract(cloud.position).normalize()
        cloud.position.addInPlace(dir.scale(cloud.moveSpeed * delta))
        if (cloud.position.distanceTo(target) < 0.5) {
          cloud.currentPathIndex = (cloud.currentPathIndex + 1) % cloud.movePath.length
        }
        break
        
      case 'rotating':
        cloud.rotation += cloud.rotationSpeed * delta
        break
    }
  }
}
```

**Falling Detection:**
```typescript
function checkParkourFall(player: Entity) {
  if (player.position.y < PARKOUR_DEATH_PLANE) {
    // Respawn at parkour start
    player.position = PARKOUR_START_POSITION.clone()
    notify("Back to the start...")
  }
}
```

---

## Collision System

### Spatial Hash

Grid divides world into cells. Only check entities in same/adjacent cells.

```
Cell Size: 10 units

┌─────┬─────┬─────┐
│  A  │     │  B  │
├─────┼─────┼─────┤
│     │  X  │  C  │
├─────┼─────┼─────┤
│     │  D  │     │
└─────┴─────┴─────┘

To check X: only test against C, D (adjacent cells)
Skip A, B (too far)
```

### Collider Shapes

All entities use **cylinder** colliders (radius + height).

```typescript
interface ColliderDef {
  radius: number
  height: number
  isStatic: boolean
}

const COLLIDERS = {
  PLAYER: { radius: 0.5, height: 2, isStatic: false },
  CHICKEN: { radius: 0.3, height: 0.5, isStatic: false },
  PIG: { radius: 0.5, height: 0.8, isStatic: false },
  COW: { radius: 0.7, height: 1.2, isStatic: false },
  BAT: { radius: 0.3, height: 0.3, isStatic: false },
  TREE: { radius: 0.8, height: 5, isStatic: true },
  CACTUS: { radius: 0.4, height: 2, isStatic: true },
  CRYSTAL: { radius: 0.3, height: 1, isStatic: true },
  BOAT: { radius: 1.5, height: 1, isStatic: false },
  SHOPKEEPER: { radius: 0.5, height: 2, isStatic: true },
}
```

---

## Save State System

### What Gets Saved

```typescript
interface SaveState {
  version: number
  timestamp: number
  
  // Player states (1-4 players)
  players: PlayerSaveData[]
  
  // World state
  resources: ResourceSaveData[]
  animals: AnimalSaveData[]
  saplings: SaplingSaveData[]
  boats: BoatSaveData[]
  
  // Structures
  structures: StructureSaveData[]
  
  // Story progress
  storyFlags: {
    stampedeTriggered: boolean
    beachAccessUnlocked: boolean
    nephewSpawned: boolean
    eruptionTriggered: boolean
    parkourCompleted: boolean
    teleporterBuilt: boolean
  }
  
  // Shopkeeper states
  shopkeepers: ShopkeeperSaveData[]
  
  // Area states
  spiderScared: boolean
  wallCrackMined: boolean
}

interface PlayerSaveData {
  playerIndex: number
  position: Vector3
  rotation: number
  currentArea: AreaType
  inventory: ResourceInventory
  capacity: number
  tools: PlayerTools
  hasBoatHouse: boolean
  hasCactusArmor: boolean
  hasTeleporter: boolean
}
```

---

## Constants Structure

```typescript
// Constants.ts

// =============================================================================
// GAME
// =============================================================================
export const GAME = {
  TARGET_FPS: 30,
  MAX_DELTA: 0.1,
  MAX_PLAYERS: 4,
}

// =============================================================================
// AREAS
// =============================================================================
export const AREAS = {
  SPRING: { position: { x: -100, z: -100 }, radius: 50 },
  SUMMER: { position: { x: 100, z: -100 }, radius: 50 },
  FALL: { position: { x: 100, z: 100 }, radius: 50 },
  WINTER: { position: { x: -100, z: 100 }, radius: 50 },
  FARM: { position: { x: 0, z: 0 }, radius: 40 },
  BEACH_DESERT: { position: { x: -200, z: 0 }, radius: 80 },
  MOUNTAIN: { position: { x: 200, z: 0 }, radius: 60 },
  VOLCANO: { position: { x: 250, z: 50 }, radius: 40 },
  ETHEREAL: { position: { x: 0, z: 0, y: 500 }, radius: 100 },
}

// =============================================================================
// RESOURCES
// =============================================================================
export const RESOURCES = {
  TREE: {
    variants: 3,
    yields: [1, 2, 3],  // Per variant
    swingsBase: 10,
    rottenChance: 0.1,
  },
  CACTUS: {
    variants: 3,
    yields: [1, 2, 3],
  },
  CRYSTAL: {
    variants: 3,
    yields: [1, 2, 3],
  },
  STONE: {
    yields: [1],
  },
  CLOUD_TREE: {
    variants: 1,
    yields: [2],
  },
  COCONUT: {
    dropAmount: 3,
  },
}

// =============================================================================
// TOOLS
// =============================================================================
export const TOOLS = {
  AXE: {
    WOOD: { swings: 10, cost: { wood: 0 } },
    STONE: { swings: 6, cost: { wood: 50 } },
    IRON: { swings: 4, cost: { wood: 100 } },
    STEEL: { swings: 3, cost: { wood: 150 } },
  },
  SWORD: {
    WOOD: { swings: 5, cost: { meat: 0 } },
    STONE: { swings: 3, cost: { meat: 30 } },
    IRON: { swings: 2, cost: { meat: 60 } },
    STEEL: { swings: 1, cost: { meat: 100 } },
  },
  MACHETE: { cost: { cactus: 20 } },
  CHISEL: { cost: { stone: 20 } },
  PICKAXE: { cost: { stone: 50 } },
  ETHEREAL_AXE: { cost: { clouds: 30 } },
}

// =============================================================================
// ANIMALS
// =============================================================================
export const ANIMALS = {
  CHICKEN: { meatYield: 1, speed: 3 },
  PIG: { meatYield: 2, speed: 4 },
  COW: { meatYield: 3, speed: 2 },
  BAT: { speed: 8, detectionRange: 10 },
  BREED_TIME: 10,  // Seconds
  RESPAWN_DELAY: 15,  // Seconds after extinction
  RESPAWN_COUNT: 2,
}

// =============================================================================
// BOATS
// =============================================================================
export const BOAT = {
  SPEED: 5,
  SINK_TIME: 5,
  HOUSE_COST: { wood: 500 },
  BOAT_COST: { wood: 20 },
}

// =============================================================================
// STRUCTURES
// =============================================================================
export const STRUCTURES = {
  CATAPULT: { cost: { stone: 3000, wood: 1000, meat: 1000 } },
  TELEPORTER: { cost: { clouds: 200 } },
  CACTUS_ARMOR: { cost: { cactus: 300, coconuts: 300 } },
}

// =============================================================================
// STORY
// =============================================================================
export const STORY = {
  NEPHEW_SPAWN_DELAY: 300,  // 5 minutes in seconds
  GUN_COST: { meat: 999 },  // Never actually purchasable
  CRYSTAL_MAGNET_COST: { crystal: 100 },
}

// =============================================================================
// PLAYER
// =============================================================================
export const PLAYER = {
  MOVE_SPEED: 8,
  HARVEST_RANGE: 2.5,
  STARTING_CAPACITY: 50,
}

// =============================================================================
// CAPACITY
// =============================================================================
export const CAPACITY = {
  DEFAULT: 50,
  BACKPACK: { amount: 100, cost: { wood: 75 } },
}

// =============================================================================
// CAMERA
// =============================================================================
export const CAMERA = {
  FOV: 60,
  NORMAL_OFFSET: { x: 0, y: 8, z: -12 },
  FIRST_PERSON_OFFSET: { x: 0, y: 1.7, z: 0 },
  FREE_SPEED: 16,
}

// =============================================================================
// RENDER (N64 style)
// =============================================================================
export const RENDER = {
  WIDTH: 320,
  HEIGHT: 240,
  PALETTE_SIZE: 16,
}

// =============================================================================
// CLOUD PARKOUR
// =============================================================================
export const PARKOUR = {
  DISAPPEAR_VISIBLE_TIME: 3,
  DISAPPEAR_HIDDEN_TIME: 2,
  MOVE_SPEED: 2,
  ROTATE_SPEED: 0.5,
  DEATH_PLANE_Y: -50,
}

// =============================================================================
// DEBUG
// =============================================================================
export const DEBUG = {
  ENABLED: true,
  SKIP_MAIN_MENU: true,
  DEFAULT_PLAYERS: 1,
  SHOW_CONTROLS_BAR: true,
  LOG_EVENTS: false,
}
```

---

## React Integration

### Zustand Store

```typescript
interface Store {
  // Game state
  screen: 'menu' | 'playing' | 'paused' | 'cutscene'
  playerCount: 1 | 2 | 3 | 4
  
  // Per-player data (UI-relevant only)
  players: {
    inventory: ResourceInventory
    currentArea: AreaType
    equippedTool: ToolType
  }[]
  
  // Story progress
  storyFlags: StoryFlags
  
  // Debug
  timeScale: number
  cameraMode: CameraMode
  showDevVis: boolean
  
  // Notifications
  notifications: { id: string, text: string }[]
  
  // Actions
  sync: (data: Partial<Store>) => void
  notify: (text: string) => void
}
```

### Data Flow

```
Game (owns truth)
    │
    │ syncToZustand() each frame
    ▼
Zustand Store (read-only mirror)
    │
    │ React reads via hooks
    ▼
UI Components (display only)
    │
    │ User clicks button
    ▼
EventBus.emit('ui:action', data)
    │
    │ Game listens
    ▼
Game (processes action)
```

---

## Implementation Phases

### Phase 1: Skeleton
```
1. Vite + React + Babylon.js setup
2. Constants.ts, Types.ts
3. Clock.ts, EventBus.ts
4. Scene.ts (single camera)
5. Basic Game.ts shell
6. Verify: Empty 3D scene renders
```

### Phase 2: Debug Infrastructure
```
1. Debug.ts, TimeControl.ts
2. ControlsBar.tsx, Notifications.tsx
3. SaveState.ts
4. Verify: Can pause, save/load
```

### Phase 3: Input & Camera
```
1. Input.ts (1-4 controllers)
2. CameraSystem.ts (3rd person, 1st person, free cam)
3. Splitscreen rendering (1-4 viewports)
4. Verify: 4 players can look around
```

### Phase 4: Collision & Entities
```
1. Collision.ts (spatial hash)
2. EntityManager.ts, EntityFactory.ts
3. Basic cylinder colliders
4. Verify: Entities block each other
```

### Phase 5: Procedural Assets
```
1. Palettes.ts, Geometry.ts, Textures.ts
2. Models.ts shell
3. Build trees, characters, animals
4. Verify: N64-style models render
```

### Phase 6: World Generation
```
1. AreaManager.ts
2. All area generators (Seasonal, Farm, Beach, Mountain, Volcano, Ethereal)
3. RiverGenerator.ts
4. WorldGenerator.ts
5. Verify: Complete world exists
```

### Phase 7: Player Movement
```
1. PlayerSystem.ts
2. Movement with collision
3. Area transition detection
4. Verify: Players can walk between areas
```

### Phase 8: Resource Harvesting
```
1. ResourceSystem.ts
2. InteractionSystem.ts
3. Tool-based harvesting
4. Inventory management
5. Verify: Can chop trees, harvest cactus
```

### Phase 9: Animals
```
1. AnimalSystem.ts
2. Wandering, fleeing AI
3. Slaughtering for meat
4. Breeding mechanics
5. Verify: Farm animals work
```

### Phase 10: Boats & River
```
1. BoatSystem.ts
2. RiverSystem.ts
3. River flow, drifting
4. Boat collisions
5. Verify: Can travel via river
```

### Phase 11: Shopkeepers & Trading
```
1. ShopkeeperSystem.ts
2. ShopSystem.ts
3. ShopMenu.tsx
4. Per-area shops with local currency
5. Verify: Can buy/sell at shops
```

### Phase 12: Planting
```
1. PlantingSystem.ts
2. Sapling placement
3. Growth timers
4. Growth elixir
5. Verify: Can plant and grow resources
```

### Phase 13: Story Events
```
1. StoryEventSystem.ts
2. CutsceneSystem.ts
3. Stampede sequence
4. Eruption sequence
5. Verify: Full story events play
```

### Phase 14: Special Systems
```
1. BatSystem.ts (volcano hazard)
2. SpiderSystem.ts (tunnel blocker)
3. CatapultSystem.ts (volcano access)
4. TeleporterSystem.ts (fast travel)
5. LavaSystem.ts (water-to-stone)
6. CloudParkourSystem.ts
7. Verify: All special mechanics work
```

### Phase 15: Polish
```
1. All UI components
2. N64Pipeline.ts (visual post-process)
3. Audio.ts
4. Full save/load testing
5. Performance optimization
```

---

## Key Invariants

1. **Clock is the only source of delta.** No `Date.now()` in game logic.
2. **Collision before movement.** Query, resolve, then apply.
3. **EventBus for state changes only.** Never per-frame.
4. **Constants.ts for all values.** No magic numbers.
5. **UI reads, never writes.** Emit events, game processes.
6. **Debug behind flags.** All debug respects `DEBUG.ENABLED`.
7. **All assets procedural.** No imports.
8. **Each area owns its currency.** No universal money.
9. **Resources finite but renewable.** Saplings enable regrowth.
10. **Story events are one-way.** Once triggered, permanent.
11. **No load zones.** Entire world is one seamless scene, all areas loaded at once.

---

## Anti-Patterns

| Don't | Do Instead |
|-------|------------|
| `setTimeout`/`setInterval` | Clock-based timers |
| Scattered globals | Game owns everything |
| EventBus for positions | Direct system calls |
| Hardcoded `3.0` | `SWING_DURATION` constant |
| Collision after move | Collision before move |
| UI mutates state | UI emits events |
| Universal currency | Per-area currencies |
| Placeholder assets | Procedural from start |
| "Fix later" | Fix now or don't build |
