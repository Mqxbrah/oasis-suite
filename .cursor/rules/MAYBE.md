# MAYBE.md - Game Ideas & Suggestions

Read this file only when prompted to suggest ideas for gameplay mechanics, story, dialogue, puzzles, etc.

---

## THE VIBE

Two lumberjacks. One forest. Finite trees.

No story. No lore. Just two people who showed up to work and now it's personal.

---

## CHARACTERS

### Blue

The grinder. Shows up, does the work, doesn't celebrate.

- Chops with short, efficient swings
- When idle: shifts weight, adjusts grip—never fully still
- When selling: tosses wood onto belt without looking
- When stunned by nail: sits up slowly, stares at the tree
- Victory pose: plants axe in stump, walks away
- Defeat: stands motionless, axe hanging at side

*Blue doesn't get mad. Blue gets even.*

### Red

The hustler. Always moving, always scheming.

- Chops with wide, aggressive swings
- When idle: paces, looks around at nearby trees
- When depositing: watches the wood move on the conveyor
- When stunned by nail: pounds the ground, gets up fast
- Victory pose: raises axe overhead, holds it
- Defeat: throws axe on ground

*Red plays to win. Second place is losing.*

### The Vendor

Old. Tired. Seen every trick in the book.

- Never stands up
- Missing two fingers on left hand
- Doesn't look at you when you talk
- Counts wood by weight, not sight
- Has a coffee mug that's never empty

*He was doing this before you were born. He'll be doing it after you're gone.*

---

## VENDOR DIALOGUE

Short. Flat. No enthusiasm. Funny because he doesn't care.

### Greetings

```
"Mm."
```
```
"What."
```
```
"Back."
```
```
"You again."
```
```
"Still going."
```

### Depositing Wood

**Tiny load (under 10 wood):**
```
"Hardly worth the trip."
```
```
"That all?"
```

**Small load (10-30 wood):**
```
"Fine."
```

**Medium load (30-70 wood):**
```
"Acceptable."
```

**Big load (70-150 wood):**
```
"Hm. Not bad."
```

**Full capacity (150+):**
```
"Where'd you find all that."
```
```
"Been busy."
```

**Selling cedar specifically:**
```
"Red stuff. Good."
```

**Selling only hemlock:**
```
"Lot of hemlock."
```
*(Slight judgment. Hemlock is the cheap wood.)*

### Buying Axes

**Stone:**
```
"Upgrade."
```

**Iron:**
```
"Don't chip it."
```

**Steel:**
```
"Best I got. Don't lose it."
```

**Already own it:**
```
"You have that."
```

**Can't afford it:**
```
"No."
```

### Hiring Workers

**First:**
```
"He works. That's it."
```

**Second:**
```
"Two now."
```

**Third:**
```
"They'll sort it out."
```

**Fourth:**
```
"Getting crowded."
```

**Fifth+:**
```
"Your choice."
```

**Can't afford:**
```
"Come back with more."
```

### Buying Nails

**First time:**
```
"Dirty. Works though."
```

**Second time:**
```
"More nails."
```

**Third+:**
```
"Got a grudge?"
```

**Buying 5+ at once:**
```
"Lot of nails."
```
*(Long pause before this one.)*

### Capacity Upgrades

**Backpack:**
```
"Holds more."
```

**Donkey:**
```
"Slow. Steady."
```

**Horse:**
```
"Fast. Eats a lot. Not your problem."
```

### Situational Lines

**Leaving without buying:**
```
"See you."
```
```
"Just looking."
```

**Coming back within 30 seconds:**
```
"Forget something?"
```
```
"What now."
```

**First visit of the game:**
```
"New faces."
```

**Haven't visited in 5+ minutes:**
```
"Thought you quit."
```

### Game State Reactions

**Trees getting sparse (25% remaining):**
```
"Fewer trees now."
```
```
"Thinning out."
```

**Very few trees left (10% remaining):**
```
"Almost done out there."
```
```
"Trees are going."
```

**Player has been very busy (lots of deposits):**
```
"Doing well."
```
*(Said without enthusiasm. Almost suspicious.)*

### After Major Events

**Player just hit a nail (axe broke):**
```
"Need a new axe?"
```
*(He knows. He always knows.)*

**Player's worker just hit a nail:**
```
"Heard that from here."
```

**Opponent is currently at the trading post too:**
```
"Busy day."
```

---

## GAMEPLAY MOMENTS

The small things that make it feel real.

### The Opening Seconds

Both players spawn. The forest is full. You look at each other across the map. Then you run.

That first 10 seconds—before anyone's cut anything—is pure anticipation.

### The First Nail

You're chopping. You hear the PING. Your finger freezes on the button. Do you let go? Can you afford to lose this tree?

If you let go: the tree is dead forever. Wasted.
If you hold: you're gambling your axe.

The nail doesn't hurt you. The *decision* does.

### Spotting a Rotten Tree Too Late

Three swings in. You see the mushroom. You already committed.

The animation plays out. No wood drops. You wasted time you didn't have.

### Racing for the Same Tree

You both see it. Large cedar. You're running. They're running. You get there first—barely. You start chopping. They stand there for a half-second. Then they leave.

You won that tree. But you showed them where the good wood is.

