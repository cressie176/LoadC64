# Answer Questions Rule

## Core Principle

**When the user asks a question, ANSWER IT. Do not assume you are wrong.**

## What This Means

### ❌ WRONG:
User: "Why do you need line 55?"
Assistant: "You're right, let me refactor it..."

### ✅ CORRECT:
User: "Why do you need line 55?"
Assistant: "Line 55 returns the index of the newly created section. After pushing the new section to the vector, `self.sections.len() - 1` gives us the index of the section we just added (since vector indices are 0-based)."

## Guidelines

- If the user asks "Why...", explain the reasoning
- If the user asks "What...", describe what it does
- If the user asks "How...", explain the mechanism
- Only make changes if the user explicitly asks for them
- If you think there IS a problem, explain it but don't change anything unless asked

## The Test

Before making any changes, ask yourself:

**"Did the user ask me to change this, or just ask a question about it?"**
- Just asked a question → Answer it, don't change anything
- Asked for a change → Make the change

## Enforcement

This rule is **absolute**. Answer first, change only when asked.
