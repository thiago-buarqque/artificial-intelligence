from typing import Union

from model.BoardPiece import BoardPiece
from model.dto.DTO import DTO


class BoardPieceDTO(DTO):
    def __init__(self, fen: Union[str, None],
                 moves: list[int],
                 position: int,
                 white: bool):

        dict.__init__(self, fen=fen, moves=moves, position=position,
                      white=white)

    @staticmethod
    def from_board_piece(board_piece: BoardPiece):
        return BoardPieceDTO(
            board_piece.fen, board_piece.moves, board_piece.position,
            board_piece.white)
