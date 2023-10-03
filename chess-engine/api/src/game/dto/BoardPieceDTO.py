import json

from typing import Union

from ai_engine.ai_engine import PieceDTO

from game.dto.BoardMoveDTO import BoardMoveDTO
from game.dto.DTO import DTO


class BoardPieceDTO(DTO):
    def __init__(self, fen: Union[str, None],
                 moves: list[int],
                 position: int,
                 white: bool):

        dict.__init__(self, fen=fen, moves=moves, position=position,
                      white=white)

    @staticmethod
    def from_str_piece(board_piece: str):
        board_piece = json.loads(board_piece)

        fen = None if board_piece['fen'] == "" else board_piece['fen']

        moves = [BoardMoveDTO.from_str_piece_move(move) for move in board_piece['moves']]

        return BoardPieceDTO(
            fen, moves, board_piece['position'],
            board_piece['white'])
