import json

from game.dto.DTO import DTO


class BoardMoveDTO(DTO):
    def __init__(self, from_position: int,
                 to_position: int,
                 promotion_type: str,
                 is_en_passant: bool):

        dict.__init__(self, fen=from_position, moves=to_position, position=promotion_type,
                      white=is_en_passant)

    @staticmethod
    def from_str_piece_move(piece_move: str):
        piece_move = json.loads(piece_move)

        return BoardMoveDTO(
            piece_move['from'], piece_move['to'],
            piece_move['promotion_type'], piece_move['is_en_passant'])
