"""Gate runner loads YAML rules.

>>> run_gate({'action': 'fail'})
False
"""

from typing import Any


def run_gate(rule: dict[str, Any]) -> bool:
    """Return True if gate passes."""
    return rule.get("action") != "fail"
