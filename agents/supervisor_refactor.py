"""Supervisor for refactor tasks.

>>> SupervisorRefactor().run()
'refactor'
"""

from agents.base_agent import BaseAgent


class SupervisorRefactor(BaseAgent):
    def run(self) -> str:
        return "refactor"
