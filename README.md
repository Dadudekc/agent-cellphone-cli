# agent-cellphone-cli

This README is the single source of truth (SSOT) for project information and diagrams.

## Foul Tree

The following Mermaid diagram is the SSOT for failure analysis.

```mermaid
flowchart TD
    SystemFailure[System failure]

    SystemFailure --> InstructionFailures[A) Instruction failures]
    InstructionFailures --> MalformedInstruction[A1) Malformed instruction]
    InstructionFailures --> SupervisorConflicts[A2) Supervisor conflicts]
    InstructionFailures --> InfiniteRetry[A3) Infinite retry loops]

    SystemFailure --> ExecutorFailures[B) Executor failures]
    ExecutorFailures --> LockingDeadlock[B1) Locking deadlock]
    ExecutorFailures --> PatchCorruption[B2) Patch/edit corruption]
    ExecutorFailures --> ProcessTimeout[B3) Process timeout (tests, lint, build)]
    ExecutorFailures --> RustCrash[B4) Rust executor crash (panic, IO error)]

    SystemFailure --> ComplianceFailures[C) Compliance failures]
    ComplianceFailures --> SSOTViolation[C1) SSOT violation]
    ComplianceFailures --> DuplicateHelpers[C2) Duplicate helpers]
    ComplianceFailures --> MonolithViolation[C3) Monolith violation (>300 LOC/file)]
    ComplianceFailures --> GateBypass[C4) Gate bypass (commit without check)]

    SystemFailure --> BusFailures[D) Bus / messaging failures]
    BusFailures --> MisroutedMessage[D1) Misrouted message]
    BusFailures --> DeliveryFailure[D2) Inbox/outbox delivery failure]
    BusFailures --> LogCorruption[D3) Log corruption / missing snapshots]

    SystemFailure --> HumanFailures[E) Human failures]
    HumanFailures --> RoleMisassignment[E1) Role misassignment (supervisor edits directly)]
    HumanFailures --> WeakPrompts[E2) Weak or ambiguous prompts]
    HumanFailures --> GovernanceDrift[E3) Governance drift (disabled gates, ignored reports)]
```

