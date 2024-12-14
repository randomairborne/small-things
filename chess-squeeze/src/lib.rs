fn pack(board: ChessBoard) -> SqueezedChessBoard {

}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct SqueezedChessBoard(usize, usize, usize);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ChessBoard {
    board: [Option<ChessPiece>; 64],
    black_castle: CastleState,
    white_castle: CastleState,
    en_passant: EnPassant,
    fifty_move_draw_ctr: usize,
    to_move: ToMove,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct CastleState {
    queenside: bool,
    kingside: bool,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum ToMove {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
/// Which pawn has moved two spaces last turn
struct EnPassant {
    black: Option<u8>,
    white: Option<u8>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ChessPiece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}
