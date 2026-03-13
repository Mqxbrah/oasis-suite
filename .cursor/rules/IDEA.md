# Sandbox Exploration Game

## Core Concept
A 1-4 player local co-op sandbox game about exploring interconnected biomes, gathering resources, and progressing through a whimsical world. Each area has its own currency, shopkeeper, and unique mechanics. Features resource management, area progression, story events, and procedural N64-style graphics.

## Gameplay

### Objective
Explore the world, gather resources, trade with shopkeepers, and unlock new areas. Progress from the starting seasonal forests through the Animal Farm, Beach/Desert, Mountain, Volcano, and finally the Ethereal Realm. There is no win condition - it's a sandbox experience.

### Seamless World
**NO LOAD ZONES** - The entire world is one connected space. All areas exist simultaneously and you can see neighboring areas from where you stand. Travel between areas is physical (walking, boating, catapulting) with no loading screens or transitions. The only exception is the Ethereal Realm which floats high above but is still part of the same scene.

### Player Spawning
- **1 PLAYER:** Spawn in 1 random seasonal area
- **2 PLAYERS:** Each spawns in a different random seasonal area (2 areas used)
- **3 PLAYERS:** Each spawns in a different random seasonal area (3 areas used)
- **4 PLAYERS:** Each spawns in a different seasonal area (all 4 areas used, one per player)

---

## Economy System

### No Universal Money
There is NO money in the game. Each area uses its **local resource as currency** for that area's shop. Some special cross-area items require combinations of materials from different areas.

### Currency by Area
| Area | Currency | Secondary |
|------|----------|-----------|
| Starting Areas (Seasons) | WOOD | - |
| Animal Farm | MEAT | - |
| Beach/Desert | CACTUS | COCONUTS |
| Mountain | STONE | - |
| Volcano | CRYSTAL | - |
| Ethereal Realm | CLOUDS | - |

### Cross-Area Items
These require material combinations from multiple areas:
- **Cactus Armor:** Cactus + Coconuts
- **Catapult:** Stone + Wood + Meat
- **Teleporter:** Clouds only

### Resources Don't Respawn
Resources are **finite** - once harvested, they're gone. However, you get **sapling/seed items** when harvesting that can be planted to grow more.

### Resource Yield Variants
All harvestable resources follow the same pattern - one resource type, multiple source variants with different yields:
- **TREES:** Different tree types give different wood amounts (best type = 3x wood)
- **LIVESTOCK:** Chickens = 1 meat, Pigs = 2 meat, Cows = 3 meat
- **CACTUS:** Different cactus types give different amounts (best type = 3x cactus)
- **CRYSTALS:** Different crystal colors give different amounts (best color = 3x crystals)

### Planting/Breeding Location Restrictions
- **SAPLINGS** (trees, cactus, cloud trees, etc): Can only be planted in FARM or STARTING AREAS
- **CRYSTALS** (baby crystals): Can only be planted in VOLCANO
- **LIVESTOCK:** Can technically be anywhere, but practically only in FARM or DESERT/BEACH since animals can't cross the river

---

## Areas & Progression

### 1. STARTING AREAS - The Four Seasons

Four starting areas that are gameplay-identical but visually represent the 4 seasons:
- **SPRING** - Top-left position
- **SUMMER** - Top-right position
- **WINTER** - Bottom-left position
- **FALL** - Bottom-right position

**Resources:** Mine trees with AXES. You get SAPLINGS to plant and WOOD to trade. Some trees are ROTTEN - these yield no wood but still fall with sound effects.

**Currency:** WOOD - all shop items cost wood

**Key Purchase:** Boat House (500 wood), then Boats (20 wood each)

---

### 2. ANIMAL FARM - Central Hub

Located at the CENTER of the four seasonal areas. Accessed via platform + tunnel from the rivers.

**Resources:** Breed and slaughter animals (CHICKENS, COWS, PIGS) to get MEAT. Upgrade your SWORD to kill them more efficiently (fewer swings). Buy HAY from the Farm shop (costs MEAT) to feed animals, causing them to breed with other animals of the same type that were also fed hay. Breeding produces ADULT animals immediately, not babies.

**Currency:** MEAT - all shop items cost meat

**Animal Respawn:** If you kill ALL of a certain animal type, two will spawn after 15 seconds.

**Key Item:** GUN (triggers the Stampede Event - see Story Events)

