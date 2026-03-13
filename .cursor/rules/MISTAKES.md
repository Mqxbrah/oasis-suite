# MISTAKES.md - Common Mistakes to Avoid

## Wall/Geometry Alignment Between Different Mesh Types

When connecting two different mesh types (e.g., circular area walls to straight ravine walls):

1. **Use identical formulas for edge positions** - Both meshes must calculate their connection points using the EXACT same math. If circular walls use `effectiveHalfWidth = ravineHalfWidth - overlapMargin`, ravine walls must use the same formula with the same values.

2. **Tessellation can cause gaps** - Uniform tessellation samples may not land at exact connection points. Solution: Insert extra vertices at the EXACT connection angles by adding them to the angle sample list before building the mesh.

3. **Add tiny overlap for hairline cracks** - Even with exact matching, floating-point precision can cause hairline rendering cracks. Add a tiny overlap (0.05 units) to the wall that should extend slightly past the other.

4. **World-position-based displacement** - When applying noise/displacement to walls that should connect, use world-position-based displacement (based on x, y, z coordinates) so vertices at the same world position get displaced identically regardless of which mesh they belong to.

5. **Consistent displacement direction** - Use the same displacement direction formula for both meshes. Using mesh normals won't work if meshes have different `sideOrientation` (FRONTSIDE vs BACKSIDE have opposite normals). Instead, use a consistent world-space direction like radial from world origin.

