import { TBoard, TPieceColor, TPieceType } from "./types";

export const BOARD: TBoard = {
  blackCaptures: [],
  whiteCaptures: [],
  pieces: {
    "7-6": {
      color: TPieceColor.White,
      type: TPieceType.King,
      position: {
        row: 7,
        column: 6,
      },
      availableMoves: [
        {
          row: 7,
          column: 7,
        },
        {
            row: 6,
            column: 6,
          }
      ],
    },
    "7-7": {
      color: TPieceColor.Black,
      type: TPieceType.King,
      position: {
        row: 7,
        column: 7,
      },
      availableMoves: [
        {
          row: 7,
          column: 6,
        },
        {
          row: 6,
          column: 7,
        },
        {
          row: 6,
          column: 6,
        },
      ],
    },
  },
};
