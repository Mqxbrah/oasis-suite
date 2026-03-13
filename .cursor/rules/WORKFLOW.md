# WORKFLOW.md - Development Processes

## Important: Testing Responsibility

**The user handles all in-game testing.** As the AI, you:
- Write correct, complete code the first time
- Do NOT open browser or attempt to playtest
- Do NOT use browser dev tools unless specifically checking for build errors
- Rely on linter errors and build output for validation
- Trust the architecture — if you follow these workflows, the code will work

---

## Quick Reference

| Task | Section |
|------|---------|
| Add/change a game value | [Constants](#constants) |
| Create a 3D model | [Models](#models) |
| Add new entity type | [Entities](#entities) |
| Add shopkeeper dialogue | [Dialogue](#dialogue) |
| Add shop item | [Shop Items](#shop-items) |
| Add debug feature | [Debug Features](#debug-features) |
| Add UI element | [UI Components](#ui-components) |
| Add/modify animal behavior | [Animal AI](#animal-ai) |
| Add new area | [Areas](#areas) |
| Add story event | [Story Events](#story-events) |
| Balance gameplay | [Balancing](#balancing) |

---

## Before Any Change

1. **Read relevant existing code** — Understand what's there
2. **Check Constants.ts** — Does the value exist? Add it if not
3. **Check Types.ts** — Does the type exist? Extend it if needed
4. **Identify the owner** — One system owns each piece of data
5. **Consider save state** — Does this need to persist?
6. **Check area context** — Which area(s) does this affect?

---

## Constants

Every number in the game comes from `Constants.ts`. No exceptions.

### Adding a Value

```typescript
// Constants.ts - find the right section
export const PLAYER = {
  MOVE_SPEED: 8,
  HARVEST_RANGE: 2.5,
  // Add new value with clear name
  SWIM_SPEED: 4,  // For river swimming (if added)
}
```

### Using a Value

```typescript
import { PLAYER } from '../Constants'

// ✅ Correct
if (distance <= PLAYER.HARVEST_RANGE) { ... }

// ❌ Wrong - magic number
if (distance <= 2.5) { ... }
```

### Sections in Constants.ts

| Section | Contains |
|---------|----------|
| `GAME` | FPS, delta cap, max players |
| `AREAS` | Position, radius per area |
| `RESOURCES` | Variants, yields, spawn weights |
| `TOOLS` | Swings and costs per tool tier |
| `ANIMALS` | Speeds, meat yields, breed times |
| `BOAT` | Speed, sink time, costs |
| `STRUCTURES` | Costs for buildables |
| `STORY` | Event timers, trigger costs |
| `PLAYER` | Speed, range, starting values |
| `CAPACITY` | Inventory limits and costs |
| `CAMERA` | FOV, offsets, speeds |
| `RENDER` | Resolution, palette size |
| `PARKOUR` | Cloud platform timings |
| `DEBUG` | Flags (enabled, skip menu, logging) |

---

## Models

All models are procedural. Built vertex-by-vertex, textured pixel-by-pixel.

### Process

1. **Colors** → Add palette in `Palettes.ts` (max 16 colors)
2. **Shape** → Use helpers in `Geometry.ts` or add new ones
3. **Texture** → Generate in `Textures.ts`
4. **Assembly** → Combine in `Models.ts`
5. **Register** → Add to `Models.init()`

### Example: New Resource

```typescript
// Palettes.ts
CRYSTAL: [0x9400D3, 0xBA55D3, 0xDA70D6, 0xEE82EE],

// Models.ts
function buildCrystal(variant: number): Mesh {
  const height = 0.5 + variant * 0.3
  const base = Geometry.cone(0.2, height, 6)
  const top = Geometry.cone(0.1, height * 0.5, 6)
  top.position.y = height
  
  const mesh = Mesh.mergeMeshes([base, top])
  mesh.material = Materials.fromPalette(PALETTES.CRYSTAL)
  return mesh
}

// In Models.init()
for (let v = 1; v <= 3; v++) {
  this.crystals[v] = buildCrystal(v)
}
```

### Character Models

All characters share the same structure:
- Head (cube with face)
- Torso (box)
- Arms (cylinders, attached at shoulders)
- Hips (box)
- Legs (cylinders)
- Accessories (hats, items vary by role)

Differentiate by:
- Clothing color (varies by role)
- Accessories (shopkeepers have unique items per area)
- Scale (some NPCs larger/smaller)

### Animal Models

| Animal | Structure |
|--------|-----------|
| Chicken | Small body, beak, wings, legs |
| Pig | Round body, snout, curly tail, short legs |
| Cow | Large body, horns, udder, four legs |
| Bat | Small body, large wings, fangs |

---

## Entities

Entities are typed objects. Each type has specific data.

### Current Entity Types

| Type | Data | System |
|------|------|--------|
| `player` | inventory, tools, area, capacity | PlayerSystem |
| `resource` | type, variant, size, progress | ResourceSystem |
| `animal` | type, state, fedHay, target | AnimalSystem |
| `boat` | passengers, position, sinking | BoatSystem |
| `shopkeeper` | type, alive, dialogue | ShopkeeperSystem |
| `structure` | type, built, owner | StructureSystem |
| `sapling` | type, growthProgress | PlantingSystem |
| `cloudPlatform` | platformType, timers | CloudParkourSystem |
| `stump` | sinkProgress | ResourceSystem |

### Adding New Entity Type

1. **Define data** in `Types.ts`:
```typescript
export interface NewEntityData {
  state: 'idle' | 'active'
  value: number
}
```

2. **Extend Entity** in `Types.ts`:
```typescript
export interface Entity {
  // ... existing
  newEntity?: NewEntityData
}
```

3. **Add to EntityType**:
```typescript
export type EntityType = 'player' | 'resource' | ... | 'newEntity'
```

4. **Create factory** in `EntityFactory.ts`:
```typescript
export function createNewEntity(position: Vector3, area: AreaType): Entity {
  return {
    id: generateId('newentity'),
    type: 'newEntity',
    position,
    rotation: 0,
    area,
    mesh: Models.clone('newEntity', position),
    newEntity: { state: 'idle', value: 0 },
  }
}
```

5. **Add collider** in `Constants.ts`:
```typescript
COLLIDERS: {
  NEW_ENTITY: { radius: 0.5, height: 1.0 },
}
```

6. **Create system** if it has behavior:
```typescript
// systems/NewEntitySystem.ts
export class NewEntitySystem {
  update(entities: Entity[], delta: number, collision: Collision) {
    for (const e of entities) {
      if (!e.newEntity) continue
      // Behavior here
    }
  }
}
```

7. **Register in Game.ts**:
```typescript
this.systems.newEntity = new NewEntitySystem()
// In update: this.systems.newEntity.update(...)
```

8. **Add to save state** in `SaveState.ts`

---

## Dialogue

Shopkeeper dialogue is per-area with unique personalities.

### Adding Dialogue Lines

```typescript
// In Dialogue.ts or Constants.ts
export const SHOPKEEPER_LINES = {
  // Per-area greetings
  SEASONAL: {
    GREETING: ["Need wood?", "Trees are plentiful.", "Welcome, traveler."],
    TRADE: ["Good trade.", "Come again."],
    CANT_AFFORD: ["Not enough wood.", "Gather more first."],
  },
  FARM: {
    GREETING: ["Fresh meat today?", "Animals are restless."],
    GUN_ATTEMPT: ["Sure, let me just get that for ya..."],
  },
  FARM_NEPHEW: {
    GREETING: ["I travelled here to work with my uncle, but I can't find him. Have you seen him?"],
    SUBSEQUENT: ["Still no sign of him...", "The farm is mine now, I guess."],
  },
  BEACH: {
    GREETING: ["Cactus or coconuts?", "Watch out for spiders."],
  },
  MOUNTAIN: {
    GREETING: ["Stone is valuable here.", "The volcano rumbles..."],
  },
  VOLCANO: {
    GREETING: ["Mind the lava.", "Crystals glow bright today."],
    ERUPTION: ["OH NO, WE'VE DISTURBED THE VOLCANO'S PEACE"],
  },
  VOLCANO_BAT: {
    GREETING: ["*screech*", "*flap flap*"],  // Bat noises
  },
  CLOUD_BEAR: {
    GREETING: ["*happy bear sounds*", "Clouds are soft."],
  },
}
```

### Triggering Dialogue

```typescript
function getShopkeeperGreeting(shopkeeper: Entity): string {
  const lines = SHOPKEEPER_LINES[shopkeeper.shopkeeper.shopkeeperType]
  return randomFrom(lines.GREETING)
}
```

---

## Shop Items

Items purchased at area shopkeepers using local currency.

### Adding Shop Item

1. **Add constant**:
```typescript
// Constants.ts
export const SHOP = {
  SEASONAL: {
    BOAT_HOUSE: { cost: { wood: 500 } },
    BOAT: { cost: { wood: 20 } },
    STONE_AXE: { cost: { wood: 50 } },
    IRON_AXE: { cost: { wood: 100 } },
    STEEL_AXE: { cost: { wood: 150 } },
    BACKPACK: { cost: { wood: 75 } },
  },
  FARM: {
    HAY: { cost: { meat: 10 } },
    STONE_SWORD: { cost: { meat: 30 } },
    IRON_SWORD: { cost: { meat: 60 } },
    STEEL_SWORD: { cost: { meat: 100 } },
    GUN: { cost: { meat: 999 } },  // Triggers stampede, never obtained
  },
  BEACH: {
    MACHETE: { cost: { cactus: 20 } },
    CACTUS_ARMOR: { cost: { cactus: 300, coconuts: 300 } },
    GROWTH_ELIXIR: { cost: { cactus: 30 } },
  },
  MOUNTAIN: {
    CHISEL: { cost: { stone: 20 } },
    PICKAXE: { cost: { stone: 50 } },
    CATAPULT: { cost: { stone: 3000, wood: 1000, meat: 1000 } },
    WATER: { cost: { stone: 10 } },
  },
  VOLCANO: {
    CRYSTAL_MAGNET: { cost: { crystal: 100 } },  // Triggers eruption
  },
  ETHEREAL: {
    ETHEREAL_AXE: { cost: { clouds: 30 } },
    TELEPORTER: { cost: { clouds: 200 } },
  },
}
```

2. **Add player data** (if it's a persistent upgrade):
```typescript
// Types.ts
interface PlayerTools {
  axeTier: 'wood' | 'stone' | 'iron' | 'steel'
  hasMachete: boolean
  // Add new tool
  hasNewTool: boolean
}
```

3. **Add UI button**:
```typescript
// ShopMenu.tsx
<ShopButton
  label="Cactus Armor"
  cost={SHOP.BEACH.CACTUS_ARMOR.cost}
  owned={player.hasCactusArmor}
  disabled={!canAfford(player, SHOP.BEACH.CACTUS_ARMOR.cost) || player.hasCactusArmor}
  onBuy={() => eventBus.emit('ui:buy', { item: 'cactusArmor', player: playerId })}
/>
```

4. **Handle purchase**:
```typescript
// Game.ts or ShopSystem.ts
eventBus.on('ui:buy', ({ item, player }) => {
  const p = this.getPlayer(player)
  
  if (item === 'cactusArmor') {
    const cost = SHOP.BEACH.CACTUS_ARMOR.cost
    if (canAfford(p, cost) && !p.hasCactusArmor) {
      subtractResources(p, cost)
      p.hasCactusArmor = true
    }
  }
  
  // Special case: story triggers
  if (item === 'gun') {
    eventBus.emit('story:stampede:trigger', { player })
    // Player never gets the gun, never loses resources
  }
})
```

5. **Add to save state**

---

## Debug Features

All debug features respect `DEBUG.ENABLED` flag.

### Adding Debug Hotkey

1. **Add key mapping**:
```typescript
// Constants.ts
DEBUG_KEYS: {
  TELEPORT_AREA: ['t'],
}
```

2. **Update ControlsBar**:
```typescript
// ControlsBar.tsx
'>: Load | <: Save | ... | T: Teleport'
```

3. **Handle in Debug.ts**:
```typescript
handleHotkeys(input: InputState) {
  if (input.justPressed(DEBUG_KEYS.TELEPORT_AREA)) {
    this.showTeleportMenu = !this.showTeleportMenu
  }
}
```

### Current Debug Tools

| Key | Feature |
|-----|---------|
| `<` or `,` | Save state |
| `.` or `>` | Load state |
| `/` or `?` | Delete state |
| `0` | Speed 0% (stop time, no pause overlay) |
| `1-5` | Speed presets (15%-500%) |
| `Scroll` | Speed ±10% |
| `Middle Mouse` | Speed 100% |
| `C` (hold) | Camera radial menu |
| `Z` | Toggle dev visualization |
| `X` | Spawn menu |
| `G` | Remove entities menu |
| `R` | Reset world |
| `T` | Teleport to area |

---

## UI Components

React components read from Zustand, emit events to game.

### Adding UI Element

1. **Add state to store** (if needed):
```typescript
// store.ts
interface Store {
  currentArea: AreaType
  inventory: ResourceInventory
}
```

2. **Create component**:
```typescript
// ui/AreaIndicator.tsx
export function AreaIndicator() {
  const area = useStore(s => s.currentArea)
  const areaName = AREAS[area].displayName
  
  return (
    <div style={{ position: 'absolute', top: 10, left: 10 }}>
      {areaName}
    </div>
  )
}
```

3. **Add to GameUI**:
```typescript
// GameUI.tsx
<AreaIndicator />
```

4. **Sync from game** (in Game.ts `syncToZustand()`):
```typescript
store.sync({ 
  currentArea: player.currentArea,
  inventory: player.inventory,
})
```

5. **Handle user actions** via events:
```typescript
// In component
onClick={() => eventBus.emit('ui:action', { type: 'openShop' })}

// In Game.ts
eventBus.on('ui:action', (data) => { /* handle */ })
```

### UI Rules

- UI reads from Zustand store
- UI never mutates game state directly
- UI emits events, Game listens and processes
- Game pushes to Zustand each frame via `syncToZustand()`

---

## Animal AI

Animals use a state machine defined in `AnimalSystem.ts`.

### Animal States

```
Farm Animals:  IDLE → WANDERING → FLEEING → BREEDING → IDLE
Bats:          IDLE → CHASING → (steal crystals / die) → IDLE
Beach Animals: RELAXING (permanent, decorative)
```

### Modifying Animal Behavior

1. **State logic**:
```typescript
// AnimalSystem.ts
updateAnimal(animal: Entity, delta: number) {
  switch (animal.animal.state) {
    case 'idle':
      if (this.isPlayerNearby(animal, ANIMAL.FLEE_RANGE)) {
        animal.animal.state = 'fleeing'
      } else if (animal.animal.fedHay && this.hasBreedingPartner(animal)) {
        animal.animal.state = 'breeding'
      } else if (Math.random() < 0.01) {
        animal.animal.wanderTarget = this.randomNearbyPoint(animal)
        animal.animal.state = 'wandering'
      }
      break
      
    case 'wandering':
      this.moveToward(animal, animal.animal.wanderTarget, delta)
      if (this.reached(animal, animal.animal.wanderTarget)) {
        animal.animal.state = 'idle'
      }
      break
      
    case 'fleeing':
      this.moveAwayFrom(animal, this.nearestPlayer(animal), delta)
      if (!this.isPlayerNearby(animal, ANIMAL.FLEE_RANGE * 1.5)) {
        animal.animal.state = 'idle'
      }
      break
      
    case 'breeding':
      animal.animal.stateTimer -= delta
      if (animal.animal.stateTimer <= 0) {
        this.spawnBabyAnimal(animal)
        animal.animal.fedHay = false
        animal.animal.state = 'idle'
      }
      break
  }
}
```

2. **Breeding mechanics**:
```typescript
hasBreedingPartner(animal: Entity): Entity | null {
  return this.animals.find(a => 
    a.animal.animalType === animal.animal.animalType &&
    a.animal.fedHay &&
    a.id !== animal.id &&
    distance(a, animal) < ANIMAL.BREED_RANGE
  )
}
```

3. **Bat behavior**:
```typescript
updateBat(bat: Entity, delta: number) {
  if (bat.animal.state === 'idle') {
    // Check if any player is looking at the bat
    for (const player of this.players) {
      if (this.isLookingAt(player, bat)) {
        bat.animal.targetPlayer = player.id
        bat.animal.state = 'chasing'
        break
      }
    }
  }
  
  if (bat.animal.state === 'chasing') {
    const target = this.getEntity(bat.animal.targetPlayer)
    this.moveToward(bat, target.position, delta, ANIMALS.BAT.speed)
    
    if (distance(bat, target) < 1) {
      // Steal crystals
      target.player.inventory.crystal = 0
      bat.animal.state = 'idle'
      bat.animal.targetPlayer = null
    }
  }
}
```

---

## Areas

Each area is a distinct zone with its own rules.

### Adding New Area

1. **Define area** in `Constants.ts`:
```typescript
AREAS: {
  NEW_AREA: { 
    position: { x: 300, z: 0 }, 
    radius: 50,
    currency: 'newResource',
    toolRequired: 'newTool',
    fogColor: 0xFF00FF,
  },
}
```

2. **Create area generator**:
```typescript
// world/NewArea.ts
export function generateNewArea(scene: Scene): Entity[] {
  const entities: Entity[] = []
  
  // Generate terrain
  const ground = createGround(AREAS.NEW_AREA)
  
  // Spawn resources
  for (let i = 0; i < 20; i++) {
    const pos = randomPointInArea(AREAS.NEW_AREA)
    entities.push(createResource('newResource', pos, 'newArea'))
  }
  
  // Spawn shopkeeper
  entities.push(createShopkeeper('newArea', AREAS.NEW_AREA.position))
  
  return entities
}
```

3. **Register in WorldGenerator.ts**:
```typescript
this.entities.push(...generateNewArea(scene))
```

4. **Add area transitions** in `AreaManager.ts`:
```typescript
CONNECTIONS: {
  mountain: ['volcano', 'newArea'],
  newArea: ['mountain'],
}
```

5. **Add shop inventory** in `Constants.ts`:
```typescript
SHOP: {
  NEW_AREA: {
    ITEM_1: { cost: { newResource: 50 } },
  },
}
```

---

## Story Events

Major story events that change the world permanently.

### Adding Story Event

1. **Define trigger** in `Constants.ts`:
```typescript
STORY: {
  NEW_EVENT_TRIGGER: { item: 'specialItem', area: 'someArea' },
}
```

2. **Add flag** to story state:
```typescript
interface StoryFlags {
  newEventTriggered: boolean
}
```

3. **Create trigger detection**:
```typescript
// StoryEventSystem.ts
checkTriggers() {
  // Listen for purchase that triggers event
  eventBus.on('ui:buy', ({ item, player }) => {
    if (item === STORY.NEW_EVENT_TRIGGER.item && !this.flags.newEventTriggered) {
      this.triggerNewEvent(player)
    }
  })
}
```

4. **Create event sequence**:
```typescript
triggerNewEvent(playerId: string) {
  this.flags.newEventTriggered = true
  
  // Start cutscene
  eventBus.emit('cutscene:start', {
    sequence: 'newEvent',
    onComplete: () => {
      // Apply world changes
      this.applyNewEventChanges()
    }
  })
}

applyNewEventChanges() {
  // Spawn new entities
  // Remove old entities
  // Unlock new areas
  // Change NPC states
}
```

5. **Add to save state**:
```typescript
// SaveState.ts
storyFlags: {
  newEventTriggered: this.game.storyFlags.newEventTriggered,
}
```

---

## Balancing

All balance values are in `Constants.ts`. Balancing = changing numbers only.

### Balance Pass

When adjusting game balance:

1. **Identify the constant** to change
2. **Change only in Constants.ts**
3. **Consider ripple effects** (e.g., changing yields affects economy)

### Key Balance Levers

| Constant | Affects |
|----------|---------|
| `RESOURCES.*.yields` | Gathering efficiency |
| `TOOLS.*.swings` | Harvesting speed |
| `TOOLS.*.cost` | Upgrade pacing |
| `ANIMALS.*.meatYield` | Farm economy |
| `ANIMALS.BREED_TIME` | Animal reproduction rate |
| `BOAT.SPEED` | River travel time |
| `BOAT.SINK_TIME` | Jump timing difficulty |
| `STRUCTURES.*.cost` | Progression pacing |
| `STORY.NEPHEW_SPAWN_DELAY` | Post-stampede wait |
| `PLAYER.MOVE_SPEED` | Travel time |
| `PARKOUR.*` | Cloud challenge difficulty |

---

## Events

Events are for discrete state changes with multiple listeners.

### When to Use Events

✅ Use events for:
- Resource harvested
- Item purchased
- Story event triggered
- Area entered
- Animal slaughtered/bred
- Boat collision
- Cutscene start/end

❌ Don't use events for:
- Position updates (per-frame)
- Animation ticks
- Collision checks

### Event Format

```typescript
eventBus.emit('category:action', {
  // Include all data listeners might need
  entityId: entity.id,
  playerId: player.id,
  area: entity.area,
  // etc.
})
```

### Listening

```typescript
// In Game.ts init or system constructor
eventBus.on('resource:harvested', (data) => {
  this.entities.add(createStump(data.position, data.area))
  this.addToInventory(data.playerId, data.resourceType, data.yield)
})

eventBus.on('story:stampede:triggered', (data) => {
  this.cutsceneSystem.play('stampede')
})
```

---

## Save State

If data matters across sessions, it must be in save state.

### What Gets Saved

- Player positions, rotations, current areas
- Player inventories (all resource types)
- Player tools and upgrades
- All resources (position, type, variant, progress)
- All animals (position, type, state, fed status)
- All saplings (position, type, growth progress)
- All boats (position, state)
- All structures (type, built status, owner)
- Story flags (all event triggers)
- Shopkeeper states (alive, replaced)
- Special states (spider scared, wall crack mined)

### Adding to Save State

```typescript
// SaveState.ts

// In serialize():
newData: this.game.newEntities.map(e => ({
  id: e.id,
  position: serializeVector(e.position),
  data: e.newEntity,
}))

// In deserialize():
for (const d of saveState.newData) {
  this.game.addEntity(createNewEntity(
    deserializeVector(d.position),
    d.data
  ))
}
```

---

## File Reference

| Adding... | Touch These Files |
|-----------|-------------------|
| Game value | `Constants.ts` |
| Entity type | `Types.ts`, `EntityFactory.ts`, `Models.ts`, new system, `SaveState.ts` |
| Model | `Palettes.ts`, `Geometry.ts`, `Models.ts` |
| Shop item | `Constants.ts`, `Types.ts`, `ShopMenu.tsx`, `ShopSystem.ts`, `SaveState.ts` |
| UI element | `store.ts`, new `.tsx`, `GameUI.tsx`, `Game.ts` (sync) |
| Debug tool | `Constants.ts`, `Debug.ts`, `ControlsBar.tsx` |
| Dialogue | `Dialogue.ts`, `ShopkeeperSystem.ts` |
| Animation | `Animation.ts`, triggering system |
| Area | `Constants.ts`, new area generator, `WorldGenerator.ts`, `AreaManager.ts` |
| Story event | `Constants.ts`, `StoryEventSystem.ts`, `CutsceneSystem.ts`, `SaveState.ts` |
| Balance | `Constants.ts` only |

---

## Checklist

Before completing any change:

- [ ] No magic numbers (all in Constants.ts)
- [ ] Types updated if data structure changed
- [ ] Save state includes new persistent data
- [ ] UI syncs from Game via Zustand
- [ ] Events only for state changes
- [ ] Collision checked before movement
- [ ] Uses `clock.delta` for timing
- [ ] Debug features respect `DEBUG.ENABLED`
- [ ] All assets are procedural
- [ ] Area context considered (which area owns this?)
- [ ] Story flags updated if progression affected
