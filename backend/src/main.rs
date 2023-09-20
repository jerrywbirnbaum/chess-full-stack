use chess::{Board, Square, ChessMove, MoveGen};
mod evaluation;
use evaluation::mini_max_alpha_beta;
use std::str::FromStr;
mod display;

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
        let new_eval = mini_max_alpha_beta(board.make_move_new(m), 3, -9999, 9999, true, &mut positions);
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