from typing import Union

import numpy as np

from MoveGenerator import MoveGenerator
from Piece import (PieceColor, PieceType, PIECE_SYMBOLS, PIECE_FEN,
                   PIECE_VALUE_TO_TYPE)
from utils import is_white_piece, INITIAL_FEN


class Board:
    def __init__(self):
        self.squares = np.repeat(0, 64)  # Starting from top left
        self.white_captures = []
        self.black_captures = []

        self.black_en_passants = []
        self.white_en_passants = []

        self.is_white_move = True

        self.move_generator = MoveGenerator(self)

        self.winner: Union[None, PieceColor.Black, PieceColor.White] = None

        self.load_position(INITIAL_FEN)

    def get_available_moves(self):
        generate_functions = {
            PieceType.Empty: lambda *args: [],
            PieceType.Bishop: self.move_generator.generate_bishop_moves,
            PieceType.Knight: self.move_generator.generate_knight_moves,
            PieceType.Pawn: self.move_generator.generate_pawn_moves,
            PieceType.Queen: self.move_generator.generate_queen_moves,
            PieceType.Rook: self.move_generator.generate_rook_moves
        }

        black_moves = []
        white_moves = []
        pieces = []

        white_king_position = -1
        black_king_position = -1
        for position, piece in enumerate(self.squares):
            if PIECE_VALUE_TO_TYPE[piece] == PieceType.King:
                is_white_king = is_white_piece(piece)

                if is_white_king:
                    white_king_position = position
                else:
                    black_king_position = position

                pieces.append(None)
            else:
                pieces.append({
                    'moves':
                        generate_functions[PIECE_VALUE_TO_TYPE[piece]](
                            position),
                    'position': int(position),
                    'type': PIECE_FEN[piece] if (
                                piece != PieceType.Empty) else None
                })

        pieces[white_king_position] = {
            'moves': self.move_generator.generate_king_moves(
                black_moves, white_king_position
            ),
            'position': white_king_position,
            'type': PIECE_FEN[PieceColor.White | PieceType.King]
        }

        pieces[black_king_position] = {
            'moves': self.move_generator.generate_king_moves(
                white_moves, black_king_position
            ),
            'position': black_king_position,
            'type': PIECE_FEN[PieceColor.Black | PieceType.King]
        }

        # Check for invalid moves
        return pieces

    def place_piece(self, index: int, piece: int):
        self.__validate_board_index(index)

        if self.squares[index] != PieceType.Empty:
            current_piece_white = is_white_piece(index)

            if current_piece_white:
                self.black_captures.append(self.squares[index])
            else:
                self.white_captures.append(self.squares[index])

            if PIECE_VALUE_TO_TYPE[self.get_piece(index)] == PieceType.King:
                if current_piece_white:
                    self.winner = PieceColor.Black
                else:
                    self.winner = PieceColor.White

                # Add events on finish?
                return self.winner

        self.squares[index] = piece

        return None

    def get_winner(self):
        if self.winner is not None:
            return {
                PieceColor.White: "w",
                PieceColor.Black: "b",
            }[self.winner]

        return self.winner

    def move_piece(self, from_index: int, to_index: int):
        self.__validate_board_index(from_index)

        if self.squares[from_index] == PieceType.Empty:
            raise IndexError(f"No piece at position {from_index}")

        piece_value = self.squares[from_index]

        if self.place_piece(to_index, piece_value) is None:
            self.squares[from_index] = PieceType.Empty

    def get_piece(self, index: int):
        self.__validate_board_index(index)

        return self.squares[index]

    def is_valid_position(self, index: int):
        return 0 <= index < len(self.squares)

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
                    str_row += PIECE_SYMBOLS[PIECE_FEN[piece]]
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
