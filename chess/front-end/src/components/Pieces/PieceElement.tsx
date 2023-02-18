import React from "react";

import Piece from "../../game/Pieces/Piece";

import { PIECES_ICONS } from "./PieceIcons";

interface IPiece {
    selectedPosition: [number, number] | null;
    setDraggedPosition: React.Dispatch<React.SetStateAction<[number, number] | null>>;
    piece: Piece;
}

const PieceElement: React.FC<IPiece> = ({ selectedPosition, piece, setDraggedPosition }) => {
    if (piece === null || piece.getPosition() === null) {
        return null;
    }

    const togglePieceSelection = (i: number, j: number) => {
        if (selectedPosition === null) {
            setDraggedPosition([i, j]);
        } else {
            const x = selectedPosition[0];
            const y = selectedPosition[1];

            if (x === i && y === j) {
                setDraggedPosition(null);
            }
        }
    };

    let piecePosition = piece.getPosition();
    let x = piecePosition?.getX() || 0;
    let y = piecePosition?.getY() || 0;

    return (
        <div
            className="piece bg-no-repeat bg-center bg-cover w-full h-full"
            draggable
            onClick={(e) => togglePieceSelection(x, y)}
            style={{ backgroundImage: `url(${PIECES_ICONS[`${piece.getColor()}-${piece.getType()}`]})` }}
        ></div>
    );
};

export default PieceElement;
