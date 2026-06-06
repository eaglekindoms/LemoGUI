# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.



## Code Review Workflow

When proposing code changes (bug fixes, refactors, new features), follow this mandatory review process:

1. **Describe each problem and solution in detail** — for every issue, explain:
   - What the bug/problem is and why it causes incorrect behavior
   - The proposed fix with code snippets
   - Any edge cases or risks introduced by the fix

2. **Submit to `codex-cli-dispatcher` agent for review** — send all problems and solutions to the agent and require it to give an explicit **pass** or **fail** verdict on each item and an overall conclusion.

3. **Act on the verdict:**
   - If the overall verdict is **fail**: revise the flagged items and re-submit to the agent for another review round. Repeat until all items pass.
   - If the overall verdict is **pass**: proceed with applying the changes.

4. **Timeout / no-response fallback** — if the `codex-cli-dispatcher` agent times out or returns no response, **do not cancel the changes**. Instead, switch to manual review:
   - Present the full problem-and-solution description to the user in the chat.
   - Ask the user to give an explicit **pass** or **fail** verdict before proceeding.
   - Once the user approves, apply the changes as if the automated review had passed.

Do not apply any code changes before either the Codex review or the manual user review passes.

## Session Logging

After every conversation session, append a summary of all user inputs and assistant responses to:
`/home/axi/.claude/projects/-home-axi-workspace-code-cv/memory/session_log.md`

Format each session as:

```
## YYYY-MM-DD

**用户:** <user message>

**助手:** <assistant response summary>

---
```

**Rules:**
- Record every user input and the assistant's response (summarized if long).
- Record code modifications: include file path and a brief description of what changed.
- Always update `MEMORY.md` index if new memory files are created.

## Karpathy Coding Guidelines

Behavioral guidelines to reduce common LLM coding mistakes.

**Tradeoff:** These guidelines bias toward caution over speed. For trivial tasks, use judgment.

### 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:
- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

### 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

### 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:
- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.

When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: Every changed line should trace directly to the user's request.

### 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:
- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:
```
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
```

Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.
