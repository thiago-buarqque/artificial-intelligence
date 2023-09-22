import os
import sys

from flask import request

from flask_app import app, to_json_response
from model.Board import Board
from model.dto.BoardDTO import BoardDTO

from ai_engine.ai_engine import Piece, get_ai_move

module_path = os.path.abspath(os.path.join("../../"))

if module_path not in sys.path:
    sys.path.append(module_path)

board = Board()


@app.route('/board/', methods=['GET'])
def get_board():
    return to_json_response(BoardDTO(board))


@app.route('/board/move/piece', methods=['POST'])
def move_piece():
    from_index = request.json["from"]
    to_index = request.json["to"]

    board.move_piece(from_index, to_index)

    # Ai move
    pieces = board.get_available_moves()

    # TODO Piece (PieceBoard) should come from Rust crate see #[pyo3(get, set)]
    if board.get_winner_fen() is None:
        source, destination = \
            get_ai_move([Piece(
                          "" if piece.fen is None else piece.fen,
                          piece.moves, piece.position, bool(piece.white)
                        ) for piece in pieces], False)

        board.move_piece(source, destination)

    return get_board()


@app.route('/board/load/fen', methods=['POST'])
def reset_board():
    board.load_position(request.json["fen"])

    return get_board()


if __name__ == '__main__':
    app.run(debug=True, use_debugger=True, use_reloader=False, host='0.0.0.0',
            port=8000)
