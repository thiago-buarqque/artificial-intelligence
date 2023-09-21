from game.model.Board import Board
from game.model.dto.DTO import DTO


class BoardDTO(DTO):
    def __init__(self, board: Board):
        dict.__init__(self,
                      blackCaptures=board.black_captures_to_fen(),
                      pieces=board.get_available_moves(),
                      whiteCaptures=board.white_captures_to_fen(),
                      winner=board.get_winner_fen(),
                      whiteMove=board.is_white_move,
                      whiteEnPassant=board.white_en_passant,
                      blackEnPassant=board.black_en_passant
                      )