**Key values in MapGenerator.ts:**
- Circular wall `overlapMargin`: 2.55 (slightly more than ravine's 2.5)
- Ravine wall `overlapMargin`: 2.5
- Both use `effectiveHalfWidth = Math.max(1, ravineHalfWidth - overlapMargin)`

## Gamepad Button Mapping

The HTML5 Gamepad API button indices for Xbox-style controllers in this project:

| Index | Button |
|-------|--------|
| 0 | Y (top) |
| 1 | B (right) |
| 2 | A (bottom) |
| 3 | X (left) |
| 4 | Left Bumper (LB) |
| 5 | Right Bumper (RB) |
| 6 | Left Trigger (LT) |
| 7 | Right Trigger (RT) |
| 8 | Back/View |
| 9 | Start/Menu |
| 10 | Left Stick Click |
| 11 | Right Stick Click |
| 12 | D-Pad Up |
| 13 | D-Pad Down |
| 14 | D-Pad Left |
| 15 | D-Pad Right |

**Current button assignments:**
- A (index 2): Unassigned (does nothing)
- B (index 1): Jump + Interact
- X (index 3): Unassigned (does nothing)
- Y (index 0): Inventory + Cancel/Back (closes menus) + Exit Boat
- RT (index 7): Chop (outside menus), Drop All (inside inventory menu only)
- LT (index 6): Feed/Place
- LB (index 4): Unassigned (previously camera radial - now keyboard C only)
- RB (index 5): Drop item (tap for one, hold 0.5s for rapid drop)
- Left Stick Click (index 10): Sprint trigger
- Right Stick Click (index 11): Toggle first-person/third-person camera
- Start (index 9): Pause

## Babylon.js Rotation Direction

In Babylon.js, **positive rotation.x tilts BACKWARD** (top moves toward -Z in local space), NOT forward. If you want a mesh to tilt forward (in the direction it's facing after rotation.y), use **negative rotation.x**.

Example: For tree falling animation, use `rotation.x = -Math.PI/2` to tilt forward (away from player), not `rotation.x = Math.PI/2` which tilts backward (toward player).

## Ground/Wall Displacement Direction Matching

When both ground and wall meshes use displacement based on world position, they MUST use the same radial direction calculation:
- **WRONG**: Ground uses radial from area center, wall uses radial from world origin → MISMATCH
- **CORRECT**: Both use radial from world origin `(worldX / distFromWorldOrigin, worldZ / distFromWorldOrigin)`

## Wall Seam at Angle 0/2π

When creating circular wall ribbons:
- **DON'T** include both angle 0 and angle 2π in the vertex list (they're the same position, causes seam issues)
- **DO** use `i < tessellation` not `i <= tessellation` when sampling angles
- **DO** add a closing profile at angle 0 to segments that wrap around (end near 2π)

## Camera Mode Initialization and Texture Corruption

Switching camera mode (e.g., to first-person) during scene initialization can cause texture corruption. The fix:
1. Initialize cameras with default mode (third-person)
2. Wait for `scene.executeWhenReady()` to ensure textures are loaded
3. THEN switch camera mode after first frame renders

## Gamepad Look Inversion

Gamepad vertical look requires different handling for different camera types:
- **Player camera**: Negate `gpLookY` in Input.ts so stick-up = positive lookY = look up
- **Free camera**: Re-negate `input.lookY` in CameraSystem.ts because free cam expects raw delta (opposite convention)

## Cursor Browser Pointer Lock Issue

The Cursor Browser (embedded browser in Cursor IDE) blocks canvas click events after pointer lock is released. This means:
- Clicking on the canvas to re-capture the mouse does NOT work
- Click events on the canvas are simply not received by JavaScript handlers
- Document-level and window-level capture handlers also don't receive these clicks

**The Workaround:**
Use a transparent overlay `<div>` that appears when pointer lock is NOT active. This div:
1. Covers the entire screen with `zIndex: 10` (above canvas and UI)
2. Has `onClick` handler that requests `canvas.requestPointerLock()`
3. Disappears when pointer IS locked (allowing normal gameplay)
4. Shows a "Click anywhere to capture mouse" hint

**Implementation in App.tsx:**
```tsx
const [isPointerLocked, setIsPointerLocked] = useState(false)

// Track pointer lock state
useEffect(() => {
  const handleLockChange = () => {
    setIsPointerLocked(document.pointerLockElement === canvasRef.current)
  }
  document.addEventListener('pointerlockchange', handleLockChange)
  return () => document.removeEventListener('pointerlockchange', handleLockChange)
}, [])

// Render overlay when not locked
{!isPointerLocked && (
  <div onClick={() => canvasRef.current?.requestPointerLock()}
       style={{ position: 'absolute', top: 0, left: 0, width: '100%', height: '100%', zIndex: 10 }}>
    <div style={{ /* hint styles */ }}>Click anywhere to capture mouse</div>
  </div>
)}
```

**Note:** This issue is specific to Cursor Browser. Regular browsers (Chrome, Firefox, Safari) don't have this problem.

## Camera Radial Menu - Release to Confirm

The camera radial menu (C key) uses a **hold-move-release** pattern:

1. **Hold C** to open the radial menu
2. **Move mouse/joystick** to select a direction (visual feedback shows selected sector)
3. **Release C** to confirm the selection

**DO NOT** change this to "select immediately on direction detection" - that breaks the UX because:
- Users need time to see the menu and aim at their desired option
- Immediate selection on movement means any accidental mouse movement dismisses the menu
- The release-to-confirm pattern is intentional and expected

**Quick tap behavior**: If C is tapped quickly without moving, it cycles through camera modes (normal → free → static → normal).

## Tunnel/Area Transition Zone Collision Gap

When tunnel entrances connect to areas (like farm), there's a **transition zone** where the player is:
1. Outside the area's collision circle (e.g., beyond farm radius of 95)
2. But NOT yet inside the tunnel's corridor detection

This happens because:
- Tunnel walls use `getWallDisplacement()` to push the visual entrance OUTWARD (by ~2-5 units)
- The tunnel's `farmEnd` coordinate is displaced to match the visual geometry
- This creates a gap between the area's collision boundary and where tunnel detection begins

**The Mistake:**
In `getTunnelFloorY`, using `isInTunnelCorridor(position, tunnel, false)` means `farmOverlap = 0`. Positions in the transition zone (e.g., x=96.4 when farm radius=95 and displaced farmEnd≈97) return `tunnelFloorY=null`, causing the player to fall to y=0.

**The Fix:**
Use `isInTunnelCorridor(position, tunnel, true)` so that `farmOverlap = 12`. This extends the corridor detection 12 units INSIDE the farm area, covering any possible transition gap.

**Key insight:** Visual geometry displacement affects collision boundaries. When walls are displaced outward, collision detection must extend INWARD to cover the gap.

## Verify Changes Were Actually Applied

When fixing bugs that involve changing values in multiple locations:

1. **Always grep/search after making changes** - Don't assume edits were applied. Search for the pattern again to confirm:
   - The old values are gone
   - The new values are in place
   - All locations were updated

2. **Multiple locations with similar values** - If a value appears in 8+ places, it's easy to miss some or have edits fail silently. After editing, run a search like `grep "overlapMargin.*="` to see ALL occurrences.

3. **Comments can lie** - If a comment says "Must match X!" but the value doesn't match, the comment is wrong, not the documentation. Always trust the documented values in MISTAKES.md over inline comments that may be stale.

**Example:** The hairline crack fix requires:
- Circular walls: `overlapMargin = 2.55` (in `createAreaCliffs`, `getAreaWallSegments`)
- Ravine walls: `overlapMargin = 2.5` (in `createRavineWalls` and ALL ravine-related functions)

If you "fix" this but don't verify, you might end up with all values still at 2.55, which leaves the crack unfixed.

## Terrain Height Detection - "At or Below" vs "Near"

When checking if a player should use a specific floor/platform height (like volcano interior floor or entrance platform):

**WRONG:** `player.y <= floorY + tolerance` ("at or below")
- This is TRUE for any position BELOW the floor, including external terrain at a much lower Y level
- Causes player to SNAP UP from lower terrain to the floor (e.g., from Y=119 on mountain to Y=130 volcano floor)

**CORRECT:** `player.y >= floorY - tolerance` ("near floor level")
- This is only TRUE when player is at or above the floor minus a small tolerance
- Allows landing from above, standing on floor
- Prevents snapping UP from much lower terrain

**Example:**
- Volcano floor at Y=130, player on mountain terrain at Y=119
- WRONG: `119 <= 132` is TRUE → snaps player up to Y=130
- CORRECT: `119 >= 125` is FALSE → player stays on mountain terrain at Y=119

**Key insight:** "At or below" sounds correct but includes positions far below on other terrain. Use a MINIMUM threshold to prevent upward snapping.

**Affected functions in PlayerSystem.ts:**
- `getGroundLevel()` - volcano floor detection
- `updateJump()` - volcano floor and entrance platform detection
- `updateMovement()` - volcano floor and entrance platform detection

## Always Use Three-Point Bilinear Materials for Textures

When creating textured materials in this game, **ALWAYS** use `createThreePointBilinearMaterial()` from `N64Pipeline.ts`. This is the standard rendering pipeline for all textures in the game.

**WRONG:**
```typescript
const mat = new StandardMaterial('myMat', this.scene)
mat.emissiveTexture = myTexture
mat.diffuseColor = Color3.Black()
```

**CORRECT:**
```typescript
const mat = createThreePointBilinearMaterial(
  'myMat',
  myTexture,
  texWidth,
  texHeight,
  this.scene
)
```

**Why:** The three-point bilinear material provides:
- N64-authentic rendering with the custom shader pipeline
- Consistent look with all other textures in the game
- Proper brightness/emissive handling for the unlit style

**Never** use `StandardMaterial` with `emissiveTexture` directly for textured objects. Always go through the N64 pipeline.

## Z-Fighting with DOUBLESIDE + backFaceCulling = false

When creating ribbon meshes (walls, surfaces), **DO NOT** combine `Mesh.DOUBLESIDE` with `backFaceCulling = false`. This causes z-fighting (flickering/artifacts when viewed up close).

**Why it happens:**
- `sideOrientation: Mesh.DOUBLESIDE` creates geometry for BOTH front and back faces (two layers of triangles)
- `backFaceCulling = false` renders both sides of EVERY triangle
- Result: Front-facing and back-facing triangles overlap at nearly identical depths → GPU flickers between them

**WRONG:**
```typescript
const wall = MeshBuilder.CreateRibbon('wall', {
  pathArray,
  sideOrientation: Mesh.DOUBLESIDE,  // Creates front + back geometry
}, scene)
const mat = new StandardMaterial('wallMat', scene)
mat.backFaceCulling = false  // BAD: Now both layers render from both sides
wall.material = mat
```

**CORRECT:**
```typescript
const wall = MeshBuilder.CreateRibbon('wall', {
  pathArray,
  sideOrientation: Mesh.DOUBLESIDE,  // Creates front + back geometry
}, scene)
const mat = new StandardMaterial('wallMat', scene)
mat.backFaceCulling = true  // GOOD: Each layer renders its own side only
wall.material = mat
```

**Alternative (single-layer, both sides visible):**
```typescript
const wall = MeshBuilder.CreateRibbon('wall', {
  pathArray,
  sideOrientation: Mesh.FRONTSIDE,  // Single layer of geometry
}, scene)
const mat = new StandardMaterial('wallMat', scene)
mat.backFaceCulling = false  // GOOD: Single layer visible from both sides
wall.material = mat
```

**Symptoms:** Brown/gray square artifacts, flickering surfaces, visual noise when camera gets within ~1-2 meters of the wall.

**Fixed in:** Mountain semicircle wall, mountain side walls (MapGenerator.ts)

## Adding New Inventory Item Types - Complete Checklist

When adding a new item type (like `fish`, `crystal`, etc.), you MUST update ALL of the following locations. Missing even one causes bugs like items not showing in UI, not counting toward weight, not saving/loading, etc.

### 1. Type Definition
- **Types.ts** - Add to `InventoryItemType` union type:
  ```typescript
  export type InventoryItemType = 'wood' | 'meat' | ... | 'newItem'
  ```

### 2. Player Data
- **Types.ts** - Add field to `PlayerData` interface:
  ```typescript
  newItem: number
  ```
- **EntityFactory.ts** - Initialize in `createPlayer()`:
  ```typescript
  newItem: 0,
  ```

### 3. Constants
- **Constants.ts** - Add to `INVENTORY.ITEM_NAMES`:
  ```typescript
  newItem: 'New Item',
  ```
- **Constants.ts** - Add to `INVENTORY.ITEM_COLORS`:
  ```typescript
  newItem: '#hexcolor',
  ```

### 4. Store (UI State)
- **store.ts** - Add to player state interface and default values:
  ```typescript
  newItem: number
  // ...
  newItem: 0,
  ```

### 5. Weight/Capacity Calculations (CRITICAL - easy to miss!)
All these locations sum up inventory items for capacity checks:

- **HUD.tsx** - `usedSpace` calculation:
  ```typescript
  const usedSpace = player.wood + ... + player.newItem
  ```
- **HUD.tsx** - `getItemCount()` switch statement - add case:
  ```typescript
  case 'newItem': return player.newItem
  ```
- **HUD.tsx** - `allItemTypes` array:
  ```typescript
  const allItemTypes: InventoryItemType[] = [..., 'newItem']
  ```
- **PlayerSystem.ts** - `totalSpaceUsed` calculation (capacity check)
- **PlayerSystem.ts** - `totalMaterials` calculation (animation)
- **Game.ts** - `totalMaterials` calculation (multiple locations for animations)

### 6. Sync to Store
- **Game.ts** - `syncPlayerToStore()` - add field to playerData object:
  ```typescript
  newItem: player.player.newItem,
  ```

### 7. Save/Load State
- **Game.ts** - `loadSaveState()` - restore the value:
  ```typescript
  player.player.newItem = savedPlayer.data.newItem ?? 0
  ```

### 8. Storage Barrel
- **Game.ts** - `createEmptyStorageBarrelInventory()`:
  ```typescript
  newItem: 0,
  ```
- **Game.ts** - `ejectAllFromStorageBarrel()` - add to keys array AND switch case:
  ```typescript
  const keys: InventoryItemType[] = [..., 'newItem']
  // ...
  case 'newItem':
    playerData.newItem += amount
    break
  ```

### 9. Inventory Menu
- **InventoryMenu.tsx** - Add to `inventoryItems` array:
  ```typescript
  {
    id: 'newItem',
    name: INVENTORY.ITEM_NAMES.newItem,
    count: player.newItem,
    color: INVENTORY.ITEM_COLORS.newItem,
    isPinned: pinnedItems.includes('newItem'),
    canEquip: true,
  },
  ```

### 10. Drop/Pickup System
- **Game.ts** - `dropItem()` switch case to decrement count
- **DroppedItemSystem.ts** - pickup switch case to increment count
- **Game.ts** - drop-all `getCount()` switch case

### 11. Debug Commands
- **Debug.ts** - L key +50 resource switch case:
  ```typescript
  case 'newItem':
    player.player.newItem += 50
    break
  ```

### 12. Visual Model (if equippable)
- **Models.ts** - Create held item model
- **Models.ts** - Create dropped item template
- **EntityManager.ts** - `updateLeftHandItem()` switch case to show/hide model

### Summary of Files to Touch
| File | What to Add |
|------|-------------|
| Types.ts | Type union + PlayerData field |
| EntityFactory.ts | Default value |
| Constants.ts | Name + color |
| store.ts | State field + default |
| HUD.tsx | usedSpace, getItemCount, allItemTypes |
| PlayerSystem.ts | totalSpaceUsed, totalMaterials |
| Game.ts | syncPlayerToStore, loadSaveState, dropItem, storage barrel, weight calcs |
| InventoryMenu.tsx | inventoryItems array |
| DroppedItemSystem.ts | Pickup handler |
| Debug.ts | L key command |
| Models.ts | Visual models |
| EntityManager.ts | Left hand display |

**The most commonly missed locations are:**
1. Weight/capacity calculations (HUD + PlayerSystem + Game)
2. HUD `getItemCount()` switch statement
3. Storage barrel withdrawal
4. Save state restoration

## Adding New Entity Types

When adding a new entity type, you MUST update:

1. **Types.ts** - Add to `EntityType` union
2. **EntityManager.ts** - Add to the `types` array in constructor:
   ```typescript
   const types: EntityType[] = [
     'player', 'tree', ... 'yourNewType'  // Add here!
   ]
   ```

**If you forget step 2**, the entity will be created but `entities.ofType('yourNewType')` will return an empty array because `byType.get('yourNewType')` returns `undefined` - the entity ID is never registered.

**Symptoms of this mistake:**
- Entity spawns and appears visually
- But hit detection, AI systems, or any code using `ofType()` can't find it
- No errors thrown - just silent failure

## Storage Barrel IDs Must Be Deterministic

Storage barrels are created during world setup via `createStorageBarrel()`. Their inventories are saved keyed by the barrel's entity ID. If the barrel IDs are random (using `generateId()`), then when the game loads:

1. New barrels get NEW random IDs (e.g., `storagebarrel_abc123`)
2. Saved inventories reference OLD IDs (e.g., `storagebarrel_xyz789`)
3. Inventories never match - items appear lost

**The Fix:**
Storage barrel IDs must be deterministic based on their area:
- `storagebarrel_SPRING`
- `storagebarrel_SUMMER`
- `storagebarrel_FARM`
- etc.

This is done by passing `areaId` to `createStorageBarrel()`:
```typescript
createStorageBarrel(position, rotation, areaId)  // Uses: `storagebarrel_${areaId}`
```

**Key insight:** Any entity whose persistent data is keyed by entity ID (like storage barrel inventories) must use deterministic IDs that survive save/load cycles.
