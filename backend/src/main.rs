use chess::{Board, BoardStatus, Square, ChessMove, MoveGen};
use chess::Color;
use std::io::{stdin,stdout,Write};
mod evaluation;
use evaluation::mini_max_alpha_beta;
use evaluation::mini_max;
use std::str::FromStr;
mod display;
use display::print_board;

#[macro_use] extern crate rocket;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/best_move/<fen>")]
fn best_move(fen: &str) -> String {
    let mut board = Board::from_str(fen).expect("valid position");

    let mut iterable = MoveGen::new_legal(&board);
    let mut cpu_move = iterable.next().unwrap();
    let mut eval = 999999;

    for m in iterable{
        let mut positions: u32 = 0;
        let new_eval = mini_max(board.make_move_new(m), 3, true, &mut positions);

        let mut positions2: u32 = 0;
        let new_eval = mini_max_alpha_beta(board.make_move_new(m), 3, -9999, 9999, true, &mut positions2);
        if new_eval < eval{
            eval = new_eval;
            cpu_move = m
        }
    }

    board = board.make_move_new(cpu_move);
    format!("{}", board)

}

#[get("/player_move/<fen>/<source>/<dest>")]
fn player_move(fen: &str, source: u8, dest: u8) -> String {
    let mut board = Board::from_str(fen).expect("valid position");

    let player_move = ChessMove::new(unsafe {Square::new(source)} , unsafe {Square::new(dest)}, None);
    if board.legal(player_move){
        board = board.make_move_new(player_move);
    }

    format!("{}", board)
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS).mount("/", routes![best_move]).mount("/", routes![player_move])
}

// fn main_loop() {

//     // create a board with the initial position
//     let mut board = Board::default();
//     // let mut board = Board::from_str("3k4/8/8/8/q7/8/6PP/6K1 w - - 0 1").expect("valid position");

//     while board.status() == BoardStatus::Ongoing {
//         print_board(board);

//         let valid_move;
//         loop {
//             let mut s=String::new();
//             print!("Enter your move: ");
//             let _=stdout().flush();
//             stdin().read_line(&mut s).expect("Did not enter a correct string");
//             if let Some('\n')=s.chars().next_back() {
//                 s.pop();
//             }
//             if let Some('\r')=s.chars().next_back() {
//                 s.pop();
//             }
//             match ChessMove::from_san(&board, &s){
//                 Ok(_) => {valid_move = ChessMove::from_san(&board, &s).unwrap();
//                 break;},
//                 Err(_) => {println!("Invalid Move");continue;},

//             };

//         }
//         board = board.make_move_new(valid_move);

//         let mut iterable = MoveGen::new_legal(&board);
//         if iterable.len() == 0{
//             break;
//         }
//         let mut cpu_move = iterable.next().unwrap();
//         let mut eval = 999999;

//         for m in iterable{
//             let mut positions: u32 = 0;
//             let new_eval = mini_max(board.make_move_new(m), 3, true, &mut positions);
//             print!("Positions evaluated: ");
//             println!("{}", positions);

//             let mut positions2: u32 = 0;
//             let new_eval = mini_max_alpha_beta(board.make_move_new(m), 3, -9999, 9999, true, &mut positions2);
//             print!("Positions evaluated alpha beta: ");
//             println!("{}", positions2);
//             if new_eval < eval{
//                 eval = new_eval;
//                 cpu_move = m
//             }
//         }

//         board = board.make_move_new(cpu_move);
//     }

//     print_board(board);

//     if board.status() == BoardStatus::Stalemate{
//         println!("Stalemate");
//     }

//     if board.status() ==BoardStatus::Checkmate && board.side_to_move() == Color::White{
//         println!("Checkmate Black wins");
//     }

//     if board.status() ==BoardStatus::Checkmate && board.side_to_move() == Color::Black{
//         println!("Checkmate White wins");
//     }

// }