---

### 3. BEACH/DESERT

Located to the LEFT of the map, extending outward from the main hub. Connected to Animal Farm via tunnel that opens after the Stampede Event.

The area is split in half:
- **DESERT HALF:** Faces inward toward other areas (sandy terrain, cacti)
- **BEACH HALF:** Faces outward toward the ocean (shore, palm trees)

**Resources:** 
- Harvest 3 different types of CACTUS with MACHETE. You get CACTUS SEEDS to plant and CACTUS to trade.
- Shake PALM TREES (right trigger) to drop COCONUTS for trading. Palm trees are ONLY on the Beach half.

**Currency:** CACTUS (primary) + COCONUTS (secondary)

**Key Purchase:** Cactus Armor (cactus + coconuts) - protects you from being pricked by cactuses, and scares the spider blocking the tunnel to Mountain. Press E when near the spider to prick it with your armor, causing it to run off into the cave and disappear forever.

---

### 4. MOUNTAIN

Located to the RIGHT of the map. Connected via spider tunnel located in the ravine between SUMMER and FALL (opposite side from Animal Farm). Requires Cactus Armor to scare the spider and unlock access.

**Resources:** Mine STONE with CHISEL. A lava stream flows down from the Volcano - buy WATER (costs stone) to pour on lava, which creates new stone to mine.

**Currency:** STONE - all shop items cost stone

**Key Purchase:** Catapult (stone + wood + meat) - launches you into the Volcano

---

### 5. VOLCANO

Large CONE-SHAPED mountain. Players enter by launching via catapult from Mountain into the crater opening.

**Interior Layout:**
- Shopkeeper's hut FLOATING on the central lava pool
- 4 BRIDGES connecting crater edges to the floating shop platform
- When catapulted in, you land RIGHT BEFORE the start of the front bridge leading to the shop
- OVERHANG LAVA POOL in upper corner with lava spout dripping into main pool
- STALACTITES hanging from ceiling
- Light rays streaming down from crater opening

**Resources:** Mine CRYSTALS with PICKAXE. You get BABY CRYSTALS to plant and CRYSTALS to trade.

**Currency:** CRYSTAL - all shop items cost crystal

**Hazard - BATS:** If you look at bats, they chase you. If they reach you, you lose all crystals. Throw COCONUTS at bats to kill them.

**Exit:** A CRACK IN THE WALL (visible from mountainside) can be mined with pickaxe FROM INSIDE THE VOLCANO ONLY to create exit back to Mountain.

**Key Purchase:** Crystal Magnet (triggers the Eruption Event - see Story Events)

---

### 6. ETHEREAL REALM

Located HIGH ABOVE the entire map, floating in space. Accessed via cloud parkour after volcano eruption.

**Visual Style:**
- Main platform made of MANY CLUSTERED CLOUD SHAPES forming a large landmass
- GLASS FLOOR underneath clouds with subtle star reflection
- SKY: Deep space with blue/purple tint, visible Milky Way galaxy band, distant spiral galaxies, dense starfield

**Resources:** Mine CLOUD TREES with ETHEREAL AXE. You get ETHEREAL SAPLINGS and CLOUDS to trade.

**Currency:** CLOUDS - all shop items cost clouds

**Shopkeeper:** A CLOUD BEAR

**Falling:** If you jump off, you respawn back and your character says "Ah, I must have drifted off"

**Key Purchase:** Teleporter (clouds only) - places floating cloud platforms at every area's shopkeeper. Jump onto a cloud to open a top-down map popup where you select your destination for fast travel.

---

## Story Events

**Multiplayer Cutscenes:** When a story event triggers, the cutscene plays FULL SCREEN for all players (no splitscreen during cutscene). After the cutscene ends, splitscreen resumes.

### Stampede Event (Farm → Beach/Desert Access)

**Trigger:** Try to buy the GUN in the Farm shop.

