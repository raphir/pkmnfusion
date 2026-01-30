# Progression System Design Summary

## Core Principles

**Goal:** Remove grinding, preserve species identity, respect player time.

**Method:** Badge-gated stat scaling + encounter gating + evolution gating.

---

## Stat Scaling System

### Badge-to-Tier-to-Level Mapping
| Tier | Badge | Equivalent Level | Context |
|------|-------|-----------------|---------|
| 1    | 0     | 5               | Pre-badge tutorial |
| 2    | 1     | 13              | First gym beaten |
| 3    | 2     | 21              | Early-mid transition |
| 4    | 3     | 27              | Mid-game baseline |
| 5    | 4     | 33              | Branching paths |
| 6    | 5     | 39              | Late-mid game |
| 7    | 6     | 45              | Endgame prep |
| 8    | 7     | 51              | Pre-Elite Four |
| 9    | 8     | 57              | Champion-tier |
| 10+  | Post-8 | TBD            | Post-game (future design) |

### Stat Calculation
**Use official Pokémon stat formula:**
```
stat = floor((2 × base_stat × level / 100) + level + 10)
HP = floor((2 × base_HP × level / 100) + level + 10)
```

**Lookup process:**
1. Player badge count → Tier → Equivalent level
2. Species base stats + level → Calculate current stats
3. Apply to all team members + wild encounters

**Example:**
- Pidgey badge 0 (tier 1, level 5): Official level 5 Pidgey stats
- Dragonite badge 8 (tier 9, level 57): Official level 57 Dragonite stats
- **Power gaps preserved:** Dragonite 600 BST >> Pidgey 251 BST at all tiers

### Scaling Rules
- **Player team:** Scales with badge count (badge earned = tier unlocked)
- **Wild encounters:** Scale with player badge count at time of encounter
- **Global route scaling:** All routes scale simultaneously with player badges
- **Species identity:** Base stat totals preserved (Dragonite always stronger than Pidgey)

---

## Evolution System

### Evolution Gating
**Species-specific badge requirements:**
- Early-game mons: Badge 1-2 (Rattata → Raticate)
- Mid-game mons: Badge 3-4 (Dratini → Dragonair)
- Late-game mons: Badge 6-8 (Dragonair → Dragonite, Charmeleon → Charizard)

**Design philosophy:** Weak species evolve early, powerful species evolve late.

### Evolution Trigger UX
1. **Badge earned** → One-time dialogue: "Your team feels stronger!"
2. **Team menu** → Static indicator (icon/text) shows evolution-ready mons
3. **Player initiates evolution** via:
   - NPC service at Pokémon Center (free, unlimited), **OR**
   - Cutscene interaction (touch Pokémon), **OR**
   - Consumable item (Rare Candy)
   - **Method TBD—all options viable**

### Evolution Choice Consequences
- **Un-evolved forms:** NO stat compensation (stay proportionally weaker)
- **Player choice respected:** Can keep Dratini un-evolved indefinitely (aesthetic/preference)
- **Tradeoff accepted:** Un-evolved = weaker stats, evolved = stronger stats + new moves
- **Strategic depth:** Evolution changes moveset access (Dragonair learns moves Dratini cannot)
- **Timing matters:** Evolve now for power vs delay for pre-evolution exclusive moves

---

## Encounter & Spawn System

### Route Scaling Model
- **Global scaling:** All routes scale with player badge tier
- **Badge 0 visit:** Route 1 = tier-1 encounters
- **Badge 5 revisit:** Route 1 = tier-6 encounters (same location, stronger Pokémon)
- **Result:** Revisiting routes stays rewarding throughout game

### Species Spawn Gating
**Each species has minimum badge tier to spawn.**

**Example: Desert Zone**
| Species | Tier Min | BST | Role |
|---------|---------|-----|------|
| Sandshrew | 1 | 300 | Early-game |
| Trapinch | 1 | 290 | Early-game |
| Rhyhorn | 4 | 345 | Mid-game |
| Hippopotas | 6 | 330 | Late-game |
| Flygon | 8 | 520 | Endgame |

**Player badge 0:** Only Sandshrew, Trapinch spawn (tier-1 stats)
**Player badge 5:** Sandshrew, Trapinch, Rhyhorn, Hippopotas spawn (all tier-6 stats)
**Player badge 8:** All five species spawn (all tier-9 stats)

### Spawn Rate Shifting
**Spawn rates adjust per badge tier to keep zones fresh:**

**Desert Zone Example:**
- **Badge 0-3:** Sandshrew 50%, Trapinch 50%
- **Badge 4-5:** Sandshrew 20%, Trapinch 20%, Rhyhorn 40%, Hippopotas 20%
- **Badge 6-7:** Sandshrew 10%, Trapinch 10%, Rhyhorn 35%, Hippopotas 35%, Flygon 10%
- **Badge 8+:** Rhyhorn 30%, Hippopotas 30%, Flygon 40%

**Result:** Early Pokémon become rare (still catchable), new dominant species emerge.

---

## Route Obstacles & Exploration

### Obstacle Gating
- **Hard gates:** Cannot pass until badge unlocks obstacle-clearing ability
- **Unlock progression:**
  - Rock Smash: Badge 1
  - Surf: Badge 3 (example)
  - Strength: Badge 5 (example)
  - Fly: Badge 7 (example)

### Hidden Area Design
- Obstacles block sub-areas within routes
- **Hidden areas contain:**
  - Species gated to higher tiers
  - Rare items
  - Optional NPCs
  - Side quests

