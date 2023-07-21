from src.Piece import PieceColor, PieceType


def is_white_piece(pieceValue):
    return (PieceColor.White | PieceType.Bishop) <= \
        pieceValue <= (PieceColor.White | PieceType.Rook)


def is_black_piece(pieceValue):
    return (PieceColor.Black | PieceType.Bishop) <= \
        pieceValue <= (PieceColor.Black | PieceType.Rook)