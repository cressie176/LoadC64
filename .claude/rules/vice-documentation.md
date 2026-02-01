# VICE Documentation Reference

## Overview
The VICE emulator documentation is available locally in `docs/vice-manual/`. Refer to these sections when working on Load!64 features to ensure compatibility and correct implementation.

## Key Documentation Sections

### Command-Line Tools (vice_14.html)
**When to reference:**
- Implementing disk image creation/manipulation
- Understanding .prg file structure
- Converting BASIC programs
- Working with VICE utilities

**Key tools:**
- `c1541` - Disk image tool for creating/manipulating .d64 files
- `petcat` - BASIC program converter (tokenize/detokenize)
- `cartconv` - Cartridge conversion (may be useful for ROM loading)

### File Formats (vice_17.html)
**When to reference:**
- Understanding C64 file formats
- Implementing binary file loading
- Creating .prg files
- Working with disk images

**Key formats:**
- PRG files - Program format with load address
- D64 files - Disk image format
- T64 files - Tape image format
- CRT files - Cartridge format

### C64 Emulator Features (vice_7.html)
**When to reference:**
- Understanding C64 memory layout
- Working with BASIC commands
- Implementing SYS calls
- Understanding hardware limitations

**Key topics:**
- Memory map (where to load binaries safely)
- BASIC ROM entry points
- Hardware features (VIC-II, SID, CIA)
- C64 models and compatibility

### Resource Configuration (vice_6.html)
**When to reference:**
- Testing with different C64 configurations
- Understanding emulator settings
- Automating VICE for testing

**Key resources:**
- Command-line options for x64
- Configuration file format
- Video/audio settings for testing

### Keyboard and Joystick (vice_1.html, Section 1.2)
**When to reference:**
- Understanding C64 keyboard layout
- Implementing user interaction in test programs
- Debugging input issues

### Monitor/Debugger (vice_12.html, vice_13.html)
**When to reference:**
- Debugging generated BASIC/machine code
- Verifying memory loads
- Testing binary execution
- Understanding memory state

**Key features:**
- Text monitor commands
- Memory inspection
- Breakpoints and debugging

## When to Consult Documentation

### Always Reference When:
1. **Implementing file format handling** - Check vice_17.html for specifications
2. **Working with VICE command-line** - Check vice_14.html for tool usage
3. **Dealing with memory addresses** - Check vice_7.html for memory map
4. **Creating disk images** - Check vice_14.html for c1541 usage
5. **Converting BASIC code** - Check vice_14.html for petcat usage

### Proactive Reading:
- Before implementing a feature that interacts with VICE
- When encountering C64-specific behavior or limitations
- When designing the binary-to-BASIC conversion algorithm
- When setting up automated testing with VICE

## How to Reference

When you need information from the VICE manual:
1. Read the relevant HTML section directly
2. Reference specific sections by filename and section number
3. Extract relevant command-line options or format specifications
4. Cite the documentation when making design decisions

Example: "According to vice_17.html, PRG files start with a 2-byte little-endian load address..."

## Documentation Location

- **HTML**: `docs/vice-manual/vice_*.html`
- **PDF**: `docs/vice-full-manual.pdf` (for offline reference)
- **Table of Contents**: `docs/vice-manual/vice_toc.html`
- **Online**: https://vice-emu.sourceforge.io/vice_toc.html
