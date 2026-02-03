# Guard Conditions Rule

## Core Principle

**ALWAYS use guard conditions (early returns) instead of if/else blocks.**

## What This Means

### ❌ WRONG:
```rust
fn process(&self, value: &str) -> bool {
    if is_valid(value) {
        do_something();
        do_something_else();
        true
    } else {
        false
    }
}
```

### ✅ CORRECT:
```rust
fn process(&self, value: &str) -> bool {
    if !is_valid(value) {
        return false;
    }
    do_something();
    do_something_else();
    true
}
```

## Benefits

- Reduces nesting levels
- Makes the "unhappy path" explicit and visible
- Keeps the main logic at the lowest indentation level
- Easier to read and understand the flow
- Prevents deeply nested code

## Application

Use guard conditions for:
- Validation checks
- Null/None checks
- Error conditions
- Preconditions
- Any condition that should exit early

## Enforcement

This rule is **absolute**. Every if/else should be checked to see if it can be rewritten as a guard condition.
