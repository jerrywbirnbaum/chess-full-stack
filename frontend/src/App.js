import React, { useState, useEffect } from 'react';
import {ChessBoard, ChessBoardDndProvider} from "react-fen-chess-board";


function App(){
  return (
  <ChessBoardDndProvider>
    <MyChessBoard/>
  </ChessBoardDndProvider>);
}



const MyChessBoard = () => {
  const [fen, setFen] = useState("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

  // useEffect(() => {
  //   fetch('http://localhost:8000/best_move/3k4%2F8%2F8%2F8%2Fq7%2F8%2F6PP%2F6K%20w%20-%20-%200%201')
  //     .then(response => response.text())
  //     .then(json => setFen(json))
  //     .catch(error => console.error(error));
  // }, []);

  return (
    <ChessBoard fen={fen}
    onMove={({fromPosition, toPosition, board}) => {
      let from_coords = fromPosition.x + 8 * (7 - fromPosition.y);
      let to_coords = toPosition.x + 8 * (7 - toPosition.y);

      let formatted_fen = fen.replaceAll('/', '%2F');
      formatted_fen = formatted_fen.replaceAll(' ', '%20');
      fetch(`http://localhost:8000/player_move/${formatted_fen}/${from_coords}/${to_coords}`)
        .then(response => response.text())
        .then(fen => fetch(`http://localhost:8000/best_move/${fen.replaceAll('/', '%2F').replaceAll(' ', '%20')}`)
        .then(response => response.text())
        .then(json => setFen(json))
        .then(console.log(fen))
        .catch(error => console.error(error)
        ))
        .then(console.log(fen))
        .then()

        .catch(error => console.error(error));
    }}/>

  );
}

export default App;