# goal

1 v 1

two players opposing sides, competing to reach the end of the opposing player's arena (nidhogg style), fixed number of times to win game, each player has bow

-------------
x->       <-x
-------------

## Core Mechanics
- **Arena**: 3D space, long & wide
- **Movement Speed**: sensible base movement speed
- **Combat**: Physics-based bow fighting with gravity effects, instant death on hit
- **Movement**: Free 3D movement with jump and dodge abilities
- **Dodge**: Dash with 1-second i-frames, 7-second cooldown, can dodge in air, no chaining
- **Advancement**: Players advance toward opponent's side. No advancement allowed until at least one kill occurs. Only the player who last killed their opponent can advance.
- **Respawn Logic**: 20m distance threshold triggers respawn in front of advancing player. No spawn protection.
- **Win Condition**: First player to advance off opponent's east/west edge 5 times wins. Falling off north/south edges counts as death, not advancement.
- **Biome System**: Start with forest biome only. Winner chooses new biome when advancing.

## Technical Specs
- **Rendering**: 3D with fixed 3rd person camera behind player - **Each player runs their own client**
- **Networking**: **GGRS + Matchbox** - P2P with rollback netcode, 30 ticks/second
- **Network Authority**: Host-based authority where needed, owner-based for arrow hit detection
- **Prediction**: Client prediction for movement, network packets for arrow trajectory synchronization
- **Physics**: Avian 3D for projectile physics and player movement with character controller
- **Engine**: Bevy 0.16 + bevy-egui for UI
- **Platform**: Mac and Windows desktop
- **Map System**: Persistent biome generation - map grows and stays, previous biomes remain
- **State Management**: Player components for individual state, central GameState for match tracking
- **Error Handling**: Comprehensive logging for network disconnections, desyncs, and crashes

## Combat Details
- **Bow**: Single arrow type, 1-second reload time, no catching/deflecting arrows
- **Shooting**: Can shoot while moving, jumping, or dodging. Hold mouse button for power shot (affects range)
- **Arrow Physics**: Realistic gravity drop-off and arc trajectory
- **Health**: Instant death on arrow hit
- **Dodge**: Dash with 1-second invincibility frames, 7-second cooldown, usable in air, no chaining

## Controls
- **Movement**: WASD for directional movement
- **Jump**: Spacebar
- **Dodge**: Shift key
- **Aim/Shoot**: Mouse aim, click to shoot, hold for power shot

## Development Roadmap
**Step 1: Core Movement**
- 3D rendering of player box on plane with Avian 3D character controller
- WASD movement controls for single player
- Fixed 3rd person camera implementation

**Step 2: Multiplayer Foundation**
- EGUI lobby system (create/join/start game, list members)
- P2P networking with GGRS + Matchbox implementation
- Synchronized player movement testing between clients

**Step 3: Combat & Mechanics**  
- Bow mechanics with 1-second reload and power shot system
- Arrow physics with gravity and trajectory
- Instant death + dodge system with i-frames and cooldown
- Win condition tracking (first to 5 advancements)

**Step 4: Game Flow**
- Respawn logic with 20m threshold
- Advancement rules (kill requirement, east/west boundaries) 
- Biome generation system

**Future:**
- Audio system integration
- Advanced biome variety and selection
- Infinite minecraft crafting -> place bits in a grid -> create unique items / creatures

