from typing import Union


class Piece:
    fen: Union[str, None]
    moves: list[int]
    position: int
    white: bool

    def __init__(self, fen: Union[str, None],
                 moves: list[int],
                 position: int,
                 white: bool) -> None:
        pass

class BoardWrapper:
    def get_move_generation_count(self) -> int:
        pass

    def get_ai_move(self) -> (int, int):
        pass

    def black_captures_to_fen(self):
        pass

    def white_captures_to_fen(self):
        pass

    def get_available_moves(self):
        pass

    def get_winner_fen(self):
        pass

    def is_white_move(self):
        pass

    def get_black_en_passant(self):
        pass

    def get_white_en_passant(self):
        pass

    def load_position(self):
        pass