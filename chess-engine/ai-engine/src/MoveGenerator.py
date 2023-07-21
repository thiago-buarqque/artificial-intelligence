from src.Board import Board
from src.Piece import PieceType
from src.utils import is_white_piece


class MoveGenerator:
    def __init__(self, board: Board):
        self.board = board

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

        diagonal_right = next_line_position + 1

        if (position + 1) % 8 == 0:
            diagonal_right = -1

        existing_piece = self.board.get_piece(diagonal_left)

        if self.board.is_valid_position(diagonal_left) and \
                (existing_piece != PieceType.Empty) and \
                is_white_piece(existing_piece) != white_piece:
            moves.append(diagonal_left)

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
