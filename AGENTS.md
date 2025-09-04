# Repository Guidelines

## Scope
These guidelines apply to the entire repository.

## Single Source of Truth (SSOT)
- Respect the SSOT principle in all files.
- `README.md` is the SSOT for project details and diagrams.
- When modifying information covered by a SSOT file, update that file directly.
- Avoid duplicating data; link to the SSOT instead.

## Sub-300 LOC Rule
- No file may exceed 300 lines.
- Check line counts with `wc -l` before committing changes.
- Refactor or split files approaching the 300 line limit.
- This file must remain under 300 lines.

## Coding Policies
- Keep implementations simple and self-explanatory.
- Use clear naming and provide inline comments when helpful.
- Include tests for new behavior and run the project's test suite before committing.
- Use descriptive commit messages referencing the changes.
- Ensure artifacts that are the SSOT remain consistent with your changes.
