# Ribbon Game

A multiplayer 3D arena game built with Bevy 0.16 and Avian 3D physics.

## Current Status

### ✅ Completed (Step 1 & 1.1 - Refactored)

- **Project Structure Refactored**: Modular plugin-based architecture
- **3D Rendering Foundation**: 
  - 3D scene with camera system
  - Player entity as 3D capsule
  - Ground plane (150m x 50m arena)
  - Proper lighting and materials
  - Fixed 3rd person camera with smooth following
- **Character Controller**:
  - Avian 3D physics integration
  - WASD movement controls (~5 m/s speed)
  - Improved impulse-based movement system
  - Jump mechanics (Spacebar) - **FIXED**: Now working properly
  - Collision detection with ground - **FIXED**: Player no longer floating
- **Input System**: Separated input handling into dedicated module
- **Physics System**: Dedicated physics module with ground detection
- **State Management**: Proper game state system (MainMenu, Lobby, InGame, GameOver)

### 🚧 Partially Implemented (Step 2 - Networking Foundation)

- **Project Structure**: 
  - Networking module created (currently commented out due to dependency issues)
  - UI module with keyboard-based state transitions
  - Lobby and connection management resources defined
- **Basic State Transitions**: Keyboard controls for testing different game states

### 📋 TODO (Step 2 - Full Implementation)

- **Networking Architecture**: 
  - GGRS rollback netcode integration (dependencies need resolution)
  - Matchbox P2P connections
  - Network input synchronization
- **Lobby System**: 
  - EGUI-based UI (waiting for bevy_egui compatibility)
  - Room creation and joining
  - Player management
- **Multiplayer Features**:
  - Synchronized player spawning
  - Network player movement
  - Client prediction and rollback

## Controls

### Current Game Controls
- **WASD**: Move player
- **Space**: Jump
- **Arrow Up**: Shoot/Aim (placeholder)

### State Navigation (Temporary)
- **Main Menu**: 
  - Enter: Start single player game
  - L: Go to lobby
- **Lobby**: 
  - Enter: Start game
  - Escape: Back to main menu
- **In Game**: 
  - Escape: Back to main menu

## Project Structure

```
src/
├── lib.rs          # Main plugin and module organization
├── main.rs         # Application entry point
├── components.rs   # Game components (Player, NetworkInput, etc.)
├── resources.rs    # Game resources (GameStats, LobbyState, etc.)
├── systems.rs      # Core game systems (scene setup, camera)
├── physics.rs      # Physics-related systems
├── input.rs        # Input handling systems
├── networking.rs   # Network systems (TODO: enable when deps resolved)
└── ui.rs          # UI systems (TODO: enable EGUI when compatible)
```

## Running the Game

```bash
cargo run
```

The game will start in the main menu state. Use the keyboard controls above to navigate between states and test the movement system.

## Development Notes

- **Bevy 0.16**: Latest version with improved ECS and rendering
- **Avian 3D**: Modern physics engine for Bevy
- **Modular Architecture**: Plugin-based system for easy extension
- **Network Ready**: Structure prepared for GGRS integration
- **Cross-Platform**: Configured for Mac and Windows builds

## Next Steps

1. Resolve bevy_egui compatibility for proper UI
2. Fix GGRS dependency issues for networking
3. Implement full lobby system
4. Add multiplayer player synchronization
5. Implement combat mechanics (bow, arrows, dodge) 