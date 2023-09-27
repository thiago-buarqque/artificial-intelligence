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


# # variables with complex values
#
# __all__ = [
#     'Piece',
#     'get_ai_move',
# ]
#
# __loader__ = None  # (!) real value is '<_frozen_importlib_external.ExtensionFileLoader object at 0x7ff985bdba90>'
#
# __spec__ = None  # (!) real value is "ModuleSpec(name='ai_engine.ai_engine', loader=<_frozen_importlib_external.ExtensionFileLoader object at 0x7ff985bdba90>, origin='/home/evry/Desktop/repositories/artificial-intelligence-projects/chess-engine/ai-engine/venv/lib/python3.9/site-packages/ai_engine/ai_engine.cpython-39-x86_64-linux-gnu.so')"
