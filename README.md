# Ribbon Game

A multiplayer 3D arena game built with Bevy 0.16 and Avian 3D physics. Brought to you by the professionals at Bespoke Development.

## Current Status

The project was a mess when we got here. Dependency hell, a UI that was more of a black screen, and a networking layer that was little more than a prayer. We fixed it.

### ✅ Step 1 & 2: Core Systems & Networking - Rescued & Implemented

-   **Project Rescued**: We dragged this project out of dependency hell. It now compiles, runs, and doesn't cry about `Cargo.toml`.
-   **UI Restored**: The "grey screen of death" has been vanquished. The UI is back, powered by a correctly configured `bevy_egui` plugin.
-   **Networking Online**: The P2P networking is no longer a TODO item. It's implemented. We're using `bevy_ggrs` and `bevy_matchbox` to create and join rooms. It just works.
-   **GGRS Integrated**: The `GGRSPlugin` is configured, rollback schedules are in place, and networked player spawning is a reality. Local and remote players now coexist in harmony.
-   **State Management Overhauled**: We've introduced a proper `SinglePlayer` state to isolate testing from the real, networked action.

### 📋 Next Steps

With the foundations solid, we're moving on to what matters: the game itself.

1.  **Combat & Mechanics**: Bows, arrows, dodging, and death. The core loop is next.
2.  **Game Flow**: Respawns, advancement, and win conditions.
3.  **Biome System**: Because a plain arena is for amateurs.

## Controls

-   **WASD**: Move
-   **Space**: Jump

## Running the Game

```bash
cargo run
```

The game will start in the main menu. From there, you can create or join a multiplayer game, or jump into single-player mode for testing.

## Development Notes

-   **Bevy 0.16**: The latest and greatest.
-   **Avian 3D**: For all your physics needs.
-   **GGRS + Matchbox**: The foundation of our superior networking stack.
-   **Bespoke Attitude**: We don't just write code. We write it with conviction.

***

*This README has been updated to reflect the reality of a project now in capable hands. The previous version was... optimistic.* 