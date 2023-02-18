export default class Position {
    #column: String;
    #line: number;
    #x: number;
    #y: number;

    constructor(x: number, y: number, line: number, column: String) {
        this.#column = column;
        this.#line = line;
        this.#x = x;
        this.#y = y;
    }

    public getColumn() {
        return this.#column;
    }

    public getLine() {
        return this.#line;
    }

    public getX() {
        return this.#x;
    }

    public setX(x: number) {
        this.#x = x;
    }

    public getY() {
        return this.#y;
    }

    public setY(y: number) {
        this.#y = y;
    }

}
