import os
import time
import sys

from flask import request

from flask_app import app, to_json_response

from ai_engine.ai_engine import BoardWrapper

from game.dto.BoardDTO import BoardDTO

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

    return to_json_response({
        "moves": board.get_move_generation_count(int(depth))
    })


@app.route('/board/move/piece', methods=['POST'])
def move_piece():
    from_index = request.json["from"]
    to_index = request.json["to"]

    board.move_piece(from_index, to_index)

    # Pure minimax on (depth: 4)
    # FEN: r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -
    # Evaluated 3619627 states
    # Elapsed time: 3127.3396015167236ms
    # 330, (0, 1)

    # Ai move
    # if board.get_winner_fen() == "-":
    #     start = time.time()
    #     move_value, destination = \
    #         board.get_ai_move()
    #
    #     end = time.time()
    #     print(f"Elapsed time: {(end - start) * 1000}")
    #     print(f"{move_value}, {destination}")
    #
    #     board.move_piece(destination[0], destination[1])

    return get_board()


@app.route('/board/load/fen', methods=['POST'])
def reset_board():
    board.load_position(request.json["fen"])

    return get_board()


if __name__ == '__main__':
    app.run(debug=True, use_debugger=True, use_reloader=False, host='0.0.0.0',
            port=8000)
