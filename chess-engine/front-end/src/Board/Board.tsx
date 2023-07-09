import React, { MouseEventHandler, useEffect, useRef, useState } from "react";

import { BOARD } from "./mock-data";
import BoardPiece from "./BoardPiece";
import { TBoardPiece, TBoardPosition } from "./types";

//@ts-ignore
import captureAudio from "../assets/sound/capture.mp3";
//@ts-ignore
import moveAudio from "../assets/sound/move-self.mp3";

import "./board.scss";

const LINES = [0, 1, 2, 3, 4, 5, 6, 7];
const COLUMNS: { [key: number]: string } = {
  0: "A",
  1: "B",
  2: "C",
  3: "D",
  4: "E",
  5: "F",
  6: "G",
  7: "H",
};

const playMoveAudio = (capture: boolean) => {
  let audio;

  if (capture) {
    audio = new Audio(captureAudio);
  } else {
    audio = new Audio(moveAudio);
  }
  audio.play();
};

const isNotAnAvailableMove = (availableMoves: TBoardPosition[], column: number, row: number) => !availableMoves.find((move) => move.row === row && move.column === column)

const Board = () => {
  const [selectedPiece, setSelectedPiece] = useState<TBoardPiece | null>(null);
  const [board, setBoard] = useState(BOARD);

  const onPieceSelect = (piece: TBoardPiece) => {
    console.log("Piece click");
    if (selectedPiece === piece) {
      setSelectedPiece(null);
    } else {
      if (selectedPiece) {
        togglePieceAvailableMoves(selectedPiece);
      }

      setSelectedPiece(piece);
    }
    togglePieceAvailableMoves(piece);
  };

  const togglePieceAvailableMoves = (piece: TBoardPiece) => {
    piece.availableMoves.forEach((move) => {
      const className = board.pieces[`${move.row}-${move.column}`]
        ? "capture-receptor"
        : "empty-receptor";

      const cell = document.querySelector(
        `.cell[data-pos='${move.row}-${move.column}']`
      ) as HTMLDivElement;

      // cell.onclick = () => onCellClick(cell, move.row, move.column);
      cell.classList.toggle(className);

      const cellPiece = document.querySelector(
        `.cell[data-pos='${move.row}-${move.column}'] button.piece-button`
      ) as HTMLDivElement;

      cellPiece?.classList.toggle("disabled");
    });
  };

  const onCellClick = (cell: HTMLDivElement, row: number, column: number) => {
    console.log("Cell click");
    if (selectedPiece) {
      const { position, availableMoves } = selectedPiece;

      if (isNotAnAvailableMove(availableMoves, column, row)) {
        return;
      }

      const copy_board = JSON.parse(JSON.stringify(board));

      let capture = false;
      if (copy_board.pieces[row + "-" + column]) {
        capture = true;
      }
      // @ts-ignore
      copy_board.pieces[position.row + "-" + position.column] = undefined;

      selectedPiece.position.row = row;
      selectedPiece.position.column = column;

      copy_board.pieces[row + "-" + column] = selectedPiece;

      setSelectedPiece(null);
      setBoard(copy_board);

      const cellPiece = document.querySelector(
        `.cell[data-pos='${row}-${column}'] button.piece-button.disabled`
      ) as HTMLDivElement;

      cellPiece?.classList.remove("disabled");

      playMoveAudio(capture);
      if(capture) {
        cell.classList.remove("capture-receptor")
      }
      togglePieceAvailableMoves(selectedPiece);
      // currentTarget.onclick = null;
      // send request to server and update the state with the result
    }
  };

  return (
    <div id="board">
      {LINES.map((i) => (
        <div key={i} className="row">
          {LINES.map((j) => (
            <div
              key={j}
              className="cell"
              data-pos={i + "-" + j}
              onClick={(e) => onCellClick(e.currentTarget, i, j)}
            >
              {j === 0 && (
                <span className={`row-index ${(i + 1) % 2 !== 0 ? "white" : ""}`}>{i + 1}</span>
              )}
              {i === 7 && (
                <span className={`column-index ${(j + 1) % 2 === 0 ? "white" : ""}`}>
                  {COLUMNS[j]}
                </span>
              )}
              {board.pieces[i + "-" + j] ? (
                <BoardPiece boardPiece={board.pieces[i + "-" + j]} onClick={onPieceSelect} />
              ) : (
                <div className="move-dot"></div>
              )}
            </div>
          ))}
        </div>
      ))}
    </div>
  );
};

export default Board;
