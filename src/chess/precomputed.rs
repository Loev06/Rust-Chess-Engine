use std::isize;

use super::{Square, util, Bitboard};

pub const KNIGHT_OFFSETS: [(isize, isize); 8] = [(-1, 2), (1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1)];
pub const KING_OFFSETS:   [(isize, isize); 8] = [(-1, 1), (0, 1), (1, 1), (1, 0 ), (1, -1), (0 , -1), (-1, -1), (-1, 0)];
pub const ROOK_DIRS:    [(isize, isize); 4] = [(0 , 1), (1, 0), (0, -1), (-1, 0 )];
pub const BISHOP_DIRS:  [(isize, isize); 4] = [(-1, 1), (1, 1), (1, -1), (-1, -1)];

pub const KNIGHT_MOVES: [Bitboard; 64] = precompute_direct_piece_moves(KNIGHT_OFFSETS);
pub const KING_MOVES:   [Bitboard; 64] = precompute_direct_piece_moves(KING_OFFSETS);

pub const ROOK_RAYS:    [[Bitboard; 4]; 64] = precompute_slider_rays(ROOK_DIRS, false);
pub const BISHOP_RAYS:  [[Bitboard; 4]; 64] = precompute_slider_rays(BISHOP_DIRS, false);

pub const ROOK_MOVES:   [Bitboard; 64] = merge_slider_rays(ROOK_RAYS);
pub const BISHOP_MOVES: [Bitboard; 64] = merge_slider_rays(BISHOP_RAYS);

pub const ROOK_MOVES_NO_BORDER: [Bitboard; 64] = merge_slider_rays(
    precompute_slider_rays(ROOK_DIRS, true)
);
pub const BISHOP_MOVES_NO_BORDER: [Bitboard; 64] = merge_slider_rays(
    precompute_slider_rays(BISHOP_DIRS, true)
);

pub const BETWEEN_BITBOARDS: [[Bitboard; 64]; 64] = precompute_between_bitboards();

pub const BORDER     : Bitboard = 0xff818181818181ff;
pub const NOT_A_FILE : Bitboard = 0xfefefefefefefefe;
pub const NOT_H_FILE : Bitboard = 0x7f7f7f7f7f7f7f7f;
pub const SECOND_ROW : Bitboard = 0x000000000000ff00;
pub const SEVENTH_ROW: Bitboard = 0x00ff000000000000;
pub const EMPTY      : Bitboard = 0x0000000000000000;
pub const FULL       : Bitboard = 0xffffffffffffffff;

pub const E1: Square = 4;
pub const E8: Square = 60;
pub const E1BB: Bitboard = util::bitboard_from_square(E1);
pub const E8BB: Bitboard = util::bitboard_from_square(E8);

const fn precompute_direct_piece_moves(offsets: [(isize, isize); 8]) -> [Bitboard; 64] {
    let mut moves = [EMPTY; 64];
    let mut sq: Square = 0;
    while sq < 64 { // 'for' is not allowed in a const fn..
        let mut j = 0;
        while j < 8 {
            let offset = offsets[j];

            j += 1;

            let x: isize = util::get_square_x(sq) as isize + offset.0;
            let y: isize = util::get_square_y(sq) as isize + offset.1;

            if util::is_out_of_bounds(x, y) {
                continue;
            }

            moves[sq] |= util::bitboard_from_square(util::square_from_coord(x as usize, y as usize));
        }
        sq += 1;
    }
    moves
}

const fn precompute_slider_rays(dirs: [(isize, isize); 4], end_before_border: bool) -> [[Bitboard; 4]; 64] {
    let mut rays = [[EMPTY; 4]; 64];

    let mut sq: Square = 0;
    while  sq < 64 {
        let mut dir = 0;
        while dir < 4 {
            let mut x = util::get_square_x(sq) as isize;
            let mut y = util::get_square_y(sq) as isize;

            loop {
                x += dirs[dir].0;
                y += dirs[dir].1;

                if util::is_out_of_bounds(x, y) {
                    break;
                }

                if util::is_out_of_bounds(x + dirs[dir].0, y + dirs[dir].1)
                    && end_before_border
                {
                    break;
                }

                rays[sq][dir] |= util::bitboard_from_square(util::square_from_coord(x as usize, y as usize)); 
            }
            dir += 1;
        }
        sq += 1;
    }

    rays
}

const fn merge_slider_rays(rays: [[Bitboard; 4]; 64]) -> [Bitboard; 64] {
    let mut moves = [EMPTY; 64];

    let mut sq: Square = 0;
    while sq < 64 {
        let mut dir = 0;
        while dir < 4 {
            moves[sq] |= rays[sq][dir];

            dir += 1;
        }
        sq += 1;
    }

    moves
}

const fn precompute_between_bitboards() -> [[Bitboard; 64]; 64] {
    let mut between_bitboards = [[EMPTY; 64]; 64];

    let mut sq1: Square = 0;
    while sq1 < 64 {
        let mut sq2: Square = 0;
        while sq2 < 64 {
            let dx = util::get_square_x(sq2) as isize - util::get_square_x(sq1) as isize;
            let dy = util::get_square_y(sq2) as isize - util::get_square_y(sq1) as isize;

            between_bitboards[sq1][sq2] = match (dx, dy) {
                (0    , 1..  ) => ROOK_RAYS[sq1][0] & ROOK_RAYS[sq2][2],
                (1..  , 0    ) => ROOK_RAYS[sq1][1] & ROOK_RAYS[sq2][3],
                (0    , ..=-1) => ROOK_RAYS[sq1][2] & ROOK_RAYS[sq2][0],
                (..=-1, 0    ) => ROOK_RAYS[sq1][3] & ROOK_RAYS[sq2][1],
                (a@..=-1, b@1..  ) if -a ==  b => BISHOP_RAYS[sq1][0] & BISHOP_RAYS[sq2][2],
                (a@1..  , b@1..  ) if  a ==  b => BISHOP_RAYS[sq1][1] & BISHOP_RAYS[sq2][3],
                (a@1..  , b@..=-1) if  a == -b => BISHOP_RAYS[sq1][2] & BISHOP_RAYS[sq2][0],
                (a@..=-1, b@..=-1) if -a == -b => BISHOP_RAYS[sq1][3] & BISHOP_RAYS[sq2][1],
                (_, _) => EMPTY
            };

            sq2 += 1;
        }
        sq1 += 1;
    }

    between_bitboards
}