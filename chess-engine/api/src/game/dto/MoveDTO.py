from game.dto.DTO import DTO
from ai_engine.ai_engine import PieceMoveDTO


class MoveDTO(DTO):
    def __init__(self, from_position: int,
                 to_position: int,
                 promotion_type: str,
                 is_promotion: bool,
                 is_en_passant: bool):

        dict.__init__(self, fromPosition=from_position, toPosition=to_position,
                      promotionType=promotion_type, isPromotion=is_promotion,
                      isEnPassant=is_en_passant)

    @staticmethod
    def from_piece_moveDTO(piece_move: PieceMoveDTO):
        return MoveDTO(
            piece_move.from_position, piece_move.to_position,
            piece_move.promotion_type, piece_move.is_promotion, piece_move.is_en_passant)

    @staticmethod
    def from_dict_piece_moveDTO(piece_move: dict):
        return PieceMoveDTO(
            piece_move['fromPosition'], piece_move['toPosition'],
            piece_move['promotionType'], piece_move['isPromotion'],
            piece_move['isEnPassant'])
