import React, { useEffect, useState, useRef } from "react";

import "./App.scss";
import Piece from "./game/Piece";

import BlackBishop from "./assets/pieces/black-bishop.svg";
import BlackKing from "./assets/pieces/black-king.svg";
import BlackKnight from "./assets/pieces/black-knight.svg";
import BlackPawn from "./assets/pieces/black-pawn.svg";
import BlackQueen from "./assets/pieces/black-queen.svg";
import BlackRooq from "./assets/pieces/black-rooq.svg";

import WhiteBishop from "./assets/pieces/white-bishop.svg";
import WhiteKing from "./assets/pieces/white-bishop.svg";
import WhiteKnight from "./assets/pieces/white-bishop.svg";
import WhitePawn from "./assets/pieces/white-bishop.svg";
import WhiteQueen from "./assets/pieces/white-bishop.svg";
import WhiteRooq from "./assets/pieces/white-bishop.svg";

import EPieceType from "./game/PieceType";
import EPieceColor from "./game/PieceColor";
import Game from "./game/Board";
import Position from "./game/Position";

const PIECES_ICONS: { [key: string]: string } = {
    "black-bishop": BlackBishop,
    "black-king": BlackKing,
    "black-knight": BlackKnight,
    "black-pawn": BlackPawn,
    "black-queen": BlackQueen,
    "black-rooq": BlackRooq,
    "white-bishop": WhiteBishop,
    "white-king": WhiteKing,
    "white-knight": WhiteKnight,
    "white-pawn": WhitePawn,
    "white-queen": WhiteQueen,
    "white-rooq": WhiteRooq,
};

const App: React.FC = () => {
    const gameRef = useRef(new Game());
    const [board, setBoard] = useState(gameRef.current.getBoard());
    const draggedPosition = useRef<[number, number] | null>(null)

    const onDragStart = (e: any, i: number, j: number) => {
        console.log("Dragged");

        draggedPosition.current = [i, j]
    };

    const onDrop = (e: any, i: number, j: number) => {
        e.preventDefault();
        if(!draggedPosition.current || !gameRef.current) return

        const draggedX = draggedPosition.current[0]
        const draggedY = draggedPosition.current[1]

        const piece = board[draggedX][draggedY]
        
        if(!piece) return

        gameRef.current.removePiece(piece)

        const position = piece.getPosition();
        position?.setX(i)
        position?.setY(j)

        gameRef.current.placePiece(piece)

        setBoard([...gameRef.current.getBoard()]);

        return false;
    };

    const addPiece = () => {
        const piece = new Piece(EPieceColor.BLACK, EPieceType.KING, new Position(0, 0, 0, "A"));

        gameRef.current.placePiece(piece);

        setBoard([...gameRef.current.getBoard()]);
    };

    const createPieceElement = (piece: Piece | null, i: number, j: number) => {
        if (piece === null) {
            return null;
        }

        return (
            <div
                className="piece bg-no-repeat bg-center bg-cover"
                draggable
                onDragStart={(e) => onDragStart(e, i ,j)}
                style={{ backgroundImage: `url(${PIECES_ICONS[`${piece.getColor()}-${piece.getType()}`]})` }}
            ></div>
        );
    };

    return (
        <div className="App">
            <button onClick={addPiece}>Add piece</button>
            <div className="grid grid-rows-8" id="board">
                {board.map((row, i) => {
                    const rowType = (i + 1) % 2 === 0 ? "even" : "odd";
                    return (
                        <div key={i} className={`row ${rowType}-row grid-cols-8 grid`}>
                            {row.map((obj, j) => (
                                <div
                                    key={j}
                                    className="cell flex items-center justify-center"
                                    onDragEnter={(e) => e.preventDefault()}
                                    onDragOver={(e) => e.preventDefault()}
                                    onDrop={(e) => onDrop(e, i, j)}
                                >
                                    {createPieceElement(obj, i, j)}
                                </div>
                            ))}
                        </div>
                    );
                })}
            </div>
        </div>
    );
};

export default App;
