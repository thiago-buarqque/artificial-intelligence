from src.Board import Board

if __name__ == '__main__':
    board = Board()

    board.print_board()

    board.move_piece(1, 18)

    board.print_board()
