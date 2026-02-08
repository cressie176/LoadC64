# Function Ordering Rule

## Core Principle

**Order functions from high-level to low-level: callers before callees.**

## What This Means

Within a module or impl block, arrange functions so that:
1. High-level functions that orchestrate logic come first
2. Lower-level helper functions they call come after

This creates a top-down reading flow - you see what the code does before seeing how it does it.

## Example

### ✅ CORRECT (High to Low):
```rust
impl App {
    // High-level - orchestrates the process
    fn create_carousel_item(game: &Game, layout: &Layout, index: usize) -> Container {
        let width = layout.game_width(index);
        let height = layout.game_height(index);
        let img = Self::create_game_cover(game, width, height);
        Self::create_game_container(img, width, height)
    }

    // Low-level - creates image
    fn create_game_cover(game: &Game, width: f32, height: f32) -> Image {
        // ...
    }

    // Low-level - creates container
    fn create_game_container(img: Image, width: f32, height: f32) -> Container {
        // ...
    }
}
```

### ❌ WRONG (Low to High):
```rust
impl App {
    // Low-level first - reader has to scroll to understand usage
    fn create_game_cover(game: &Game, width: f32, height: f32) -> Image {
        // ...
    }

    fn create_game_container(img: Image, width: f32, height: f32) -> Container {
        // ...
    }

    // High-level last - this should be first
    fn create_carousel_item(game: &Game, layout: &Layout, index: usize) -> Container {
        let img = Self::create_game_cover(game, width, height);
        Self::create_game_container(img, width, height)
    }
}
```

## Benefits

- Easier to understand the overall flow
- High-level intent is immediately visible
- Implementation details are deferred
- Natural reading order (what, then how)

## Special Cases

**Public vs Private:** Within each visibility group, still order high-to-low:
```rust
impl MyStruct {
    // Public high-level
    pub fn main_operation() { ... }

    // Public low-level (if needed)
    pub fn public_helper() { ... }

    // Private high-level
    fn orchestrate() { ... }

    // Private low-level
    fn internal_helper() { ... }
}
```

## Enforcement

When adding new functions, place them according to their level of abstraction, not chronologically.
