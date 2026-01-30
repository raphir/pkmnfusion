# Battle System Design Summary

## Core Philosophy

**Fights reward foresight + smart play, not grind.**

Corollary: No single solution; multiple viable paths per encounter.

---

## Move Pool System (Badge-Gated Progression)

### Move Tier Structure
- Moves are globally assigned to badge tiers (1-8)
- Player mons unlock access to move tiers upon earning badges
- **No grinding required.** Caught mon is immediately deployable.

### Caught Mons
- Know **4 moves max** (standard Pokemon)
- Moves are **randomized** from available pool at catch badge tier
- Rarity weighting: Each move has a spawn chance (e.g., Pikachu at badge 5: 40% Quick Attack, 10% Thunderbolt)
- Rarity scales with badge tiers: Thunderbolt at badge 5 = 10% chance; badge 6 = 20% chance
- **Discovery mechanic:** Catching multiple mons is interesting—you don't know what moveset you get

### Wild Mons
- Know **4 moves** from their badge tier availability
- **Fixed movesets per species/tier** (predictable, players can plan counterplay)

### Move Tutor System
- **Located in towns** (time-gated, not portable)
- **Teaches/swaps moves** from unlocked tiers
- **Cost scales by badge count** (early game cheap, late game expensive)
- **Purpose:** Tweak strategy per gym, prevent spam-swapping every encounter
- **Currency reward:** Earn tutor currency per badge earned (enables teaching new moves)

### Caught Mon Moveset Progression
- Mon caught at badge 1, moveset stays static
- Upon earning badge X, new moves become *available* to teach (not automatic)
- Player uses tutor currency to add/swap moves from newly unlocked tier

---

## Gym Encounters: Foreshadowing & Asymmetry

### Gym Leader Move Access
- Gym leader at gym N can use moves from tiers **1 to (N+1)**
- Creates **preview mechanic** of moves coming in next tier
- Ensures **1-tier asymmetry** (manageable gap, not overwhelming)

### Foreshadowing Strategy
- **One signature move** per gym leader's ace (from next tier)
- Shows **move synergy** and **combo play** to player
- Example (Gym 3, Miltank):
  - Miltank knows Curl Up (tier 2, available to player)
  - Miltank's ace also knows Rollout (tier 3, player can learn after winning)
  - Teaches player that move synergies exist + combos can solve "impossible" encounters
  - Player learns this is a viable strategy pattern

### Gym Difficulty Progression
- **Gym 1:** Leader has 1 move from tier 2
- **Gym 2:** Leader has 1 move from tier 3
- **Gym 3:** Leader has 1 move from tier 3
- **Gym 4:** Leader has 1 move from tier 5
- **Gym 7:** Leader has 1 move from tier 8
- **Gym 8:** Leader has tier 8 moves (final gym, no higher tier available)

### Dialogue & Signaling
- NPC dialogue hints at upcoming challenge: *"I will show you strategies you wouldn't even dream of. Prepare to be steamrolled."*
- Sets expectations, doesn't spoil specifics
- Player can adapt knowing a tough encounter is coming

---

## Fairness & Counterplay Philosophy

### Asymmetry ≠ Unfairness
- Gym leaders have access to moves player hasn't unlocked yet
- **But:** Multiple counterplay paths exist within current tiers
- Example (Gym 4 Electric, opponent knows Thunderbolt/tier 5):
  - Player can't learn Thunderbolt yet
  - Player **can** use Ground types (super-effective, always available)
  - Player **can** use Water types (resist Electric, accessible)
  - Player **can** use status moves (Thunder Wave, always available)
  - Player **can** switch intelligently (free, always works)

### Precedent: Miltank (Gold/Silver)
- Player couldn't get Miltank (no symmetrical access)
- But could get Machoke (Fighting counter), use status, switch smartly
- **Result:** Multiple viable solutions, none requiring opponent's exact tools
- This is the fairness model being replicated

---

## Encounter Design Principles

