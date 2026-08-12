"""Bytecode VM package for Kolos proto runtime."""

from .vm import VM
from .assembler import assemble, assemble_text

__all__ = ["VM", "assemble", "assemble_text"]
