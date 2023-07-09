export type TBoard = {
  blackCaptures: string[];
  whiteCaptures: string[];
  pieces: { [key: string]: TBoardPiece };
};

export type TBoardPiece = {
  color: TPieceColor;
  type: TPieceType;
  position: TBoardPosition;
  availableMoves: TBoardPosition[];
};

export type TBoardPosition = {
  row: number;
  column: number;
};

export enum TPieceColor {
  Black = "black",
  White = "white",
}

export enum TPieceType {
  Bishop = "bishop",
  King = "king",
  Knight = "knight",
  Pawn = "pawn",
  Queen = "queen",
  Rook = "rook",
}
