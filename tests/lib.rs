extern crate chess_pgn_parser;

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

use chess_pgn_parser::{GameTermination, MoveNumber, NAG};
use chess_pgn_parser::GameTermination::*;
use chess_pgn_parser::AnnotationSymbol::*;
use chess_pgn_parser::Piece::*;
use chess_pgn_parser::Move::*;

fn get_sample_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/samples");
    path.push(filename);
    path
}

fn read_sample(filename: &str) -> String {
    let sample_path = get_sample_path(filename);

    let mut file = File::open(sample_path.to_str().unwrap()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

#[test]
fn philidor() {
    //Sample taken from http://pgnmentor.com/files.html
    let text = read_sample("Philidor.pgn");

    let result = chess_pgn_parser::read_games(&text);
    assert!(result.is_ok());

    println!("{:?}", result);

    let games = result.ok().unwrap();
    assert_eq!(games.len(), 6);

    let results: Vec<GameTermination> = games.iter().map(|game| game.termination).collect();

    assert_eq!(
        results,
        vec![
            WhiteWins,
            BlackWins,
            BlackWins,
            DrawnGame,
            WhiteWins,
            BlackWins,
        ]
    );
}

#[test]
fn aronian_caruana() {
    //Sample taken from http://en.chessbase.com/post/huffington-on-aronian-s-chess-comeback
    let text = read_sample("aronian_caruana.pgn");

    let games = chess_pgn_parser::read_games(&text).unwrap();
    assert_eq!(games.len(), 1);

    let game = &games[0];
    let white25 = &game.moves[0];
    assert_eq!(white25.number, Some(MoveNumber::White(25)));
    assert_eq!(white25.nag, Some(NAG(3)));

    let black28 = &game.moves[7];
    assert_eq!(black28.variations[0].comment, Some("After".to_string()));
}

#[test]
fn euwe_alekhine() {
    //Sample taken from http://www.chess.com/article/view/the-openings-of-world-champions-max-euwe-and-the-slav-defense
    let text = read_sample("euwe_alekhine.pgn");

    let games = chess_pgn_parser::read_games(&text).unwrap();
    assert_eq!(games.len(), 1);

    let game = &games[0];
    let black21 = &game.moves[41];

    assert_eq!(black21.move_.annotation_symbol, Some(Good));
}

#[test]
fn cutechess() {
    //Sample produced by cutechess log
    let text = read_sample("results.pgn");

    let games = chess_pgn_parser::read_games(&text).unwrap();
    assert_eq!(games.len(), 1);

    let game = &games[0];
    let white58 = &game.moves[114];

    match white58.move_.move_ {
        BasicMove {
            piece: _,
            to: _,
            from: _,
            is_capture: _,
            ref promoted_to,
        } => assert_eq!(promoted_to.clone(), Some(Queen)),
        _ => assert!(false),
    }
}
