import React, { useState, useRef, useEffect } from "react";

import "./App.scss";
import Piece from "./game/Pieces/Piece";

import EPieceType from "./game/Pieces/PieceType";
import EPieceColor from "./game/Pieces/PieceColor";
import Game from "./game/Game";
import Position from "./game/Pieces/Position";
import PieceElement from "./components/Pieces/PieceElement";

const GameContext: React.FC = () => {
    const gameRef = useRef(new Game());
    const [board, setBoard] = useState(gameRef.current.getBoard());
    const [selectedPosition, setSelectedPosition] = useState<[number, number] | null>(null);

    const movePiece = (e: any, i: number, j: number) => {
        if (!selectedPosition || !gameRef.current) return;

        const x = selectedPosition[0];
        const y = selectedPosition[1];

        if (x === i && y === j) return;
        console.log("Clicked on board position");

        const piece = board[x][y];

        // Must throw an error?
        if (!piece) return;

        gameRef.current.removePiece(piece);

        const position = piece.getPosition();
        position?.setX(i);
        position?.setY(j);

        gameRef.current.placePiece(piece);

        setBoard([...gameRef.current.getBoard()]);

        setSelectedPosition(null);
    };

    const addPiece = () => {
        const piece = new Piece(EPieceColor.BLACK, new Position(0, 0, 0, "A"), EPieceType.KING);

        gameRef.current.placePiece(piece);

        setBoard([...gameRef.current.getBoard()]);
    };

    const highlightAvailableMoves = () => {};

    useEffect(() => {
        highlightAvailableMoves();
    }, [selectedPosition]);

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
                                    onClick={(e) => {
                                        if (selectedPosition !== null) {
                                            movePiece(e, i, j);
                                        }
                                    }}
                                >
                                    {obj && (
                                        <PieceElement
                                            selectedPosition={selectedPosition}
                                            setDraggedPosition={setSelectedPosition}
                                            piece={obj}
                                        />
                                    )}
                                </div>
                            ))}
                        </div>
                    );
                })}
            </div>
        </div>
    );
};

export default GameContext;
