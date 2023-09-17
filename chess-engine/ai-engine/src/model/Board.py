from typing import Union

import numpy as np

from model.dto.BoardPieceDTO import BoardPiece
from model.MoveGenerator import MoveGenerator
from model.Piece import (
    PieceColor,
    PieceType,
    PIECE_SYMBOLS,
    piece_fen_from_value,
    get_piece_type, pieces_to_fen, piece_value_from_fen, is_piece_of_type)
from model.utils import is_white_piece, INITIAL_FEN


class Board:
    def __init__(self):
        self.squares: [int] = list(np.repeat(0, 64))  # Starting from top left
        self.white_captures: [int] = []
        self.black_captures: [int] = []

        self.black_en_passant: int = -1
        self.white_en_passant: int = -1

        self.black_king_moved = False
        self.white_king_moved = False

        self.is_white_move: bool = True
        self.half_moves: int = 0
        self.full_moves: int = 0

        self.is_white_in_check: bool = False
        self.is_black_in_check: bool = False

        self.move_generator: MoveGenerator = MoveGenerator(self)

        self.winner: Union[None, PieceColor.Black, PieceColor.White] = None

        self.load_position(INITIAL_FEN)

    def __reset(self):
        self.squares = np.repeat(0, 64)  # Starting from top left
        self.white_captures = []
        self.black_captures = []

        self.black_en_passant = -1
        self.white_en_passant = -1

        self.black_king_moved = False
        self.white_king_moved = False

        self.is_white_move = True
        self.half_moves = 0
        self.full_moves = 0

        self.move_generator = MoveGenerator(self)

        self.is_white_in_check = False
        self.is_black_in_check = False

        self.winner = None

    def white_captures_to_fen(self):
        return pieces_to_fen(self.white_captures)

    def black_captures_to_fen(self):
        return pieces_to_fen(self.black_captures)

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
        pieces: [BoardPiece] = []

        white_king_position = -1
        black_king_position = -1
        for position, piece in enumerate(self.squares):
            white_piece = is_white_piece(piece)
            piece_type = get_piece_type(piece)

            if piece_type == PieceType.King:

                if white_piece:
                    white_king_position = position
                else:
                    black_king_position = position

                pieces.append(None)
            else:
                # TODO Refactor to a to_json_object o something, this shouldn't
                # be in this class.
                moves = generate_functions[piece_type](position)

                piece_fen = piece_fen_from_value(piece) \
                    if not is_piece_of_type(piece, PieceType.Empty) else None

                pieces.append(BoardPiece(moves=moves, position=position,
                                         fen=piece_fen,
                                         white=white_piece))

                if white_piece:
                    white_moves += moves
                else:
                    black_moves += moves

        self.__get_king_available_moves(black_king_position, black_moves,
                                        pieces, white_king_position,
                                        white_moves)

        # Check for invalid moves
        return pieces

    def __get_king_available_moves(self, black_king_position, black_moves,
                                   board_pieces, white_king_position,
                                   white_moves):

        white_king_moves = self.move_generator.generate_king_moves(
            black_moves, white_king_position)

        black_king_moves = self.move_generator.generate_king_moves(
            white_moves, black_king_position)

        common_moves = list(set(white_king_moves) & set(black_king_moves))

        for move in common_moves:
            white_king_moves.remove(move)
            black_king_moves.remove(move)

        board_pieces[white_king_position] = \
            BoardPiece(moves=white_king_moves, position=white_king_position,
                       fen=piece_fen_from_value(PieceColor.White | PieceType.King),
                       white=True)

        board_pieces[black_king_position] = \
            BoardPiece(moves=black_king_moves, position=black_king_position,
                       fen=piece_fen_from_value(PieceColor.Black | PieceType.King),
                       white=False)

    def place_piece(self, index: int, piece: int):
        self.__validate_board_index(index)

        current_piece = self.squares[index]
        if not is_piece_of_type(current_piece, PieceType.Empty):

            white_piece = is_white_piece(current_piece)
            if white_piece:
                self.black_captures.append(current_piece)
            else:
                self.white_captures.append(current_piece)

            if is_piece_of_type(current_piece, PieceType.King):
                if white_piece:
                    self.winner = PieceColor.Black
                else:
                    self.winner = PieceColor.White

                # Add events on finish?

        self.squares[index] = piece

    def get_winner(self):
        if self.winner is not None:
            return {
                PieceColor.White: "w",
                PieceColor.Black: "b",
            }[self.winner]

        return self.winner

    def move_piece(self, from_index: int, to_index: int,
                   rook_castling: bool = False):

        self.__validate_board_index(from_index)

        if is_piece_of_type(self.squares[from_index], PieceType.Empty):
            raise IndexError(f"No piece at position {from_index}")

        moving_piece = self.squares[from_index]

        if self.__is_en_passant_capture(moving_piece, to_index):
            self.__capture_en_passant(moving_piece)
        elif is_piece_of_type(moving_piece, PieceType.King):
            self.__handle_king_move(from_index, moving_piece, to_index)

        self.place_piece(to_index, moving_piece)

        self.squares[from_index] = PieceType.Empty

        self.__register_en_passant(from_index, moving_piece, to_index)

        if not self.is_white_move:
            self.full_moves += 1

        if not rook_castling:
            self.is_white_move = not self.is_white_move

        # Verify king check

    def __handle_king_move(self, from_index, moving_piece, to_index):
        white_piece = is_white_piece(moving_piece)

        CASTLE_MOVE = abs(from_index - to_index) == 2
        if CASTLE_MOVE and ((white_piece and not self.white_king_moved) or
                            (not white_piece and not self.black_king_moved)):
            self.__castle(from_index, to_index, white_piece)

        if white_piece:
            self.white_king_moved = True
        else:
            self.black_king_moved = True

    def __castle(self, from_index: int, to_index: int, white_piece: bool):
        QUEEN_SIDE_ROOK_POSITION = 56 if white_piece else 0
        KING_SIDE_ROOK_POSITION = 63 if white_piece else 7

        ROOK_POSITION = QUEEN_SIDE_ROOK_POSITION \
            if from_index > to_index else KING_SIDE_ROOK_POSITION

        NEW_ROOK_POSITION = from_index - 1 \
            if from_index > to_index else from_index + 1

        if white_piece:
            self.white_able_to_queen_castle = False
            self.white_able_to_king_castle = False
        else:
            self.black_able_to_queen_castle = False
            self.black_able_to_king_castle = False

        self.move_piece(ROOK_POSITION, NEW_ROOK_POSITION, rook_castling=True)

    def __capture_en_passant(self, moving_piece: int):
        white_piece = is_white_piece(moving_piece)

        if white_piece:
            self.white_captures.append(self.squares[self.black_en_passant + 8])
            self.squares[self.black_en_passant] = PieceType.Empty
            self.black_en_passant = -1
        else:
            self.black_captures.append(self.squares[self.white_en_passant - 8])
            self.squares[self.white_en_passant] = PieceType.Empty
            self.white_en_passant = -1

    def __is_en_passant_capture(self, piece: int, to_index: int):
        if is_piece_of_type(piece, PieceType.Pawn):
            white_piece = is_white_piece(piece)

            en_passant = self.black_en_passant \
                if white_piece else self.white_en_passant

            return en_passant != -1 and to_index == en_passant

        return False

    def __register_en_passant(self, from_index, piece_value, to_index):
        if is_piece_of_type(piece_value, PieceType.Pawn):
            white_piece = is_white_piece(piece_value)

            if white_piece:
                self.white_en_passant = -1

                # magic numbers...
                if 47 < from_index < 56 and 31 < to_index < 40:
                    self.white_en_passant = to_index + 8
            else:
                self.black_en_passant = -1

                # magic numbers...
                if 7 < from_index < 16 and 23 < to_index < 32:
                    self.black_en_passant = to_index - 8

    def get_piece(self, index: int):
        self.__validate_board_index(index)

        return int(self.squares[index])

    def is_valid_position(self, index: int):
        return 0 <= index < len(self.squares)

    def load_position(self, fen_position: str):
        self.__reset()

        fields = fen_position.split(" ")

        self.__generate_pieces_from_fen(fields[0])
        self.__load_active_color(fields[1])
        self.__load_castling(fields[2])
        self.__load_en_passant(fields[3])
        self.__load_half_move_clock(fields[4])
        self.__load_full_move_number(fields[5])

    def __load_half_move_clock(self, half_move: str):
        if half_move.isdigit():
            self.half_moves = int(half_move)
        else:
            self.half_moves = 0

    def __load_full_move_number(self, moves: str):
        if moves.isdigit():
            self.full_moves = int(moves)
        else:
            self.full_moves = 0

    def __load_en_passant(self, en_passant: str):
        if en_passant == '-':
            self.white_en_passant = -1
            self.black_en_passant = -1
        else:
            column = en_passant[0]

            row = int(en_passant[1])

            is_white = False
            if row == 3:
                is_white = True
                row = 5
            else:
                row = 4

            position = ((ord(column) - 97) + (row * 8)) - 8

            if is_white:
                self.white_en_passant = position
                self.black_en_passant = -1
            else:
                self.black_en_passant = position
                self.white_en_passant = -1

    def __load_castling(self, castling: str):
        if castling == '-':
            self.black_able_to_queen_castle = False
            self.black_able_to_king_castle = False
            self.white_able_to_queen_castle = False
            self.white_able_to_king_castle = False
            self.black_king_moved = True
            self.white_king_moved = True
        else:
            if 'K' in castling:
                self.white_able_to_king_castle = True
            if 'Q' in castling:
                self.white_able_to_queen_castle = True
            if 'k' in castling:
                self.black_able_to_king_castle = True
            if 'q' in castling:
                self.black_able_to_queen_castle = True

    def print_board(self):
        index = 0

        for i in range(8):
            str_row = "|"
            for j in range(8):
                piece = self.get_piece(index)
                if not is_piece_of_type(piece, PieceType.Empty):
                    str_row += PIECE_SYMBOLS[piece_fen_from_value(piece)]
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

    def __generate_pieces_from_fen(self, board_rows: str):
        rows = board_rows.split("/")

        index = 0
        for row in rows:
            for char in row:
                if char.isnumeric():
                    index += int(char)
                else:
                    self.squares[index] = piece_value_from_fen(char)
                    index += 1

    def __validate_board_index(self, index):
        if not 0 <= index < len(self.squares):
            raise IndexError(f"Invalid board index {index}")
