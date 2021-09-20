use std::sync::Arc;
use std::io::{self, Write};
use std::collections::HashSet;
use rayon::prelude::*;

type PieceData = [[i32; 5]; 5];
type BoardData = [[i32; 6]; 10];
type PieceVec  = Vec<HashSet<Piece>>;

struct Board {
	b: BoardData,
	row: usize,
	col: usize
}

impl Board {
	fn new(x: &BoardData) -> Board {
		Board {
			b: *x,
			row: x.len(),
			col: x[0].len()
		}
	}

	fn show(&self, labels: &[char]) {
		let stdout = io::stdout();
		let mut handle = stdout.lock();
		handle.write_all(b"======BOARD======\n").unwrap();
		for r in self.b.iter() {
			handle.write_all(&r.iter().map(|i| labels[*i as usize]).collect::<String>().as_bytes()).unwrap();
			handle.write(b"\n").unwrap();
		}
		handle.flush().unwrap();
	}

	fn copy_from(&mut self, from: &Board) {
		if self.row == from.row && self.col == from.col {
			for r in 0..self.row {
				for c in 0..self.col {
					self.b[r][c] = from.b[r][c];
				}
			}
		}
	}

	fn put(&mut self, p: &Piece, r_pos: usize, c_pos: usize, v: i32) -> bool {
		if self.row < r_pos + p.row || self.col < c_pos + p.col {
			return false
		}

		for r in 0..p.row {
			for c in 0..p.col {
				if p.b[r][c] != 0 {
					if self.b[r + r_pos][c + c_pos] != 0 {
						return false;
					}
					self.b[r + r_pos][c + c_pos] = v;
				}
			}
		}

		true
	}

	fn scan(&self, bd: &mut BoardData, r: usize, c: usize) -> usize {
		if bd[r][c] != 0 {
			return 0;
		}

		let mut ret = 1;
		bd[r][c] = 1;

		if r + 1 < self.row {
			ret += self.scan(bd, r + 1, c);
		}
		if c + 1 < self.col {
			ret += self.scan(bd, r, c + 1);
		}
		if r > 0 {
			ret += self.scan(bd, r - 1, c);
		}
		if c > 0 {
			ret += self.scan(bd, r, c - 1);
		}
		return ret;
	}

	fn check_hole(&self) -> bool {
		let mut b_tmp = self.b.clone();

		for r in 0..self.row {
			for c in 0..self.col {
				if b_tmp[r][c] == 0 && 
						self.scan(&mut b_tmp, r, c) % 5 != 0 {
					return true
				}
			}
		}

		false
	}
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Piece {
	b: PieceData,
	row: usize,
	col: usize,
}

impl Piece {
	fn new(x: &PieceData, r: usize, c: usize) -> Self {
		Piece {
			b: *x,
			row: r,
			col: c,
		}
	}

	fn rot(&self) -> Piece {
		let mut x: PieceData = Default::default();
		for r in 0..self.row {
			for c in 0..self.col {
				x[c][self.row - r - 1] = self.b[r][c];
			}
		}
		Piece { b: x, row: self.col, col: self.row }
	}

	fn rev(&self) -> Piece {
		let mut x: PieceData = Default::default();
		for r in 0..self.row {
			for c in 0..self.col {
				let index: usize = self.row - r - 1;
				x[index][c] = self.b[r][c]
			}
		}
		Piece { b: x, row: self.row, col: self.col }
	}
}


fn make_piece_set(p: Piece) -> HashSet<Piece> {
	let mut h = HashSet::new();
	h.insert(p.rot());
	h.insert(p.rot().rot());
	h.insert(p.rot().rot().rot());
	h.insert(p.rev());
	h.insert(p.rev().rot());
	h.insert(p.rev().rot().rot());
	h.insert(p.rev().rot().rot().rot());
	h.insert(p);
	h
}

fn main() {
	let h: Vec<_> = [
		Piece::new(
			&[[1,1,1,1,1],
			  [0,0,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0]], 1, 5),
		Piece::new(
			&[[1,1,1,1,0],
			  [1,0,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0]], 2, 4),
		Piece::new(
			&[[1,1,1,1,0],
			  [0,1,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0]], 2, 4),
		Piece::new(
			&[[0,1,1,1,0],
			  [1,1,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0]], 2, 4),
		Piece::new(
			&[[1,1,1,0,0],
			  [1,0,0,0,0],
			  [1,0,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0]], 3, 3),
		Piece::new(
			&[[1,1,1,0,0],
			  [0,1,0,0,0],
			  [0,1,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0]], 3, 3),
		Piece::new(
			&[[0,0,1,0,0],
			  [1,1,1,0,0],
			  [0,1,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0]], 3, 3),
		Piece::new(
			&[[0,1,0,0,0],
			  [1,1,1,0,0],
			  [0,1,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0]], 3, 3),
		Piece::new(
			&[[1,1,0,0,0],
			  [0,1,1,0,0],
			  [0,0,1,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0]], 3, 3),
		Piece::new(
			&[[1,0,0,0,0],
			  [1,1,1,0,0],
			  [0,0,1,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0]], 3, 3),
		Piece::new(
			&[[1,1,1,0,0],
			  [1,0,1,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0]], 2, 3),
		Piece::new(
			&[[1,1,0,0,0],
			  [1,1,1,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0],
			  [0,0,0,0,0]], 2, 3),
	]
	.iter()
	.map(|x| make_piece_set(*x))
	.collect();

	start(Arc::new(h));
}

fn recursive_put(depth: usize, h: Arc<PieceVec>, b_orig: &mut Board, r_pos: usize, c_pos: usize) {
	let mut b_work: Board = Board::new(&Default::default());

	for p in &h[depth] {
		b_work.copy_from(b_orig);

		if !b_work.put(p, r_pos, c_pos, (depth + 1) as i32) {
			continue;
		}

		if depth + 1 == h.len() {
			b_work.show(&['-', 'I', 'L', 'Y', 'N', 'V', 'T', 'F', 'X', 'W', 'Z', 'U', 'P']);
			continue;
		}

		if b_work.check_hole() {
			continue;
		}

		for r in 0..b_work.row {
			for c in 0..b_work.col {
				recursive_put(depth + 1, h.clone(), &mut b_work, r, c);
			}
		}
	}
}

fn start(h: Arc<PieceVec>) {
	let b : Board = Board::new(&Default::default());
	let rc_list = (0..b.row).into_iter()
		 .map(|r| (0..b.col).into_iter().map(move |c| (r, c)))
		 .flatten()
		 .collect::<Vec<_>>();

	rc_list.par_iter().for_each(|(r, c)| {
		recursive_put(0, h.clone(), &mut Board::new(&Default::default()), *r, *c);
	});
}
