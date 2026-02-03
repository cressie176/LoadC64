# Strict Encapsulation Rule

## Core Principle

**NEVER access private fields of objects across module boundaries.**

If you find yourself wanting to access a private field, you are breaking encapsulation. STOP and refactor.

## What This Means

### ❌ NEVER DO THIS:
```rust
// In library.rs
let value = position.section_idx;  // Private field access - WRONG!
let value = game.title;             // Private field access - WRONG!
```

### ✅ ALWAYS DO THIS:
```rust
// Objects provide behavior, not data access
position.do_something_with_section();
game.visit(|title, ...| { /* use title here */ });
```

## When You're Tempted to Access a Field

**Ask yourself:**
1. Does this object need a behavior method instead?
2. Should the calling code be moved into the object?
3. Am I treating an object like a data structure?

**Then:**
- Add a behavior method to the object
- Move the logic into the object
- Use visitor pattern to access data without exposing it

## Rules for Rust Modules

In Rust, privacy is at the **module** level:
- Private fields are accessible within the same module
- Private fields are NOT accessible from other modules
- If you're in a different .rs file, you can't access private fields

**This is enforced by the compiler** - if you access a private field across modules, compilation will fail.

## Coordination Between Objects

When objects need to coordinate (like Position and Library):

**Wrong Way:**
```rust
// Position accessing Library internals
library.sections[position.section_idx]  // WRONG!

// Library accessing Position internals
pos.section_idx  // WRONG!
```

**Right Way:**
```rust
// Position delegates to Library
position.next_section(library)  // Library returns new Position

// Library provides pub(super) helpers that Position calls
library.next_section_from(section_idx)  // Takes primitives, returns Position
```

## The Test

Before writing any line of code that accesses a field:

**Ask: "Is this field in a different module than my current code?"**
- YES → You're breaking encapsulation. Stop and refactor.
- NO → OK to access (same module)

## Enforcement

This rule is **absolute**. There are no exceptions for convenience or "just this once."

Every violation must be refactored immediately.
