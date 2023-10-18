import os
import time
import sys

from flask import request

from flask_app import app, to_json_response

from ai_engine.ai_engine import BoardWrapper, PieceMoveDTO

from game.dto.BoardDTO import BoardDTO
from game.dto.MoveDTO import MoveDTO

module_path = os.path.abspath(os.path.join("../../"))

if module_path not in sys.path:
    sys.path.append(module_path)

board = BoardWrapper()


@app.route('/board/', methods=['GET'])
def get_board():
    return to_json_response(BoardDTO(board))


@app.route('/board/moves/count', methods=['POST'])
def get_move_count():
    depth = request.json["depth"]

    start = time.time()
    states = board.get_move_generation_count(int(depth))
    end = time.time()
    print(f"Elapsed time: {(end - start) * 1000}")

    return to_json_response({
        "moves": states
    })


@app.route('/board/move/piece', methods=['POST'])
def move_piece():
    board.move_piece(MoveDTO.from_dict_piece_moveDTO(request.json))

    # Evaluated 104201 states
    # Time elapsed is: 512.858322ms
    # Elapsed time: 0.512904167175293
    # 2, (12, 28)
    #
    # Evaluated 165322 states
    # Time elapsed is: 1.170370627s
    # Elapsed time: 1.170409917831421
    # 4, (14, 30)
    #
    # Evaluated 82993 states
    # Time elapsed is: 663.333424ms
    # Elapsed time: 0.6633729934692383
    # 5, (3, 21)
    #
    # Evaluated 2343 states
    # Time elapsed is: 56.840847ms
    # Elapsed time: 0.056849002838134766
    # 5, (21, 13)
    #
    # Evaluated 6460 states
    # Time elapsed is: 109.605544ms
    # Elapsed time: 0.10961484909057617
    # 5, (4, 13)

    # Ai move
    # if board.get_winner_fen() == "-":
    #     start = time.time()
    #     move_value, move = \
    #         board.get_ai_move(5)
    #
    #     # Evaluated 5480 states
    #     # Time elapsed is: 40.07236ms
    #
    #     end = time.time()
    #     print(f"Elapsed time: {(end - start)}")
    #     print(f"{move_value}, "
    #           f"({move.from_position}, {move.to_position})")
    #
    #     if move.from_position != -1 and move.to_position != -1:
    #         board.move_piece(move)

    return get_board()


@app.route('/board/load/fen', methods=['POST'])
def reset_board():
    board.load_position(request.json["fen"])

    return get_board()


if __name__ == '__main__':
    app.run(debug=True, use_debugger=True, use_reloader=False, host='0.0.0.0',
            port=8000)
