
from dataclasses import dataclass
from typing import Union


@dataclass
class BoardPiece:
    fen: Union[str, None]
    moves: list[int]
    position: int
    white: bool
