import enum

import model.Board as Board
from model.Piece import PieceType
from model.utils import is_white_piece, is_same_color


class SquareOffset(enum.Enum):
    LINE_BELOW = 8
    LINE_ABOVE = -8
    TOP_RIGHT = -7
    TOP_LEFT = -9
    BOTTOM_RIGHT = 9
    BOTTOM_LEFT = 7
    LEFT = -1
    RIGHT = 1


class MoveGenerator:
    def __init__(self, board: 'Board'):
        self.board = board

    def __validate_knight_position(self, lines_apart: int, new_position: int,
                                   position: int):

        if self.__get_positions_line_distance(
                position, new_position) == lines_apart:

            return new_position

        return -1

    def generate_knight_moves(self, position: int):
        positions = [
            self.__validate_knight_position(2, position - 17, position),
            self.__validate_knight_position(2, position - 15, position),
            self.__validate_knight_position(1, position - 10, position),
            self.__validate_knight_position(1, position - 6, position),
            self.__validate_knight_position(1, position + 6, position),
            self.__validate_knight_position(1, position + 10, position),
            self.__validate_knight_position(2, position + 15, position),
            self.__validate_knight_position(2, position + 17, position)
        ]

        moves = []
        knight_piece = self.board.get_piece(position)
        for current_position in positions:
            if self.board.is_valid_position(current_position):
                current_piece = self.board.get_piece(current_position)

                if current_piece == PieceType.Empty or \
                        (not is_same_color(knight_piece, current_piece)):
                    moves.append(current_position)

        return moves

    def generate_king_moves(self, opponent_moves: list[int], king_position: int):
        positions = [king_position - 1,
                     king_position + 1,
                     king_position - 9,
                     king_position - 8,
                     king_position - 7,
                     king_position + 7,
                     king_position + 8,
                     king_position + 9]

        moves = []
        king = self.board.get_piece(king_position)
        for position in positions:
            if self.board.is_valid_position(position) and \
                    (position not in opponent_moves):
                piece = self.board.get_piece(position)

                if piece == PieceType.Empty or \
                        (not is_same_color(king, piece)):
                    moves.append(position)

        if king_position not in opponent_moves:
            self.__generate_castle_moves(king, moves,
                                         opponent_moves, king_position)

        return moves

    def __generate_castle_moves(self, king_piece: int, moves: list[int],
                                opponent_moves: list[int], position: int):

        def is_path_clear(start, end, step):
            for i in range(start, end, step):
                if self.board.get_piece(i) != PieceType.Empty:
                    return False

            return True

        def position_is_not_attacked(n: int):
            return n not in opponent_moves

        def is_able_to_castle_queen_side(white_king):
            return (white_king and self.board.white_able_to_queen_castle) or \
                (not white_king and self.board.black_able_to_queen_castle)

        def is_able_to_castle_king_side(white_king):
            return (white_king and self.board.white_able_to_king_castle) or \
                    (not white_king and self.board.black_able_to_king_castle)

        is_white_king = is_white_piece(king_piece)

        if (is_white_king and not self.board.white_king_moved) or \
                (not is_white_king and not self.board.black_king_moved):

            queen_side_rook_position = 56 if is_white_king else 0
            king_side_rook_position = 63 if is_white_king else 7

            able_to_castle_queen_side = \
                is_able_to_castle_queen_side(is_white_king)

            able_to_castle_king_side = \
                is_able_to_castle_king_side(is_white_king)

            if able_to_castle_queen_side and \
                    is_path_clear(position - 1, queen_side_rook_position, -1):

                new_position = position - 2

                if position_is_not_attacked(new_position) and \
                        position_is_not_attacked(position - 1):

                    moves.append(new_position)

            if able_to_castle_king_side and \
                    is_path_clear(position + 1, king_side_rook_position, 1):

                new_position = position + 2

                if position_is_not_attacked(new_position) and \
                        position_is_not_attacked(position + 1):

                    moves.append(new_position)

    def generate_queen_moves(self, position: int):
        moves = []

        moves += self.generate_bishop_moves(position)

        moves += self.generate_rook_moves(position)

        return moves

    def generate_bishop_moves(self, position: int):
        piece = self.board.get_piece(position)

        moves = []

        self.__generate_sliding_moves(moves, piece, position,
                                      SquareOffset.TOP_LEFT)

        self.__generate_sliding_moves(moves, piece, position,
                                      SquareOffset.TOP_RIGHT)

        self.__generate_sliding_moves(moves, piece, position,
                                      SquareOffset.BOTTOM_LEFT)

        self.__generate_sliding_moves(moves, piece, position,
                                      SquareOffset.BOTTOM_RIGHT)

        return moves

    def generate_rook_moves(self, position: int):
        piece = self.board.get_piece(position)

        moves = []

        self.__generate_sliding_moves(moves, piece, position, SquareOffset.LINE_ABOVE)

        self.__generate_sliding_moves(moves, piece, position,
                                      SquareOffset.LEFT)

        self.__generate_sliding_moves(moves, piece, position,
                                      SquareOffset.RIGHT)

        self.__generate_sliding_moves(moves, piece, position,
                                      SquareOffset.LINE_BELOW)

        return moves

    def __generate_sliding_moves(self, moves: list[int], piece: int,
                                 position: int,
                                 offset: SquareOffset):
        for i in range(7):
            if (offset == SquareOffset.BOTTOM_RIGHT or
                offset == SquareOffset.TOP_RIGHT) and \
                    (position + 1) % 8 == 0:
                break

            if (offset == SquareOffset.BOTTOM_LEFT or
                offset == SquareOffset.TOP_LEFT) \
                    and position % 8 == 0:
                break

            if (offset == SquareOffset.LEFT and position % 8 == 0) or \
                    (offset == SquareOffset.RIGHT and (position + 1) % 8 == 0):
                break

            current_position = position + ((i + 1) * offset.value)

            if not self.board.is_valid_position(current_position):
                break

            current_piece = self.board.get_piece(current_position)

            if current_piece == PieceType.Empty:
                moves.append(current_position)
            elif not is_same_color(piece, current_piece):
                moves.append(current_position)
                break
            else:
                break

            if (offset != SquareOffset.LINE_ABOVE and
                    offset != SquareOffset.LINE_BELOW):

                right_offset = offset == SquareOffset.RIGHT or \
                                  offset == SquareOffset.TOP_RIGHT or \
                                  offset == SquareOffset.BOTTOM_RIGHT

                if (current_position + (1 if right_offset else 0)) % 8 == 0:
                    break

    def generate_pawn_moves(self, position: int):
        white_piece = is_white_piece(self.board.get_piece(position))

        offset = -8 if white_piece else 8

        next_line_position = position + offset

        moves: list[int] = []

        if not self.board.is_valid_position(next_line_position):
            return moves

        self.__generate_pawn_moves(moves, next_line_position, offset, position,
                                   white_piece)

        self.__generate_pawn_capturing_moves(moves, next_line_position,
                                             position, white_piece)

        self.__generate_en_passant_moves(moves, offset, position, white_piece)

        return moves

    def __generate_pawn_moves(self, moves: list[int], next_line_position: int,
                              offset: int, position: int, white_piece: bool):
        if self.board.is_valid_position(next_line_position):
            existing_piece = self.board.get_piece(next_line_position)

            if existing_piece == PieceType.Empty:
                moves.append(next_line_position)

        two_lines_position = position + (offset * 2)

        if self.__is_pawn_first_move(white_piece, position):
            existing_piece = self.board.get_piece(two_lines_position)

            if existing_piece == PieceType.Empty:
                moves.append(two_lines_position)

    def __generate_pawn_capturing_moves(self, moves: list[int],
                                        next_line_position: int,
                                        position: int, white_piece: bool):
        diagonal_left = next_line_position - 1

        if position % 8 == 0:
            diagonal_left = -1

        if diagonal_left != -1:
            existing_piece = self.board.get_piece(diagonal_left)

            if self.board.is_valid_position(diagonal_left) and \
                    (existing_piece != PieceType.Empty) and \
                    is_white_piece(existing_piece) != white_piece:
                moves.append(diagonal_left)

        diagonal_right = next_line_position + 1

        if (position + 1) % 8 == 0:
            diagonal_right = -1

        if diagonal_right != -1:
            existing_piece = self.board.get_piece(diagonal_right)

            if self.board.is_valid_position(diagonal_right) and \
                    (existing_piece != PieceType.Empty) and \
                    is_white_piece(existing_piece) != white_piece:
                moves.append(diagonal_right)

    def __generate_en_passant_moves(self, moves: list[int], offset: int,
                                    position: int, white_piece: bool):

        left_square = position - 1 if position % 8 != 0 else -1
        right_square = position + 1 if (position + 1) % 8 != 0 else -1

        en_passant = self.board.black_en_passant if white_piece \
            else self.board.white_en_passant

        if en_passant == -1:
            return

        if left_square == (en_passant + 8 if white_piece else en_passant - 8):
            moves.append(left_square + offset)
        elif right_square == (en_passant + 8 if white_piece else en_passant - 8):
            moves.append(right_square + offset)

    # Check if a move exposes the king after generating all moves

    @staticmethod
    def __get_positions_line_distance(position1: int, position2: int):
        line_start1 = position1 - (position1 % 8)
        line_start2 = position2 - (position2 % 8)

        if line_start1 > line_start2:
            return int((line_start1 - line_start2) / 8)

        return int((line_start2 - line_start1) / 8)

    @staticmethod
    def __is_pawn_first_move(white_piece: bool, piecePosition: int):
        if white_piece and (48 <= piecePosition <= 55):
            return True
        elif (not white_piece) and (8 <= piecePosition <= 15):
            return True

        return False
