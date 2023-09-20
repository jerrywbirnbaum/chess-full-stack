use chess::{Board, BoardStatus, MoveGen, Square};
use chess::Piece;
use chess::Color;


pub fn simple_eval(board: Board) -> i32{
    if board.status() == BoardStatus::Stalemate{
        return 0;
    }

    if board.status() ==BoardStatus::Checkmate && board.side_to_move() == Color::White{
        return -9999;
    }

    if board.status() ==BoardStatus::Checkmate && board.side_to_move() == Color::Black{
        return 9999;
    }

    let mut white_eval = (board.pieces(Piece::Pawn) & board.color_combined(Color::White)).popcnt();
    white_eval += (board.pieces(Piece::Knight) & board.color_combined(Color::White)).popcnt() * 3;
    white_eval += (board.pieces(Piece::Bishop) & board.color_combined(Color::White)).popcnt() * 3;
    white_eval += (board.pieces(Piece::Rook) & board.color_combined(Color::White)).popcnt() * 5;
    white_eval += (board.pieces(Piece::Queen) & board.color_combined(Color::White)).popcnt() * 9;

    let mut black_eval = (board.pieces(Piece::Pawn) & board.color_combined(Color::Black)).popcnt();
    black_eval += (board.pieces(Piece::Knight) & board.color_combined(Color::Black)).popcnt() * 3;
    black_eval += (board.pieces(Piece::Bishop) & board.color_combined(Color::Black)).popcnt() * 3;
    black_eval += (board.pieces(Piece::Rook) & board.color_combined(Color::Black)).popcnt() * 5;
    black_eval += (board.pieces(Piece::Queen) & board.color_combined(Color::Black)).popcnt() * 9;

    let mut eval: i32 = i32::try_from(white_eval).unwrap() - i32::try_from(black_eval).unwrap();
    // if board.side_to_move() == Color::Black{
    //     eval = -eval
    // }
    return eval;
}

pub fn mini_max(board: Board, depth: i32, maximizing_player: bool, positions:  &mut u32) -> i32{
    if depth == 0{
        *positions = *positions + 1;
        return simple_eval(board)
    }
    let mut best_eval;
    if maximizing_player{
        best_eval = -999999;
        for m in MoveGen::new_legal(&board){
            let mut new_board = board;
            new_board = board.make_move_new(m);
            let current_eval = mini_max(new_board, depth - 1, !maximizing_player, positions);
            best_eval = std::cmp::max(current_eval, best_eval);
        }
    }
    else{
        best_eval = 999999;
        for m in MoveGen::new_legal(&board){
            let mut new_board = board;
            new_board = board.make_move_new(m);
            let current_eval = mini_max(new_board, depth - 1, !maximizing_player, positions);
            best_eval = std::cmp::min(current_eval, best_eval);
        }
    }

    return best_eval;
}

pub fn mini_max_alpha_beta(board: Board, depth: i32, mut alpha: i32, mut beta: i32, maximizing_player: bool, positions: &mut u32) -> i32{
    if depth == 0{
        *positions = *positions + 1;
        return simple_eval(board)
    }

    let mut best_eval;
    if maximizing_player{
        best_eval = -999999;
        for m in MoveGen::new_legal(&board){
            let mut new_board = board;
            new_board = board.make_move_new(m);
            let current_eval = mini_max_alpha_beta(new_board, depth - 1, alpha, beta, !maximizing_player, positions);
            best_eval = std::cmp::max(current_eval, best_eval);
            if best_eval > beta{
                break;
            }
            alpha = std::cmp::max(current_eval, alpha)
        }
    }
    else{
        best_eval = 999999;
        for m in MoveGen::new_legal(&board){
            let mut new_board = board;
            new_board = board.make_move_new(m);
            let current_eval = mini_max_alpha_beta(new_board, depth - 1, alpha, beta, !maximizing_player, positions);
            best_eval = std::cmp::min(current_eval, best_eval);
            if best_eval < alpha{
                break;
            }
            beta = std::cmp::min(current_eval, beta)
        }
    }
    return best_eval;
}
