"""Reviewer supervisor.

>>> SupervisorReviewer().run()
'review'
"""

from agents.base_agent import BaseAgent


class SupervisorReviewer(BaseAgent):
    def run(self) -> str:
        return "review"
