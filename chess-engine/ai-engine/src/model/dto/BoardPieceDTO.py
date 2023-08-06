import json

from dataclasses import dataclass
from typing import Union

from model.dto.DTO import DTO


@dataclass
class BoardPiece(DTO):
    fen: Union[str, None]
    moves: [int]
    position: int
    white: bool

    def to_dict(self):
        return {
            'fen': self.fen,
            'moves': self.moves,
            'position': self.position,
            'white': self.white
        }

