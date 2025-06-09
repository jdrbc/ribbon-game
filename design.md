# goal

1 v 1

two players opposing sides, competing to reach the end of the opposing player's arena (nidhogg style), fixed number of times to win game, each player has bow

-------------
x->       <-x
-------------

## Core Mechanics
- **Arena**: 3D space, single lane (long and narrow to promote conflict)
- **Chunk Size**: 15 seconds running time end-to-end per chunk
- **Combat**: Physics-based bow fighting, instant death on hit
- **Movement**: Free movement with jump and dodge abilities
- **Dodge**: Dash with i-frames, can dodge in air
- **Advancement**: Players advance toward opponent's side. If opponent falls too far behind, they respawn in front of advancing player
- **Respawn Logic**: Distance threshold triggers respawn 20 meters in front of advancing player
- **Win Condition**: When player advances off edge of opponent's side, they choose new biome to add to map. Battle restarts from middle. **Game ends when a player falls off 5 times total.**
- **Biome System**: Start with forest biome only (TODO: add biome variety and player choice)

## Technical Specs
- **Rendering**: 3D with 3rd person camera (Risk of Rain style) - **Each player runs their own client**
- **Networking**: **GGRS + Matchbox** - P2P with rollback netcode, desktop-only
- **Network Authority**: Authoritative Replication (AR) with turn-based authority - shooter owns their arrows
- **Prediction**: Client prediction for movement, server validation for hits (simplest approach)
- **Physics**: Avian 3D for projectile physics and player movement
- **Engine**: Bevy 0.16 + egui
- **Map System**: Persistent biome generation - map grows and stays, previous biomes remain
- **State Management**: Player components for individual state, central GameState for match tracking

## Combat Details
- **Bow**: Single arrow type, quick reload, no catching/deflecting arrows
- **Health**: Instant death on arrow hit
- **Dodge**: Dash with invincibility frames, usable in air

## Development Roadmap
**Step 1: Core Movement**
- 3D rendering of player box on plane
- WASD movement controls for single player

**Step 2: Multiplayer**
- P2P networking implementation
- Synchronized player movement between clients

**Step 3: Combat & Mechanics**  
- Bow mechanics + instant death system
- Dodge system with i-frames
- Biome generation system + win condition tracking (5x)

**Future:**
- Infinite minecraft crafting -> place bits in a grid -> create unique items / creatures

