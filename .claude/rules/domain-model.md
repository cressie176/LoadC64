# Domain Model

## Overview
Load!64 is a Commodore 64 loader utility that converts binary files into BASIC programs that can be loaded on a C64.

For comprehensive information about the domain model and architecture, please refer to the [GitHub Wiki](https://github.com/cressie176/Load64/wiki).

## Key Domain Concepts

### Core Domain
- **Binary File**: The input file to be converted
- **BASIC Program**: The output C64 BASIC program with embedded data
- **Memory Address**: Target location in C64 memory (must be valid C64 address space)
- **Data Encoding**: Method for embedding binary data in BASIC (e.g., DATA statements)

## Domain Boundaries

### Pure Domain Logic (Core)
- Binary file parsing and validation
- Memory address validation (C64 address space rules)
- BASIC program generation
- Data encoding/decoding algorithms

These should be:
- Pure functions with no side effects
- Thoroughly unit tested
- Independent of infrastructure concerns
- Located in domain modules

### Infrastructure (Adapters)
- File system operations (reading binary files, writing BASIC files)
- Command-line interface
- GUI (iced framework)
- VICE emulator integration (future)

These should:
- Depend on domain logic, not vice versa
- Handle I/O and external system interactions
- Transform external data to/from domain types

## Design Principles
When working with the domain model:
1. Keep domain logic pure and testable
2. Validate at boundaries (parse, don't validate)
3. Use strong types to make invalid states unrepresentable
4. Fail fast and loud with clear error messages
5. Maintain clear separation between domain and infrastructure

## References
- [GitHub Wiki Home](https://github.com/cressie176/Load64/wiki)
- [Design Principles](https://github.com/cressie176/Load64/wiki/Design-Principles)
- [Tech Stack](https://github.com/cressie176/Load64/wiki/Tech-Stack)