### Per-Gym Questions
1. **What's the core threat?** (Raw damage? Setup sweep? Status stall?)
2. **What moveset enables that threat?** (For this specific encounter)
3. **What are 3+ viable counterplay paths?** (Not just one "correct" team)
4. **Do those mons + moves exist in catch pool?** (If no: design new options)
5. **Can a player access 2+ paths without grinding?** (If no: broaden availability)

### Move Synergy as Innovation
- Don't design individual moves for balance—design **movesets for encounters**
- Encounter drives moveset, not system balance
- Example: Miltank doesn't need priority Curl Up; encounter design forces respect via setup+sweep synergy

---

## Type Effectiveness & Damage

### Softened Multipliers
- **Super effective:** 1.3x (vs Pokemon's 2x)
- **Weak to:** 0.7x (vs Pokemon's 0.5x)
- **Dual resistance:** 0.49x (0.7 × 0.7, vs Pokemon's 0.25x)

### Dual-Type Move Mechanics
- Moves have **primary + secondary type** with weight distribution
- Example: Mud Bubble (primary Ground 60%, secondary Water 40%)
- Damage formula: `base_power × (primary_effectiveness × 0.6 + secondary_effectiveness × 0.4)`
- No STAB (removed entirely as hidden mechanic)

---

## Stat System

### Philosophy
- Stats exist; don't hide them (transparency > deception)
- Accept power level disparity between species (Magikarp ≠ Gyarados)
- **Move pools compensate**, not stat overhauls
- Example: Magikarp's low Atk is offset by access to priority/utility moves

### Progression Without Grinding
- Hybrid system: Stats scale via badge count (implicit leveling)
- Catch-level = badge-tier-equivalent stats
- Catch a mon at badge 5? Stats match badge-5-equivalent, immediately useful
- No separate "training grind" before deployment

---

## Branching Gym Paths (Gyms 4-6)

### Structure
- 3 paths; 3 gyms per path (9 total middle gyms)
- Paths are **challenge-type-based, not difficulty-based**
  - Path A: Many mons, status-heavy (resource management)
  - Path B: Fewer mons, setup-heavy (prediction & switches)
  - Path C: Mono-type threats (coverage/diversity test)
- Universal badge scaling (all paths at badge 5 = similar baseline difficulty)

### Path Selection
- Before choosing: Dialogue/NPC hints reveal gym types & leader styles
- Not railroaded—guided by information
- After choosing: Committed until convergence (gyms 7-8)
- Switching paths costs narrative (not recommended mid-story)

---

## Future Expansion (Post-Gym-8)

### Pokemon League
- Traditional Elite Four + Champion
- **Tier 9+ moves** created to keep late-game fresh
- Design phase: deferred until gyms 1-8 locked

---

## Unresolved (Acceptable Constraints)

1. **Blind first-run:** Players may enter unprepared. Design assumes ~1 retry per gym acceptable?
2. **Soft-lock risk:** If all wild mons strong + player caught weak team, leaving to train elsewhere acceptable? Or open-world require catch-anywhere-play-anywhere?
3. **Move signaling:** How visible are wild movesets? Pokedex? Trainer dialogue? Trial encounters? (Decision TBD)

---

## Replayability Hooks

1. **Randomized caught mon movesets** → Different mons, different role opportunities
2. **Move tier preview** → "I saw this move at gym 4, now I can use it for gym 5" (strategic advantage 2nd playthrough)
3. **Branching paths (4-6)** → Three different puzzle types, different encounters
4. **Encounter-specific movepools** → Same mon (Pikachu) plays differently per gym
5. **Move tutor cost** → Replays can optimize movesets knowing future challenges

---

## Key Learnings

- **Encounter-first design:** Build gym challenge, derive movepool. Not: design moves globally, hope encounters work.
- **Asymmetry teaches:** Gym leader with future moves teaches player that move synergies exist and combos solve hard problems.
- **Respect time:** Caught mon immediately useful. No grinding gate before deployment.
- **Memorable > balanced:** Design memorable, tough encounters with multiple solutions (not perfectly balanced teams).
