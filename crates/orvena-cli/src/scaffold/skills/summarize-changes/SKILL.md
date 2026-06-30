---
name: summarize-changes
description: Summarize the changes made in the current task into a short changelog entry.
triggers: ["summarize changes", "/summarize"]
role: developer
---

# Summarize Changes

When invoked, produce a concise, user-facing summary of what changed in this task.

1. Group changes as **Added** / **Changed** / **Fixed**.
2. One line per change, written for a CHANGELOG reader — not a commit log.
3. Keep it short. If it does not fit in a few lines, the change was too big.
