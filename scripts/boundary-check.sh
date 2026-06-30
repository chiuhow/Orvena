#!/usr/bin/env bash
# Two-tier pre-publish boundary check for Orvena.
#
# Orvena ships the methodology (the "how"); it must NEVER ship the lab's private
# research — hypotheses, findings, or validation evidence. This script is the
# pre-publish gate that enforces that red line, and is meant to run in CI before
# any release.
#
#   Tier 1 — HARD BLOCK: high-signal markers that can only be a real leak.
#            A single hit fails the build.
#   Tier 2 — REVIEW: bare `research/` or `hypothes…` — reported for a human to
#            eyeball (legitimate methodology vocabulary vs. lab evidence). Does
#            not fail the build on its own.
#
# Usage:   scripts/boundary-check.sh [path]   (defaults to repo root)
# Exit:    0 = Tier 1 clean (publishable), 1 = Tier 1 leak (do NOT publish).

set -uo pipefail

ROOT="${1:-.}"
SELF="$(basename "$0")"

# Never scanned: build output, VCS, deps, and THIS script (it contains the
# patterns and would match itself).
EXCLUDES=(
  --exclude-dir=target
  --exclude-dir=.git
  --exclude-dir=node_modules
  --exclude="$SELF"
  --exclude=Cargo.lock
)

# High-signal: a research validation path, a vlr result file, a hypothesis id
# (H001…), or a research-finding id (RF-00…). None are legitimate in Orvena.
TIER1='research/validation|vlr-[0-9]+-result|H00[0-9]|RF-0[0-9]'

# Suspicious-but-maybe-legitimate: a bare research/ path or hypothesis wording.
TIER2='research/|hypothes'

echo "== Orvena boundary check =="
echo "scanning: $ROOT"
echo

tier1_hits="$(grep -rnIE "${EXCLUDES[@]}" -e "$TIER1" "$ROOT" 2>/dev/null || true)"
tier2_hits="$(grep -rnIE "${EXCLUDES[@]}" -e "$TIER2" "$ROOT" 2>/dev/null || true)"

status=0

if [ -n "$tier1_hits" ]; then
  echo "✗ TIER 1 (hard block) — research-leak markers found:"
  printf '%s\n' "$tier1_hits" | sed 's/^/    /'
  echo
  status=1
else
  echo "✓ TIER 1: no high-signal research markers."
fi

if [ -n "$tier2_hits" ]; then
  echo "⚠ TIER 2 (review) — bare 'research/' or 'hypothes' references."
  echo "  Confirm each is methodology vocabulary, not lab evidence, before publishing:"
  printf '%s\n' "$tier2_hits" | sed 's/^/    /'
else
  echo "✓ TIER 2: no suspicious references."
fi

echo
if [ "$status" -eq 0 ]; then
  echo "RESULT: PASS (Tier 1 clean). Review any Tier 2 items above before publishing."
else
  echo "RESULT: FAIL — Tier 1 leak markers present. Do NOT publish."
fi
exit "$status"
