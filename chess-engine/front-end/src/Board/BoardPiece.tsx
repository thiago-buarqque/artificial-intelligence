import React from "react";

import BishopBlack from "../assets/bishop-black.svg";
import BishopWhite from "../assets/bishop-white.svg";
import KingBlack from "../assets/king-black.svg";
import KingWhite from "../assets/king-white.svg";
import KnightBlack from "../assets/knight-black.svg";
import KnightWhite from "../assets/knight-white.svg";
import PawnBlack from "../assets/pawn-black.svg";
import PawnWhite from "../assets/pawn-white.svg";
import QueenBlack from "../assets/queen-black.svg";
import QueenWhite from "../assets/queen-white.svg";
import RookBlack from "../assets/rook-black.svg";
import RookWhite from "../assets/rook-white.svg";

import { TBoardPiece, TBoardPosition } from "./types";

const PIECES: {
  [key: string]: string;
} = {
  "bishop-black": BishopBlack,
  "bishop-white": BishopWhite,
  "king-black": KingBlack,
  "king-white": KingWhite,
  "knight-black": KnightBlack,
  "knight-white": KnightWhite,
  "pawn-black": PawnBlack,
  "pawn-white": PawnWhite,
  "queen-black": QueenBlack,
  "queen-white": QueenWhite,
  "rook-black": RookBlack,
  "rook-white": RookWhite,
};

interface IProps {
  boardPiece: TBoardPiece;
  onClick: (position: TBoardPiece) => void;
}

const BoardPiece: React.FC<IProps> = ({ boardPiece, onClick }) => {
  const { availableMoves, color, position, type } = boardPiece;

  return (
    <button
      className="piece-button"
      onClick={(e) => {
        if (!e.currentTarget.classList.contains("disabled")) {
          onClick(boardPiece);
        }
      }}
    >
      {<img className="piece" src={PIECES[type + "-" + color]} alt={type + "-" + color} />}
    </button>
  );
};

export default BoardPiece;
