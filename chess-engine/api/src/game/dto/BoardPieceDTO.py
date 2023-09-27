from typing import Union

from ai_engine.ai_engine import Piece

from game.dto.DTO import DTO


class BoardPieceDTO(DTO):
    def __init__(self, fen: Union[str, None],
                 moves: list[int],
                 position: int,
                 white: bool):

        dict.__init__(self, fen=fen, moves=moves, position=position,
                      white=white)

    @staticmethod
    def from_piece(board_piece: Piece):
        fen = None if board_piece.fen == "" else board_piece.fen
        return BoardPieceDTO(
            fen, board_piece.moves, board_piece.position,
            board_piece.white)
