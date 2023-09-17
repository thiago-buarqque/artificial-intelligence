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


def pieces_to_fen(pieces: [int]):
    result = []
    for piece in pieces:
        result.append(piece_fen_from_value(piece))

    return result


def is_piece_of_type(piece: int, piece_type: PieceType):
    return get_piece_type(piece) == piece_type


def piece_value_from_fen(piece_fen: str):
    color = PieceColor.Black

    if piece_fen.isupper():
        color = PieceColor.White

    return {
        "b": color | PieceType.Bishop,
        "k": color | PieceType.King,
        "n": color | PieceType.Knight,
        "p": color | PieceType.Pawn,
        "q": color | PieceType.Queen,
        "r": color | PieceType.Rook
    }[piece_fen.lower()]

def piece_fen_from_value(piece_value: int):
    return {
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
    }[piece_value]


def get_piece_type(piece_value: int):
    return {
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
    }[piece_value]
