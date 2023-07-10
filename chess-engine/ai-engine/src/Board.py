import numpy as np

from src.Piece import (PieceColor, PieceType, PIECE_SYMBOLS, PIECE_VALUE_TO_FEN)

FEN_STARTING_POSITION = \
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"


class Board:
    def __init__(self):
        self.squares = np.repeat(0, 64)  # Starting from top left
        self.white_captures = []
        self.black_captures = []

        self.is_white_move = True

        self.load_position(FEN_STARTING_POSITION)

    def place_piece(self, index: int, piece: int):
        self.__validate_board_index(index)

        if self.squares[index] != PieceType.Empty:
            if self.__is_white_piece(index):
                self.black_captures.append(self.squares[index])
            else:
                self.white_captures.append(self.squares[index])

        self.squares[index] = piece

    def move_piece(self, from_index: int, to_index: int):
        self.__validate_board_index(from_index)

        if self.squares[from_index] == PieceType.Empty:
            raise IndexError(f"No piece at position {from_index}")

        piece_value = self.squares[from_index]

        self.squares[from_index] = PieceType.Empty

        self.place_piece(to_index, piece_value)

    def get_piece(self, index: int):
        self.__validate_board_index(index)

        return self.squares[index]

    def load_position(self, fen_position: str):
        self.squares = np.repeat(0, 64)

        fields = fen_position.split(" ")

        self.__generate_pieces(fields[0])
        self.__load_active_color(fields[1])
        # TODO Load castling, En passant, half move clock and full move number

    def print_board(self):
        index = 0

        for i in range(8):
            str_row = "|"
            for j in range(8):
                piece = self.get_piece(index)
                if piece != PieceType.Empty:
                    str_row += PIECE_SYMBOLS[PIECE_VALUE_TO_FEN[piece]]
                else:
                    str_row += " "

                str_row += "|"
                index += 1

            print(str_row)

    def __load_active_color(self, active_color: str):
        if active_color == 'w':
            self.is_white_move = True
        elif active_color == 'b':
            self.is_white_move = False
        else:
            raise ValueError(f"Invalid active color '{active_color}'.")

    def __generate_pieces(self, piece_placement: str):
        rows = piece_placement.split("/")

        index = 0
        for row in rows:
            for char in row:
                if char.isnumeric():
                    index += int(char)
                else:
                    self.squares[index] = self.__generate_piece(char)
                    index += 1

    @staticmethod
    def __generate_piece(char: str):
        color = PieceColor.Black

        if char.isupper():
            color = PieceColor.White

        return {
            "b": color | PieceType.Bishop,
            "k": color | PieceType.King,
            "n": color | PieceType.Knight,
            "p": color | PieceType.Pawn,
            "q": color | PieceType.Queen,
            "r": color | PieceType.Rook
        }[char.lower()]

    def __validate_board_index(self, index):
        if not 0 <= index < len(self.squares):
            raise IndexError(f"Invalid board index {index}")

    @staticmethod
    def __is_white_piece(pieceValue):
        return (PieceColor.White | PieceType.Bishop) <= \
            pieceValue <= (PieceColor.White | PieceType.Rook)

    @staticmethod
    def __is_black_piece(pieceValue):
        return (PieceColor.Black | PieceType.Bishop) <= \
            pieceValue <= (PieceColor.Black | PieceType.Rook)
