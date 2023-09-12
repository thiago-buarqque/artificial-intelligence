from typing import Union

from model.dto.DTO import DTO


class BoardPiece(DTO):
    def __init__(self, fen: Union[str, None],
                 moves: [int],
                 position: int,
                 white: bool):
        dict.__init__(self, fen=fen, moves=moves, position=position,
                      white=white)
