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

PIECE_FEN = {
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


PIECE_VALUE_TO_TYPE = {
    PieceType.Empty: PieceType.Empty,

    PieceColor.White | PieceType.Bishop: PieceType.Bishop,
    PieceColor.White | PieceType.King: PieceType.King,
    PieceColor.White | PieceType.Knight: PieceType.Knight,
    PieceColor.White | PieceType.Pawn: PieceType.Pawn,
    PieceColor.White | PieceType.Queen: PieceType.Queen,
    PieceColor.White | PieceType.Rook: PieceType.Rook,

    PieceColor.Black | PieceType.Bishop: PieceType.Bishop,
    PieceColor.Black | PieceType.King: PieceType.King,
    PieceColor.Black | PieceType.Knight: PieceType.Knight,
    PieceColor.Black | PieceType.Pawn: PieceType.Pawn,
    PieceColor.Black | PieceType.Queen: PieceType.Queen,
    PieceColor.Black | PieceType.Rook: PieceType.Rook
}