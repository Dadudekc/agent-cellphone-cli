# Agent Cell Phone CLI - Product Requirement Document (PRD)

## 1. Project Overview
- **Name:** Agent Cell Phone CLI
- **Goal:** Replace PyAutoGUI-based agent control with a headless, CLI-driven execution engine where supervisors issue instructions and a single Rust executor enforces them.
- **Outcome:** Multi-agent orchestration without input conflicts, duplication, or GUI overhead.

## 2. Objectives
- Eliminate keyboard and mouse hijacking by using a CLI.
- Centralize execution under one Rust executor.
- Preserve the last PyAutoGUI build as a fallback.
- Enforce compliance gates: Single Source of Truth (SSOT), deduplication, and monolith limits.
- Enable supervisors to scale infinitely without duplication.
- Provide observability through logs, snapshots, and traces.

## 3. Key Features
### 3.1 Rust Executor (Core Binary)
- JSON-RPC (stdio) interface.
- File edits, diffs, range operations, and locks.
- Gates for deduplication, SSOT, and monolith limits.
- Process runner to handle lint, test, and build commands.

### 3.2 Python Supervisors
- Strategy and planning logic.
- Generate instructions in YAML or JSON.
- Call the Rust executor via RPC.

### 3.3 Messaging Bus
- Inbox and outbox schema.
- Agent → Executor → Logs feedback loop.

### 3.4 Policy Gates
- **SSOT:** one owner per domain.
- **Dedup:** no duplicate helpers.
- **Monolith:** 300 lines of code per file maximum.

### 3.5 Playbook Runner
- YAML pipelines, e.g. `refactor_then_test.yaml`.
- Deterministic task chaining.

### 3.6 Observability
- JSONL logs.
- Failure snapshots.
- Trace capture.

## 4. Constraints
- Must run headlessly with no GUI.
- Agents issue instructions only and never touch files directly.
- Executor must enforce gates before commit.
- Should run cross-platform on Linux, Windows, and Mac.

## 5. Success Metrics
- Zero duplication after migration; dedup gate blocks pull requests.
- Under ten seconds latency for supervisor → executor round trips.
- One hundred percent reproducibility: identical results from identical instructions.
- Safe parallelism: two or more agents run without input collisions.

## 6. Roadmap
- **Phase 0:** Split repositories (PyAutoGUI legacy vs CLI).
- **Phase 1:** Rust executor MVP (edit, diff, locks, process).
- **Phase 2:** Add gates for dedup, SSOT, and monolith limits.
- **Phase 3:** Observability and recovery (snapshots, retries).
- **Phase 4:** Supervisor expansion (review, QA, planner).
- **Phase 5:** Scale to multi-repository orchestration.

## 7. Foul Tree (Failure Tree)
- Malformed instruction or supervisor conflicts.
- File lock deadlocks or invalid patches.
- Compliance violations (SSOT, dedup, monolith, gate bypass).
- System issues: log corruption, missing snapshots, bus mis-routing, or permission problems.
- Human issues: incorrect role assignment, weak prompts, or missing commit context.

