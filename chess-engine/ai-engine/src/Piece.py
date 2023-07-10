class PieceType:
    Empty = 0
    Bishop = 1
    King = 2
    Knight = 3
    Pawn = 4
    Queen = 5
    Rook = 6


class PieceColor:
    Black = 8
    White = 16


PIECE_SYMBOLS = {
    "B": "♗",
    "K": "♔",
    "N": "♘",
    "P": "♙",
    "Q": "♕",
    "R": "♖",

    "b": "♝",
    "k": "♚",
    "n": "♞",
    "p": "♟︎",
    "q": "♛",
    "r": "♜"
}

PIECE_VALUE_TO_FEN = {
    PieceColor.White | PieceType.Bishop: "B",
    PieceColor.White | PieceType.King: "K",
    PieceColor.White | PieceType.Knight: "N",
    PieceColor.White | PieceType.Pawn: "P",
    PieceColor.White | PieceType.Queen: "Q",
    PieceColor.White | PieceType.Rook: "R",

    PieceColor.Black | PieceType.Bishop: "b",
    PieceColor.Black | PieceType.King: "k",
    PieceColor.Black | PieceType.Knight: "n",
    PieceColor.Black | PieceType.Pawn: "p",
    PieceColor.Black | PieceType.Queen: "q",
    PieceColor.Black | PieceType.Rook: "r"
}