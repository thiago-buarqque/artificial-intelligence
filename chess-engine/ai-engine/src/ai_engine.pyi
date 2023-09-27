from typing import Union


def get_ai_move(py_pieces: list[Piece], white_player: bool):
    pass


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