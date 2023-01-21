import PieceType from "./PieceType";
import PieceColor from "./PieceColor";
import Position from "./Position";

export default class Piece {
    #color: PieceColor;
    #type: PieceType;
    #position: Position | null;

    constructor(color: PieceColor, type: PieceType, position: Position | null = null) {
        this.#color = color;
        this.#type = type;
        this.#position = position;
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
