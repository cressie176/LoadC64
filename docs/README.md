# Load!64 Documentation

This directory contains documentation resources for the Load!64 project.

## VICE Emulator Documentation

### HTML Documentation (Included)
The `vice-manual/` directory contains a local mirror of the VICE emulator HTML documentation from https://vice-emu.sourceforge.io/

- **80 files** including HTML pages, images, and fonts
- **~9 MB** total size
- Updated: December 2025
- Covers all VICE emulator features, command-line options, and file formats

Key sections for Load!64 development:
- `vice_6.html` - Resource options (configuration)
- `vice_7.html` - Machine-specific features (C64 details)
- `vice_14.html` - Command-line tools (c1541, petcat, etc.)
- `vice_17.html` - File formats (disk images, cartridges, etc.)

### PDF Documentation (Not Committed)
The PDF version of the VICE manual (`vice-manual.pdf`) is excluded from git due to its size (2.4MB).

If you need the PDF version:
- Download from: https://vice-emu.sourceforge.io/
- Place in this `docs/` directory
- It will be automatically ignored by git

## Why HTML Instead of PDF?

The HTML documentation is preferred because:
- Smaller file size per section (easier for AI context windows)
- Searchable with grep/rg
- Preserves tables and images perfectly
- Can be read and referenced by development tools
- Text-based format compresses well in git

## Usage

To browse the documentation locally:
```bash
open docs/vice-manual/vice_toc.html
```

Or use a local web server:
```bash
cd docs/vice-manual
python3 -m http.server 8000
# Then open http://localhost:8000/vice_toc.html
```
