import enum

from src import Board
from src.Piece import PieceType
from src.utils import is_white_piece, is_same_color


class Offset(enum.Enum):
    BOTTOM_LINE = 8
    TOP_LINE = -8
    TOP_RIGHT_DIAGONAL = -7
    TOP_LEFT_DIAGONAL = -9
    BOTTOM_RIGHT_DIAGONAL = 9
    BOTTOM_LEFT_DIAGONAL = 7
    LEFT_SQUARE = -1
    RIGHT_SQUARE = 1


class MoveGenerator:
    def __init__(self, board: Board):
        self.board = board

    def generate_knight_moves(self, position: int):
        positions = [
            position - 10,
            position - 17,
            position - 15,
            position - 6,
            position + 6,
            position + 10,
            position + 15,
            position + 17
        ]

        moves = []
        for current_position in positions:
            if self.board.is_valid_position(current_position):
                piece = self.board.get_piece(current_position)

                if piece == PieceType.Empty or \
                        (not is_same_color(position, current_position)):
                    moves.append(current_position)

    def generate_king_moves(self, opponent_moves: [int], position: int):
        positions = [position - 1,
                     position + 1,
                     position - 9,
                     position - 8,
                     position - 7,
                     position + 7,
                     position + 8,
                     position + 9]

        moves = []
        for current_position in positions:
            if self.board.is_valid_position(current_position) and \
                    (current_position not in opponent_moves):
                piece = self.board.get_piece(current_position)

                if piece == PieceType.Empty or \
                        (not is_same_color(position, current_position)):
                    moves.append(current_position)

        return moves

    def generate_queen_moves(self, position: int):
        moves = []

        moves += self.generate_bishop_moves(position)

        moves += self.generate_rook_moves(position)

        return moves

    def generate_bishop_moves(self, position: int):
        piece = self.board.get_piece(position)

        moves = []

        self.__generate_sliding_moves(moves, piece, position,
                                      Offset.TOP_LEFT_DIAGONAL)

        self.__generate_sliding_moves(moves, piece, position,
                                      Offset.TOP_RIGHT_DIAGONAL)

        self.__generate_sliding_moves(moves, piece, position,
                                      Offset.BOTTOM_LEFT_DIAGONAL)

        self.__generate_sliding_moves(moves, piece, position,
                                      Offset.BOTTOM_RIGHT_DIAGONAL)

        return moves

    def generate_rook_moves(self, position: int):
        piece = self.board.get_piece(position)

        moves = []

        self.__generate_sliding_moves(moves, piece, position, Offset.TOP_LINE)

        self.__generate_sliding_moves(moves, piece, position,
                                      Offset.LEFT_SQUARE)

        self.__generate_sliding_moves(moves, piece, position,
                                      Offset.RIGHT_SQUARE)

        self.__generate_sliding_moves(moves, piece, position,
                                      Offset.BOTTOM_LINE)

        return moves

    def __generate_sliding_moves(self, moves: [int], piece: int,
                                 position: int,
                                 offset: Offset):
        for i in range(7):
            if (offset == Offset.BOTTOM_RIGHT_DIAGONAL or
                offset == Offset.TOP_RIGHT_DIAGONAL) and \
                    (position + 1) % 8 == 0:
                break

            if (offset == Offset.BOTTOM_LEFT_DIAGONAL or
                offset == Offset.TOP_LEFT_DIAGONAL) \
                    and position % 8 == 0:
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

            if offset != Offset.TOP_LINE and offset != Offset.BOTTOM_LINE:
                is_right_offset = offset == Offset.RIGHT_SQUARE or \
                                  offset == Offset.TOP_RIGHT_DIAGONAL or \
                                  offset == Offset.BOTTOM_RIGHT_DIAGONAL

                if (current_position + (1 if is_right_offset else 0)) % 8 == 0:
                    break

    def generate_pawn_moves(self, position: int):
        white_piece = is_white_piece(self.board.get_piece(position))

        offset = -8 if white_piece else 8

        next_line_position = position + offset

        moves: [int] = []
        self.__generate_pawn_moves(moves, next_line_position, offset, position,
                                   white_piece)

        self.__generate_pawn_capturing_moves(moves, next_line_position,
                                             position, white_piece)

        self.__generate_en_passant_moves(moves, offset, position, white_piece)

        return moves

    def __generate_pawn_moves(self, moves: [int], next_line_position: int,
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

    def __generate_pawn_capturing_moves(self, moves: [int],
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

    def __generate_en_passant_moves(self, moves: [int], offset: int,
                                    position: int, white_piece: bool):
        left_square = position - 1
        right_square = position + 1

        en_passants = self.board.black_en_passants if white_piece \
            else self.board.white_en_passants

        if left_square in en_passants:
            moves.append(left_square + offset)

        if right_square in en_passants:
            moves.append(right_square + offset)

    # Check if a move exposes the king after generating all moves
    @staticmethod
    def __is_pawn_first_move(white_piece: bool, piecePosition: int):
        if white_piece and (48 <= piecePosition <= 55):
            return True
        elif (not white_piece) and (8 <= piecePosition <= 15):
            return True

        return False
