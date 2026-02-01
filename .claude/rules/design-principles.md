# Design Principles

## Overview
Load!64 follows a set of core design principles to ensure the codebase remains maintainable, testable, and easy to reason about.

For detailed information, please refer to the [Design Principles page on the GitHub Wiki](https://github.com/cressie176/Load64/wiki/Design-Principles).

## Core Principles

### 1. Small and Explicit
- Keep the codebase small and easy to understand
- Favor explicit code over clever abstractions
- Each component should have a clear, singular purpose
- Avoid premature optimization or over-engineering

### 2. Clean Domain Model
- Maintain clear boundaries between domain logic and infrastructure
- Domain logic should be pure and free of side effects
- Use strong types to encode business rules
- Make invalid states unrepresentable through the type system

### 3. Testable by Design
- Write tests first (TDD approach)
- Pure functions are inherently testable
- Separate testable logic from I/O and side effects
- Aim for high test coverage of domain logic

### 4. Fail Fast and Loud
- Validate inputs at system boundaries
- Use Result types for operations that can fail
- Provide clear, actionable error messages
- Don't silently ignore errors or use default values

### 5. Continuous Refactoring
- Refactor as you go to keep code clean
- Don't be afraid to improve existing code
- Keep functions small and focused
- Extract reusable patterns when they emerge naturally (not before)

## Code Organization

### Layered Architecture
```
src/
├── domain/       # Pure domain logic (no dependencies on infrastructure)
├── infrastructure/ # I/O, file system, external systems
└── ui/           # GUI (iced framework)
```

### Dependency Rules
- Domain has no dependencies on infrastructure or UI
- Infrastructure and UI depend on domain
- Dependencies point inward (toward domain)

## Testing Strategy
- Unit tests for all domain logic
- Integration tests for infrastructure adapters
- Property-based tests for complex algorithms
- Test edge cases and error conditions

## When to Apply These Principles
- **Always**: Keep functions small, use clear names, separate pure logic from side effects
- **When adding features**: Start with tests, design domain model first, wire up infrastructure last
- **When refactoring**: Look for opportunities to extract pure functions, improve types, clarify boundaries
- **When reviewing code**: Check that principles are followed, suggest improvements

## References
- [GitHub Wiki - Design Principles](https://github.com/cressie176/Load64/wiki/Design-Principles)
- [GitHub Wiki - Tech Stack](https://github.com/cressie176/Load64/wiki/Tech-Stack)
- [GitHub Wiki Home](https://github.com/cressie176/Load64/wiki)
