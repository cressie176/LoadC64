# VICE Configuration

Load!64 uses a flexible TOML-based configuration system for VICE emulator arguments.

## Default Configuration

The default VICE arguments are defined in `default.toml`. These are loaded automatically when launching any game.

## Game-Specific Overrides

To customize VICE settings for a specific game:

1. Create a file named `vice.toml` in the game's directory
2. Add your custom arguments using the format shown below

### Override Format

```toml
args = [
  ["-joydev1", "1"],           # Replace: changes existing argument
  ["!-autostart-warp"],        # Remove: deletes argument from defaults
  ["-sound"],                  # Add: adds new argument not in defaults
]
```

### How Merging Works

When a game-specific override exists:

1. **Default config is loaded** from `assets/vice/default.toml`
2. **Game override is loaded** from `<game-dir>/vice.toml`
3. **Merge happens**:
   - Arguments starting with `!` remove that argument from defaults
   - Other arguments replace existing ones (matched by first element)
   - New arguments are added

### Examples

#### Remove warp mode for a specific game
```toml
args = [
  ["!-autostart-warp"],
]
```

#### Change joystick configuration
```toml
args = [
  ["-joydev1", "1"],
  ["-joydev2", "0"],
]
```

#### Complex override
```toml
args = [
  ["-joydev1", "2"],           # Replace joystick setting
  ["!-autostart-warp"],        # Remove warp mode
  ["!-VICIIdscan"],           # Remove double-scan
  ["-sound"],                  # Add sound
  ["-soundrate", "48000"],     # Add custom sound rate
]
```

## Special Handling

- The `-autostart` argument is always added last by the emulator
- All VICE command-line arguments are supported
- See `docs/vice-manual/` for complete VICE documentation

## Reference

See `example-override.toml` for a complete example with comments.
