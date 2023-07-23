from src.Piece import PieceColor, PieceType


def is_white_piece(pieceValue):
    return (PieceColor.White | PieceType.Bishop) <= \
        pieceValue <= (PieceColor.White | PieceType.Rook)


def is_black_piece(pieceValue):
    return (PieceColor.Black | PieceType.Bishop) <= \
        pieceValue <= (PieceColor.Black | PieceType.Rook)


def is_same_color(piece1: int, piece2: int):
    return is_white_piece(piece1) is is_white_piece(piece2)


EMPTY_BOARD_FEN = "8/8/8/8/8/8/8/8 w - - 0 1"

INITIAL_FEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
