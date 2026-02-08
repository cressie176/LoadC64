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
[[vice.arg]]
values = ["-joydev1", "1"]

[[vice.arg]]
values = ["!-autostart-warp"]

[[vice.arg]]
values = ["-sound"]
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
[[vice.arg]]
values = ["!-autostart-warp"]
```

#### Change joystick configuration
```toml
[[vice.arg]]
values = ["-joydev1", "1"]

[[vice.arg]]
values = ["-joydev2", "0"]
```

#### Complex override
```toml
[[vice.arg]]
values = ["-joydev1", "2"]

[[vice.arg]]
values = ["!-autostart-warp"]

[[vice.arg]]
values = ["!-VICIIdscan"]

[[vice.arg]]
values = ["-sound"]

[[vice.arg]]
values = ["-soundrate", "48000"]
```

## Special Handling

- The `-autostart` argument is always added last by the emulator
- All VICE command-line arguments are supported
- See `docs/vice-manual/` for complete VICE documentation

## Reference

See `example-override.toml` for a complete example with comments.
