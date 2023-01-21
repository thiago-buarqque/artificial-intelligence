import Piece from "./Piece";

export default class Game { // Keep calling it `Board`?
    #board: (Piece | null)[][]

    constructor() {
        this.#board = this.#fillEmptyBoard();        
    }
    
    #fillEmptyBoard() {
        const BOARD_SIZE = 8;

        const board = [];

        for(let i = 0; i < BOARD_SIZE; i++) {
            board.push(Array(BOARD_SIZE).fill(null))
        }

        return board;
    }

    public getBoard() {
        return this.#board;
    }

    public placePiece(piece: Piece) {
        const position = piece.getPosition();

        if(position !== null) {
            this.#board[position.getX()][position.getY()] = piece;
        }
    }

    public removePiece(piece: Piece) {
        const position = piece.getPosition()
        
        if(position !== null) {
            this.#board[position.getX()][position.getY()] = null
        }
    }
}