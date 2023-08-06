from dataclasses import dataclass

from model.Board import Board
from model.dto.DTO import DTO


@dataclass
class BoardDTO(DTO):
    board: Board

    def to_dict(self) -> dict:
        return {
            'blackCaptures': self.board.get_black_captures(),
            'pieces': self.board.get_available_moves(),
            'whiteCaptures': self.board.get_white_captures(),
            'winner': self.board.get_winner(),
            'whiteMove': self.board.is_white_move,
            'whiteEnPassant': self.board.white_en_passant,
            'blackEnPassant': self.board.black_en_passant
        }