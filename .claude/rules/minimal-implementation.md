# Minimal Implementation Rule

## Core Principle

**ONLY generate code that is EXPLICITLY requested.**

## What This Means

### ❌ DO NOT:
- Add methods that weren't asked for
- Create extensive test suites unless specifically requested
- Implement trait impls (Default, Debug, etc.) unless requested
- Add "helpful" convenience methods
- Generate boilerplate beyond what's needed
- Add documentation unless requested
- Create multiple test cases when one would suffice

### ✅ DO:
- Generate ONLY what the user specifically asks for
- Ask if unclear about what's needed
- Keep it minimal and wait for follow-up requests

## Examples

### ❌ WRONG:
**User:** "Create a Library struct with a HashMap of games"

**Assistant creates:**
- Library struct ✓
- HashMap field ✓
- new() method (not asked for)
- add_game() method (not asked for)
- get_game() method (not asked for)
- has_game() method (not asked for)
- game_count() method (not asked for)
- visit_all_games() method (not asked for)
- Default trait impl (not asked for)
- 8 test functions (not asked for)

### ✅ CORRECT:
**User:** "Create a Library struct with a HashMap of games keyed by game id. Make the game id readable but not mutable."

**Assistant creates:**
```rust
use std::collections::HashMap;
use super::game::Game;

pub struct Library {
    games: HashMap<String, Game>,
}

impl Library {
    pub fn id(&self) -> &str {
        &self.id
    }
}
```

Then STOP and wait for further instructions.

## The Test

Before generating ANY code beyond what's explicitly requested, ask yourself:

**"Did the user specifically ask for this?"**
- NO → Don't generate it
- YES → Generate only that

## Enforcement

This rule is **absolute**. If in doubt, generate LESS, not more.

Wait for the user to ask for additional functionality.
