import PieceType from "./PieceType";
import PieceColor from "./PieceColor";
import Position from "./Position";

export default class Piece {
    #color: PieceColor;
    #position: Position | null;
    #type: PieceType;

    constructor(color: PieceColor, position: Position | null = null, type: PieceType) {
        this.#color = color;
        this.#position = position;
        this.#type = type;
    }

    public getColor() {
        return this.#color;
    }

    public setPosition(position: Position) {
        this.#position = position;
    }

    public getPosition() {
        return this.#position;
    }

    public getType() {
        return this.#type;
    }
}
