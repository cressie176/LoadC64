# VICE Joystick Mapping File Format

## Overview
VICE joystick mapping files (.vjm) define how physical controller inputs are mapped to C64 joystick/keyboard actions.

## File Format

A joystick map is read in as a patch to the current map.

### Comment Lines
Comment lines start with `#`:
```
# This is a comment
```

### Keyword Lines
Keyword lines start with `!keyword`:
- `!CLEAR` - Clear all existing mappings

### Mapping Lines
Normal mapping lines have the format:
```
joynum inputtype inputindex action [action_parameters]
```

## Input Types

| Value | Type   | Description                          |
|-------|--------|--------------------------------------|
| 0     | axis   | Analog stick axis                    |
| 1     | button | Button press                         |
| 2     | hat    | D-pad/hat switch                     |
| 3     | ball   | Trackball (rare)                     |

**Note:**
- Each axis has 2 inputindex entries (positive/negative direction)
- Each hat has 4 inputindex entries (up/down/left/right)

## Actions

| Value | Parameters        | Description                                    |
|-------|-------------------|------------------------------------------------|
| 0     | none              | No action                                      |
| 1     | port pin          | Joystick (pin: 1=up, 2=down, 4=left, 8=right, 16=fire) |
| 2     | row col           | Keyboard key press                             |
| 3     | none              | Map                                            |
| 4     | none              | UI activate                                    |
| 5     | path&to&item      | UI function                                    |

## Example Mapping

```
!CLEAR

# Left Thumbstick (left=4, right=8, up=1, down=2)
0 0 1 1 4   # Axis 1 negative = left
0 0 0 1 8   # Axis 0 positive = right
0 0 3 1 1   # Axis 3 positive = up
0 0 2 1 2   # Axis 2 positive = down

# D-pad
0 2 0 1 1   # Hat up = joystick up
0 2 1 1 2   # Hat down = joystick down
0 2 2 1 4   # Hat left = joystick left
0 2 3 1 8   # Hat right = joystick right

# Buttons
0 1 0 2 7 4 # Button 0 = Space key (row 7, col 4)
0 1 1 1 16  # Button 1 = Fire
```

## Common Joystick Pins

| Pin | Direction |
|-----|-----------|
| 1   | Up        |
| 2   | Down      |
| 4   | Left      |
| 8   | Right     |
| 16  | Fire      |

## Special Key: Restore

The Restore key on the C64 keyboard uses a special row value:
```
0 1 8 2 -3 0  # Select button = Restore key
```

## References

- VICE Manual Section 1.2 - Keyboard and Joystick
- VICE Manual Section 6 - Resource Configuration
- [logitech-f310.md](logitech-f310.md) - Example controller configuration for Load!64
