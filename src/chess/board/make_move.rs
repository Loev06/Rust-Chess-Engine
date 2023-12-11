use super::{Board, super::Move, PieceType, PieceType::*, util, precomputed, undo_move::GSHistoryEntry};

impl Board {
    pub fn make_move(&mut self, mv: &Move) {
        let from = mv.get_from();
        let to = mv.get_to();
        let moving_piece = self.piece_list[from as usize].expect("Moving piece should exist");
        let capturing_piece = self.piece_list[to as usize];

        let new_piece_type = if mv.is_promotion() {
            mv.get_promotion_piece(self.gs.player_to_move)
        } else {
            moving_piece
        };

        self.gs_history.push(GSHistoryEntry {
            gs: self.gs,
            captured_piece: capturing_piece
        });
        self.gs.en_passant_mask = precomputed::EMPTY; // Double pawn push check is later
        self.gs.castling_rights.update(from, to);
        
        if let Some(pt) = capturing_piece {
            self.remove_piece(pt, to);
        } else {
            match new_piece_type {
                WPawn | BPawn => {
                    if mv.is_ep() {
                        self.remove_piece(PieceType::from_color(WPawn, self.gs.opponent_color), to ^ 8); // En-passant
                    } else if mv.intersects(Move::DOUBLE_PAWN_PUSH) {
                        self.gs.en_passant_mask = util::bitboard_from_square(to ^ 8) // Double pawn push
                    }
                },
                WKing | BKing => {
                    let rook_type = PieceType::from_color(WRook, self.gs.player_to_move);
                    if mv.contains(Move::QUEEN_CASTLE) {
                        self.remove_piece(rook_type, from - 4); // Queen castle
                        self.place_piece(rook_type, from - 1);
                    } else if mv.contains(Move::KING_CASTLE) {
                        self.remove_piece(rook_type, from + 3); // King castle
                        self.place_piece(rook_type, from + 1);
                    }
                },
                _ => ()
            }
        }
        
        self.remove_piece(moving_piece, from);
        self.place_piece(new_piece_type, to);
        
        self.gs.switch_sides();
        self.update_board_data();
    }
}