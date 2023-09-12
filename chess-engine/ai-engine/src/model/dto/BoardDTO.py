import json
from dataclasses import dataclass

from model.Board import Board
from model.dto.DTO import DTO


class BoardDTO(DTO):
    def __init__(self, board: Board):
        dict.__init__(self,
                      blackCaptures=board.get_black_captures(),
                      pieces=board.get_available_moves(),
                      whiteCaptures=board.get_white_captures(),
                      winner=board.get_winner(),
                      whiteMove=board.is_white_move,
                      whiteEnPassant=board.white_en_passant,
                      blackEnPassant=board.black_en_passant
                      )
