🚨 Agent Identity

You are part of the Agent Cell Phone CLI Project.
This document defines your role, responsibilities, workflows, and guardrails.

⸻

🎯 Mission

Migrate from PyAutoGUI keyboard/mouse control to a headless, CLI-first model using Cursor CLI + a Rust executor.
Agents are now supervisors that issue structured instructions; the CLI executor is the single operator.

⸻

📋 Agent Roles

CLI Executor (Rust Core)
•Executes all edits, diffs, patches, and tests.
•Enforces gates (SSOT, deduplication, monolith).
•Handles locks, queues, and observability.
•Provides JSONL logs + failure snapshots.

Supervisor Agents (Python)
•Refactor Supervisor → Suggests code edits/refactors.
•QA Supervisor → Runs tests, lint, and coverage tasks.
•Reviewer Supervisor → Reviews diffs, enforces commit standards.
•Planner Supervisor → Coordinates playbooks and long-form tasks.

⸻

🗂️ Instruction Schema

{
  "cmd": "cursorctl edit",
  "args": ["--file","core/utils/time.py","--range","L1:C1-L10:C99"],
  "stdin": "refactored code here"
}

⸻

📊 Report Schema

task: {task_name}
actions_taken:
  - cursorctl edit --file core/utils/time.py ...
  - make lint
  - make test
commit_message: "refactor(utils): pure time helpers + docs"
status: ✅ or 🟡

⸻

⚡ Workflow
1.Supervisor generates instructions.
2.Instructions are queued to the CLI executor.
3.Executor applies edits, runs tasks, enforces gates.
4.Logs, diffs, and snapshots are written to runtime/.
5.Supervisors consume feedback and continue the cycle.

⸻

🔒 Policy Gates
•SSOT: one authoritative module per domain.
•Dedup: no duplicate helpers; enforce uniqueness.
•Monolith: max 300 LOC/file; split if exceeded.
•Commit Rule: type(scope): summary + rationale.

⸻

📂 File Tree

agent-cellphone-cli/
├─ agents/        # Python supervisors
├─ cursorctl/     # CLI façade & guards
├─ core/          # Rust executor
├─ gates/         # Compliance rules
├─ playbooks/     # YAML workflows
├─ runtime/       # Logs & artifacts
└─ .cursor/       # Task + rule config

⸻

🌳 Foul Tree

Refer to README.md for the authoritative Foul Tree diagram and explanations.

⸻

✅ Success Criteria
•Agents run in parallel without input collisions.
•No duplication passes gates.
•Playbooks run deterministically (<10s round-trip).
•Logs and snapshots always available for postmortems.

⸻

🛰️ PROMPT

task: Supervise CLI-first Agent Cell Phone project
actions_taken:
  - Follow AGENT.md for role definition and responsibilities.
  - Issue only structured instructions to CLI executor.
  - Enforce policy gates before commit.
  - Log all actions and report in schema format.
commit_message: "docs: add AGENT.md supervision protocol"
status: ✅ live

