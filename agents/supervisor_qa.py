"""QA supervisor.

>>> SupervisorQA().run()
'qa'
"""

from agents.base_agent import BaseAgent


class SupervisorQA(BaseAgent):
    def run(self) -> str:
        return "qa"
