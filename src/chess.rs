mod board;
pub use self::board::Board;

mod chess_move;
pub use self::chess_move::Move;

mod castling_flags;
pub use self::castling_flags::CastlingFlags;

mod piece_type;
pub use self::piece_type::PieceType;
pub use self::piece_type::Color;

mod move_gen;
pub use self::move_gen::MoveGenerator;

mod move_list;
pub use self::move_list::MoveList;

mod perft;
pub use perft::Perft;

mod precomputed;
mod util;

pub const MAX_MOVE_COUNT: usize = 218;

pub type Square = u8;
pub type Bitboard = u64;