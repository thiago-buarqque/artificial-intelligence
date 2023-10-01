from ai_engine import BoardWrapper

from game.dto.BoardPieceDTO import BoardPieceDTO
from game.dto.DTO import DTO


class BoardDTO(DTO):
    def __init__(self, board: BoardWrapper):
        pieces = board.get_available_moves()

        dict.__init__(self,
                      blackCaptures=board.black_captures_to_fen(),
                      pieces=[BoardPieceDTO.from_piece(piece) for piece in pieces],
                      whiteCaptures=board.white_captures_to_fen(),
                      winner=board.get_winner_fen(),
                      whiteMove=board.is_white_move(),
                      pawn_promotion=board.get_pawn_promotion_position()
                      )