**Scaling behavior:**
- Badge 1 unlock → Hidden area accessible
- Badge 5 revisit → Hidden area scales to tier-6 (same as main route)
- **No separate scaling:** Hidden areas scale with route globally

### Area-Specific Encounter Tables
- **Each named area has unique encounter table**
- **Design goal:** Every zone feels distinct
- **Species distribution:** Area-wide (not tile-specific)
  - Example: "Tower of the Moon" → Dratini in all water tiles
  - Not: "Dratini only on tile (37, 42)"

**Granular design accepted:** Manual encounter tables per zone for maximum distinctiveness.

---

## Post-Game Tier System

### Current Status: Deferred
- **Scope:** Tiers 1-9 (badges 0-8) fully defined
- **Tier 10+ design:** Future session
- **Confirmed decision:** Player team scales past badge 8
- **Trigger mechanism:** TBD (potential: multi-region badges, Elite Four victories, story milestones)

### Open Questions (Future Session)
1. Does post-game use 16-badge system (multi-region)?
2. What's the final level cap? (75? 100?)
3. How many post-game tiers exist? (10-12? 10-15?)
4. Do Elite Four/Champion use fixed tiers or scale with player?

---

## Special Case: Dragonite (Tier-9 Pokémon)

### Dragonite Gating Rules
- **Dratini:** Catchable tier 2+, BST 300 (early companion)
- **Dragonair evolution:** Badge 3-4 unlocks, BST 420 (mid-game power spike)
- **Dragonite evolution:** Badge 8 unlocks, BST 600 (champion-tier)
- **Wild Dragonite:** Spawns only after badge 8 (tier-9 exclusive encounter)

### Player Progression Paths
**Path A (Early catch):**
1. Catch Dratini badge 2 (tier 3, level 21)
2. Evolve → Dragonair badge 4 (tier 5, level 33)
3. Evolve → Dragonite badge 8 (tier 9, level 57)
4. **Result:** Early companion grows into endgame powerhouse

**Path B (Late catch):**
1. Encounter wild Dragonite after badge 8 (tier 9, level 57)
2. **Result:** Immediate endgame-tier capture for players who missed Dratini

---

## IV/EV System: REMOVED

- **IVs removed:** No hidden genetic variance between individuals
- **EVs removed:** No battle-grinding progression rewards
- **Result:** Every Pidgey at tier 6 has identical stats
- **Benefits:**
  - No variance frustration ("bad IV" catches)
  - First catch = optimal catch
  - Respects player time

**Variance source preserved:** Caught Pokémon have randomized movesets from tier-appropriate move pool (already designed in battle system).

---

## Implementation Notes

### Stat Calculation Process
1. **Input:** Player badge count, Pokémon species
2. **Lookup:** Badge → Tier → Equivalent level
3. **Calculate:** Species base stats + level → Current stats via official formula
4. **Apply:** To all team members + wild encounters

**No manual stat tables needed—formula auto-generates.**

### Encounter Table Design Scope
**Accepted workload:**
- 20+ named areas
- 9 badge tiers per area
- **180+ encounter configurations**
- Each requires: Species list, spawn rates, tier minimums

**Prerequisite:** Define game's Pokémon roster first.

**Design priority:** Granular control for zone distinctiveness over automation.

---

## Content Creation Blockers

### Critical Path (Must Define Before Gym Design)
1. **Pokémon roster:** Which species exist in-game? (150? 250? Original 151?)
2. **Move pool tiers:** Which moves belong to tier-1, tier-2, ..., tier-9? (Separate discussion)
3. **Evolution badge assignments:** 100+ species need tier unlock definitions

### Non-Critical (Can Parallelize)
1. Zone encounter tables (depends on roster)
2. Route obstacle placement (depends on world map)
3. Post-game tier system (deferred to future session)

---

## Key Decisions Made

| Topic | Decision |
|-------|----------|
| **Stat scaling** | Official Pokémon formulas, badge = tier = level |
| **Species power gaps** | Preserved via base stat totals |
| **Wild mon scaling** | Scales with player badge tier globally |
| **Route scaling** | All routes scale simultaneously |
| **Evolution compensation** | None (un-evolved = weaker by choice) |
| **Evolution trigger** | Static indicator + player-initiated (method TBD) |
| **Route obstacles** | Hard gates unlocked by badges |
| **Spawn gating** | Species have badge-tier minimums |
| **Spawn rate shifts** | Dynamic per badge tier (manual design) |
| **Encounter tables** | Granular per-zone design (not automated) |
| **IV/EV** | Removed entirely |
| **Post-game scaling** | Yes (player team scales past badge 8) |
| **Post-game triggers** | TBD (future session) |

---

## Open Questions (Non-Blocking)

1. **Evolution trigger method:** NPC service, item, or cutscene? (Choose during prototyping)
2. **Post-game structure:** 16-badge multi-region? Elite Four tiers? (Future session)
3. **Level cap:** 75 or 100? (Future session)
4. **Pokémon roster size:** 151? 250? Custom selection? (Prerequisite for encounter tables)

---

## Next Critical Steps

1. **Define Pokémon roster** (which species exist in-game)
2. **Move pool tier assignment** (separate discussion—blocks gym encounter design)
3. **Prototype Gym 1 + Route 1** (validate system in practice)
4. **Design 5-10 zone encounter tables** (test spawn gating + rate shifting)
5. **Assign evolution tiers to 50+ species** (common evolution chains)

---

**System Status:** Coherent, implementable, no contradictions. Ready for move pool design + roster definition phase.

**Deferred Topics:** Post-game tier system (10+), evolution trigger method, encounter table automation.
