export type TBoard = {
  blackCaptures: string[];
  pieces: TBoardPiece[];
  whiteCaptures: string[];
  whiteMove: boolean;
  winner: null | "b" | "w"
};

export type TBoardPiece = {
  white: boolean;
  moves: number[];
  position: number;
  fen: string | null;
  // Add castle/check/blocked info?
};

export enum TPieceColor {
  Black = 8,
  White = 16
}

export enum TPieceType {
  Empty = 0,
  Bishop = 1,
  King = 2,
  Knight = 3,
  Pawn = 4,
  Queen = 5,
  Rook = 6
}