**What Happens:**
1. Shopkeeper says "sure let me just get that for ya"
2. Animals STAMPEDE because they think buying a gun is cruel
3. Shopkeeper runs away before giving you the gun (you NEVER get the gun, you DON'T lose materials)
4. He JUMPS OVER THE GAP to the tunnel platform leading to beach
5. He tries to get through the wooden barrier blocking the tunnel
6. Animals ALSO JUMP THE GAP and plow him down at the barrier
7. Shopkeeper gets smushed flat against the ground as a texture (remains forever)
8. Animals break through the wooden barrier, opening access to Beach/Desert
9. Full screen cutscene plays, then splitscreen returns

**Aftermath:**
- The stampede animals DESPAWN after breaking through the barrier
- Animals were ALREADY in Beach/Desert before the stampede (this is a trick - the stampede just unlocks access)
- Some animals are on beach kickback chairs sippin pina coladas with sunglasses (DECORATIVE ONLY - cannot interact)
- Other animals roam freely and can be bred/slaughtered for meat
- 5 MINUTES after cutscene, a NEW SHOPKEEPER spawns at Farm
- New shopkeeper says: "I travelled here to work with my uncle, but I can't find him. Have you seen him?"
- New shopkeeper does NOT sell a gun

---

### Eruption Event (Volcano → Ethereal Realm Access)

**Trigger:** Buy the CRYSTAL MAGNET from the Volcano shop.

**What Happens:**
1. Volcano SHAKES and then ERUPTS
2. Shopkeeper says "OH NO, WE'VE DISTURBED THE VOLCANO'S PEACE"
3. Volcano erupts the shopkeeper's hut into the sky WITH YOU ON IT
4. You AND the shopkeeper land in a CLOUDY AREA with cloud platforms
5. Shopkeeper TRIES TO DO THE CLOUD PARKOUR but falls off (never seen again)
6. ALL BATS DISAPPEAR except ONE BAT by the crater edge - this bat becomes the NEW VOLCANO SHOPKEEPER

**Cloud Parkour Challenge:**
- Clouds that DISAPPEAR and REAPPEAR on a cycle
- Clouds that MOVE (side to side, up and down)
- Clouds that ROTATE - walk around them to not fall off
- If you FALL, respawn at start of cloud parkour
- Jump through final hole to enter Ethereal Realm
- THE HOLE CLOSES BELOW YOU - can never return to parkour area

---

## Boat & River Mechanics

### Water River System
Rivers flow as STRAIGHT CHANNELS through the ravines, directly connecting adjacent seasonal areas:
- 4 STRAIGHT RIVER CHANNELS - each runs in a straight line through a ravine between two adjacent areas
  - SPRING ↔ SUMMER (top horizontal ravine)
  - WINTER ↔ FALL (bottom horizontal ravine)
  - SPRING ↔ WINTER (left vertical ravine)
  - SUMMER ↔ FALL (right vertical ravine)
- Rivers do NOT curve around inside areas - they only exist in the ravine gaps between areas
- Rivers do NOT connect to the Animal Farm - Farm is in the CENTER, rivers flow AROUND it through the outer ravines
- River channels have CLIFFS on the sides creating natural walls
- **RIVER FLOW:** The visual water texture scrolls and PLAYERS swimming in water get pushed by current, but BOATS are NOT affected by river flow

### Boat Controls
- **PLACE BOAT:** With boat in left hand, place it in water (it spawns floating on the surface)
- **ENTER BOAT:** Right-click (feed button) on a boat to get into it
- **STEER BOAT:** W key moves the boat forward in the direction you're looking (camera direction)
- **EXIT BOAT:** Jump (A button / spacebar) to exit the boat
- **BOATS STAY IN PLACE:** Boats do NOT drift with river current - they stay stationary until you steer them
- **BOATS SINK:** After 5 seconds of the player exiting, the boat sinks and is destroyed

### Multiplayer Boat Interaction
If 2 PLAYERS SLAM THEIR BOATS INTO EACH OTHER, both boats STOP MOVING - this makes timing your jump easier.

### Animal Farm Access via River
At the HALFWAY POINT of each river channel, there is a PLATFORM on the INSIDE (toward center) that leads via TUNNEL to the Animal Farm.

### Key Purchases
- **Boat House:** 500 wood - built next to the shopkeeper hut in your seasonal area, unlocks ability to build boats
- **Boat:** 20 wood each

---

## Tools & Equipment

### Axes (Starting Areas)
| Axe Type | Swings to Cut | Upgrade Cost |
|----------|---------------|--------------|
| Wood | 10 swings | Starting axe |
| Stone | 6 swings | Wood |
| Iron | 4 swings | Wood |
| Steel | 3 swings | Wood |

### Other Tools
- **SWORD** (Farm) - For slaughtering animals, upgrades reduce swings needed
- **MACHETE** (Beach/Desert) - For harvesting cactus
- **CHISEL** (Mountain) - For mining stone
- **PICKAXE** (Volcano) - For mining crystals
- **ETHEREAL AXE** (Ethereal Realm) - For harvesting cloud trees

### Chopping Mechanics
- **Hold to chop:** Hold attack button to swing repeatedly
- **Swing direction:** Player always swings right to left
- **Swing commitment:** Once a swing starts, it completes fully (300ms)
- **No progress memory:** If you stop mid-cut, the resource resets - you must start over

---

## Shop System

### Shop Interface
Each area has a SHOPKEEPER with local items using local currency.

### Shop Sections (FUTURE FEATURE - NOT FOR FIRST PASS)
Shopkeepers will eventually have TWO sections:
1. **Items Section** - Tools, consumables, and upgrades (current implementation)
2. **Building Section** - Building blocks and placeable objects for base-building

**IMPORTANT:** Do NOT implement the Building Section during the initial development pass. This feature will be added much later, after the core game is complete. When initially creating shopkeepers, only implement the Items Section.

---

## Map Layout

### Overview
The world is **ONE SEAMLESS CONNECTED SPACE** with no loading zones. All areas exist simultaneously in a single scene - you can look across the map and see other areas in the distance.

Structure:
- 4 CIRCULAR SEASONAL PLATFORMS in a 2x2 grid pattern
- RIVER CHANNELS in ravines between seasonal areas (physically traversable by boat)
- CENTRAL ANIMAL FARM in the middle (accessed only via tunnels, NOT by river)
- BEACH/DESERT extends outward from the left side
- MOUNTAIN on the right side, connected to VOLCANO (large cone shape)
- ETHEREAL REALM floating high above in space (visible from below, same scene)

### Wall Geometry
- All walls (radial platform edges, ravine walls) use the SAME visual style
- Walls must connect PERFECTLY or overlap slightly - NO GAPS between wall segments
- Ravine walls should seamlessly blend into the circular platform walls

### Connections Summary

| From | To | How |
|------|-----|-----|
| Starting Areas | Starting Areas | Boat through rivers |
| Starting Areas | Animal Farm | Jump from boat to halfway platform, walk through tunnel |
| Animal Farm | Beach/Desert | Through tunnel (after Stampede Event) |
| Summer/Fall Ravine | Mountain | Spider tunnel (use Cactus Armor on spider - press E) |
| Mountain | Volcano | Catapult launch |
| Volcano | Mountain | Mine crack in wall |
| Volcano | Ethereal Realm | Eruption Event + Cloud Parkour |
| Ethereal Realm | Anywhere | Teleporter |

### Shortcut
The Farm tunnel platform and Spider tunnel platform are on OPPOSITE SIDES of the river (in the ravine between Summer and Fall). You can JUMP directly across for quick access between Farm and Spider tunnel.

### Progression Path (Intended Order)
1. **SPRING/SUMMER/WINTER/FALL** - Gather wood
2. **Build boat house + boats** - Visit other seasonal areas via river
3. **ANIMAL FARM** - Trade meat, trigger Stampede
4. **BEACH/DESERT** - Gather cactus and coconuts, get Cactus Armor
5. **MOUNTAIN** - Gather stone, build Catapult
6. **VOLCANO** - Gather crystals, trigger Eruption
7. **ETHEREAL REALM** - Gather clouds, build Teleporter for endgame fast travel

---

## Controls
- **1-4 controllers** plugged into MacBook
- Each player uses their own controller
- **First person camera option** available

## Visual Style
- **Vertical splitscreen** for multiplayer (2-4 players)
- **3rd person camera** - Roblox-style perspective behind player
- **First person camera** - Optional toggle
- Trees/resources distinguished by color coding

### N64 Authenticity
Chunky, retro aesthetic inspired by N64-era graphics:
- **320×240 internal resolution**
- **60° vertical FOV**
- **16 colors per texture** (4-bit indexed palettes)
- **100–500 triangles per character**
- **Segmented limbs** (no skeletal skinning)
- **Vertex lighting** (Gouraud shading)
- **Distance fog** (linear, per-area)
- **30 FPS target**

### Procedural Asset Creation
All assets are built programmatically — no external imports:
- Models built **vertex by vertex**
- Textures built **pixel by pixel**
- Pure math, no shortcuts

### Collision
- **Basic collision** on all entities - players, trees, animals, resources
- Players cannot phase through each other or any objects

### HUD Elements
- **Personal UI:** Current resources, capacity, equipped tool, area name

---

## Technical Constraints

### Architecture Rules
- **No magic numbers** — all values defined in `Constants.ts`
- **GameClock delta times** for all timing logic
- **No `setInterval`/`setTimeout`** for game logic
- **EventBus** only for state changes with multiple listeners (not per-frame operations)
- **Direct calls** for per-frame updates (movement, camera, AI)
- Systems must scale (100+ entities should still work)

### Forbidden
- ❌ Placeholder cubes/spheres as final assets
- ❌ Quick hacks or "temporary" fixes
- ❌ Hardcoded values in code
- ❌ Smooth, modern aesthetics (we want chunky N64)
- ❌ Adding core features mid-development

---

## Visual Reference
See diagram: MAP TOP DOWN DIAGRAM.png

The diagram shows:
- Central "stampede ANIMAL FARM" hub
- 4 seasonal areas (SPRING, SUMMER, FALL, WINTER) surrounding the farm
- 4 STRAIGHT BLUE RIVER CHANNELS - each runs directly between two adjacent seasonal areas through the ravine gap (rivers do NOT flow through or around Farm)
- BEACH/DESERT on the left side
- MOUNTAIN and VOLCANO on the right side (connected by catapult, with tunnel/crack for return)
- ETHEREAL REALM floating above (accessed via volcano eruption)
- Spider tunnel in the ravine between Summer and Fall, connecting to Mountain area

---

*Inspiration: Combines the satisfying progression of resource gathering games with exploration, whimsical story events, and cooperative multiplayer.*

THERE SHOULD BE 2 WILD DOGs THAT YOU CAN TAME RANDOMLY SOMEWHERE IN THE ENTIRE GAME MAP WITHIN THE BOUNDS ONE OF THE AREAS BUT NOT 2 IN EVERY AREA, AND ONCE 1 OF THE WILD DOGS IS TAMED ANOTHER WILD ONE SPAWNS SO 2 WILD DOGS AT ANY GIVEN TIME, WHEN YOU TAME A DOG, IT FOLLOWS BEHIND YOU AND BARKS, IT LOWERS THE PRICES OF ALL GOODS IN SHOPS BY 1.2x FOR EVERY ADDITIONAL DOG YOU HAVE, AND IF YOU HAVE AT LEAST ONE DOG, THE SHOP KEEPER SAYS IN A TEXT BOX BESIDE THE SHOP: "Good Boy, Good Boy!"

---

## BEAR ENEMY (All Four Starter Areas)

A BEAR spawns in each of the four seasonal starting areas. In WINTER, it's a POLAR BEAR (white). In SPRING, SUMMER, and FALL, it's a regular brown BEAR.

**Behavior:**
- Every 60-90 seconds (random interval), the bear will charge toward the player
- If the bear reaches you and you DON'T right-click a FISH on it, you instantly LOSE ALL ITEMS IN YOUR INVENTORY
- If you right-click a FISH on the bear before it reaches you, it stops, eats the fish, and goes back to wandering

**Shop Items (Seasonal Shopkeeper - costs WOOD):**
- **FISH:** 10 wood each - Used to feed bears and prevent them from attacking
- **BEAR REPELLENT:** 500 wood - Permanent upgrade. Once purchased, bears will NEVER charge at you again (they just wander peacefully)

---

### DONKEY (Desert/Beach Area)

A DONKEY is for sale at the Desert/Beach shopkeeper. It provides a massive carrying capacity upgrade.

**Shop Item (Beach/Desert Shopkeeper - costs CACTUS):**
- **DONKEY:** [TBD cactus] - Increases your carrying capacity by 3x the amount of the BACKPACK. The donkey follows behind you and carries your extra resources.

---

## MOUNTAINEERING BACKPACK (Mountain Area)

A specialized backpack for sale at the Mountain shopkeeper.

**Shop Item (Mountain Shopkeeper - costs STONE):**
- **MOUNTAINEERING BACKPACK:** [TBD stone] - A larger capacity upgrade available in the Mountain area. Increases carrying capacity (amount TBD, should fit between regular backpack and donkey).