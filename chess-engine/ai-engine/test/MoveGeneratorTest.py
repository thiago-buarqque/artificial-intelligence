import unittest

from src.Board import Board
from src.MoveGenerator import MoveGenerator
from src.Piece import PieceColor, PieceType


class MyTestCase(unittest.TestCase):

    def test_generate_pawn_moves_with_blocking_piece(self):
        board = Board()

        board.load_position("8/8/8/8/8/8/8/8 w - - 0 1")

        board.place_piece(36, PieceColor.Black | PieceType.Pawn)
        board.place_piece(52, PieceColor.White | PieceType.Pawn)

        move_generator = MoveGenerator(board)

        white_moves = move_generator.generate_pawn_moves(52)

        self.assertEqual(1, len(white_moves))
        self.assertIn(44, white_moves)

        board.move_piece(52, 44)

        white_moves = move_generator.generate_pawn_moves(44)

        self.assertEqual(0, len(white_moves))

        black_moves = move_generator.generate_pawn_moves(36)

        self.assertEqual(0, len(black_moves))

    def test_diagonal_capture_using_pawn(self):
        board = Board()

        board.load_position("8/8/8/8/8/8/8/8 w - - 0 1")

        board.place_piece(16, PieceColor.Black | PieceType.Pawn)
        board.place_piece(9, PieceColor.Black | PieceType.Pawn)
        board.place_piece(18, PieceColor.White | PieceType.Pawn)

        move_generator = MoveGenerator(board)

        moves = move_generator.generate_pawn_moves(9)

        self.assertEqual(3, len(moves))
        self.assertIn(17, moves)
        self.assertIn(25, moves)
        self.assertIn(18, moves)

    def test_generate_pawn_first_move(self):
        board = Board()

        move_generator = MoveGenerator(board)

        white_moves = move_generator.generate_pawn_moves(8)
        self.assertEqual(2, len(white_moves))
        self.assertIn(16, white_moves)
        self.assertIn(24, white_moves)

        black_moves = move_generator.generate_pawn_moves(48)
        self.assertEqual(2, len(black_moves))
        self.assertIn(40, black_moves)
        self.assertIn(32, black_moves)


if __name__ == '__main__':
    unittest.main()

