from flask import Flask, jsonify, request
from flask_cors import CORS

from Board import Board

app = Flask(__name__)
cors = CORS(app, resources={r"/*": {"origins": "*"}})

board = Board()


@app.route('/board/', methods=['GET'])
def get_board():
    return jsonify({
        'blackCaptures': [],
        'pieces': board.get_available_moves(),
        'whiteCaptures': [],
        'winner': board.get_winner(),
    })


@app.route('/board/move/piece', methods=['POST'])
def move_piece():
    from_index = request.json["from"]
    to_index = request.json["to"]

    board.move_piece(from_index, to_index)

    return get_board()


if __name__ == '__main__':
    app.run(debug=True, use_debugger=True, use_reloader=False, host='0.0.0.0',
            port=8000)
