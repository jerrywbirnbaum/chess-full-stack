
use chess::{ALL_RANKS, ALL_FILES, Square, Color, Board};

pub fn _print_board(board: Board){
    for rank in ALL_RANKS.iter().rev(){
        for file in ALL_FILES.iter() {
            let square = Square::make_square(*rank, *file);
            let mut color = Color::White;
            match board.color_on(square){
                Some(c) => {color = c;},
                None => (),
            }

            let mut piece_str;
            match board.piece_on(square){
                Some(p) => {piece_str = p.to_string(color);},
                None => {piece_str = " ".to_string();}
            }

            match piece_str.as_str() {
                "k"=> piece_str = '♔'.to_string(),
                "q"=> piece_str = '♕'.to_string(),
                "r"=> piece_str = '♖'.to_string(),
                "b"=> piece_str = '♗'.to_string(),
                "n"=> piece_str = '♘'.to_string(),
                "p"=> piece_str = '♙'.to_string(),
                "K"=> piece_str = '♚'.to_string(),
                "Q"=> piece_str = '♛'.to_string(),
                "R"=> piece_str = '♜'.to_string(),
                "B"=> piece_str = '♝'.to_string(),
                "P"=> piece_str = 'p'.to_string(),
                "N"=> piece_str = '♞'.to_string(),

                _ => ()
            }

            print!("{}", piece_str);
            print!(" ")
        }
        println!();
    }
}