### The Worker Tell

Your worker walks toward a tree. Then veers left. Skips it entirely.

That tree has a nail. *Their* nail. And now you know.

### The Nail Trap

You've planted nails in the biggest remaining cedars. No one knows.

Someone chops. PING. They panic—release too late. Axe breaks. They're stunned.

You walk past them. Cut the tree they were on. Deposit it.

They watch.

### The Last Tree

Last tree falls. The forest goes quiet.

All those trees. All that work. The stumps remain.

Time for a new area.

---

## ENVIRONMENTAL TEXTURE

### The Forest

- Morning light, fog at ground level
- Birds leave when trees fall (not before)
- Woodpecker sounds in the distance—then stops when trees get scarce
- The forest gets quiet as it empties

### Tree Falls

- Trees fall **away from the chopper** in a random direction (anywhere within a 180° arc away from you)
- Slight screen shake
- Branches scatter (low-poly debris)
- Stump remains, then slowly sinks over 30 seconds
- By endgame, the map is full of stumps

### The Trading Post

- Factory hum you can hear before you see it
- Conveyor belts run constantly, empty or not
- Warm light through dirty windows
- The vendor is a silhouette until you're at the counter

---

## AUDIO

Functional. Distinct. Every sound teaches you something.

### Chopping

| Axe | Sound | Feel |
|-----|-------|------|
| Wood | Soft thud | Slow, unsatisfying |
| Stone | Dry crack | Getting somewhere |
| Iron | Metallic bite | Now we're talking |
| Steel | Clean shear | Surgical |

### The Nail Ping

The most important sound in the game.

- Metallic. High-pitched. Brief.
- Plays on 3rd-to-last swing
- Once you've heard it, you'll never miss it
- The silence after is the decision

### Worker Sounds

- Distant chopping (so you know they're working)
- Footsteps on return trips
- Cart rattle (when they have donkey/horse)
- Subtle "ching" when they auto-sell

### Endgame

When 10% of trees remain:
- Forest ambience goes quiet
- Low pulse starts (heartbeat pace)
- Gets slightly faster as trees disappear
- Cuts out on final tree—silence before score

---

## SESSION COMPLETE

### Area Cleared

- Camera slowly pulls out to show the cleared area
- Stats fade in:
  - Trees cut
  - Time to completion
  - Workers hired
  - Resources gathered

### Co-op Celebration

In co-op mode:
- All players step forward
- Shared stats display
- "Area Complete" text appears

---

## MENUS

### Title Screen

- Two axes crossed over a stump
- "WOODCUTTERS" in chunky serif font
- Forest sounds—no music
- Press any button: controller rumbles, axes swing apart
- Both controllers must be connected to proceed

### Player Ready

- Blue and Red stand side by side
- Each player holds A to ready (progress bar fills)
- When both ready: both swing axes, hard cut to map select

### Map Select

- Top-down view of map shapes
- Three sizes: SMALL / MEDIUM / LARGE
- Time estimates below each
- One player picks, other confirms
- Selection locks in with axe-chop sound

### Pause (During Game)

- Splitscreen stays visible but dims
- "PAUSED" overlays
- Options: Resume / Restart / Quit to Menu
- Both players must confirm quit (prevents rage-quits)

---

## PROGRESSION MECHANICS

### Trees Remaining

- No counter. You feel it.
- Forest gets visibly thinner
- Stumps everywhere
- Worker pathing gets longer
- Audio cues (quiet forest)

### Endgame Atmosphere

When few trees remain:
- Forest gets eerily quiet
- Ambient sounds fade
- Time to explore new areas

---

## EDGE CASES WORTH CONSIDERING

### Inventory Full

- You try to chop. Nothing happens.
- Your character shakes head (brief animation)
- Audio: denied sound (soft thunk)
- You have to sell first

### Worker Gets Stuck

- Should basically never happen (simple pathfinding)
- But if it does: worker pauses, recalculates, finds new tree
- Player sees worker standing still for a moment

### Both Players At Same Tree

- First to finish gets the wood
- Other player's swings do nothing once tree is gone
- Their swing animation completes on empty air
- Subtle humiliation

### Trying to Place Nail on Already-Nailed Tree

- Not allowed. Nails are permanent.
- Your nail goes back to inventory
- Audio: denied sound

### Opponent Quits

- "OPPONENT LEFT" appears
- You win by default
- No stats. No fanfare. Just the win.

---

## FINAL DESIGN NOTES

**On Dialogue:**
The vendor isn't funny because he tells jokes. He's funny because he doesn't care, and he says exactly what's true with no padding. Every line should pass the test: "Would a tired old man who's seen a thousand lumberjacks actually say this?"

**On Feedback:**
Every action has a sound and a feel. Chopping feels good. Selling feels good. Hitting a nail feels bad. The game communicates through sensation, not UI.

**On Competition:**
The best games end close. If every game is a blowout, something's wrong with the economy. Nails exist to create comebacks. Worker costs scale to prevent runaway leads. The lead bar exists so you always know, but never feel safe.

**On Atmosphere:**
The forest starts full and alive. It ends empty and quiet. That's the arc. That's the game. You're not saving anything. You're competing to destroy it faster.

*"Nothing personal. Just lumber."*
