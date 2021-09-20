use std::io;
use std::io::Stdin;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;

const SLABINFO_HEADER: &str = "slabinfo - version: 2.1\n";

/*# name
1: <active_objs>
2: <num_objs>
3: <objsize>
4: <objperslab>
5: <pagesperslab>
6:
7: tunables
8: <limit>
9: <batchcount>
10: <sharedfactor>
11:
12: slabdata
13: <active_slabs>
14: <num_slabs>
15: <sharedavail>
*/

#[derive(Debug, PartialEq)]
struct Slabinfo {
	name: String,
	active_objs:  i32,
	num_objs:     i32,
	objsize:      i32,
	objperslab:   i32,
	pagesperslab: i32,
	active_slabs: i32,
	num_slabs:    i32,
	total_size:   i32,
}

impl Slabinfo {
	fn new(a: &Vec<&str>) -> Result<Slabinfo, ParseIntError> {
		Ok(Slabinfo {
			name:         a[0].to_string(),
			active_objs:  i32::from_str(a[1])?,
			num_objs:     i32::from_str(a[2])?,
			objsize:      i32::from_str(a[3])?,
			objperslab:   i32::from_str(a[4])?,
			pagesperslab: i32::from_str(a[5])?,
			active_slabs: i32::from_str(a[13])?,
			num_slabs:    i32::from_str(a[14])?,
			total_size:   4096 * i32::from_str(a[5])? * i32::from_str(a[13])?,
		})
	}

	fn minus(&self, s: &Self) -> Option<Self> {
		if self.name == s.name {
			Some(
				Slabinfo {
					name: self.name.clone(),
					active_objs:  self.active_objs - s.active_objs,
					num_objs:     self.num_objs - s.num_objs,
					objsize:      self.objsize,
					objperslab:   self.objperslab,
					pagesperslab: self.pagesperslab,
					active_slabs: self.active_slabs - s.active_slabs,
					num_slabs:    self.num_slabs - s.num_slabs,
					total_size:   self.total_size - s.total_size,
				}
			)
		} else {
			None
		}
	}
}

#[derive(PartialEq)]
enum InfoState {
	Start, HeaderFound, End
}

struct Info {
	stdin: Stdin,
	state: InfoState
}

impl Info {
	fn new() -> Self {
		Info { stdin: io::stdin(), state: InfoState::Start }
	}
}

impl Iterator for Info {
	type Item = HashMap::<String, Slabinfo>;
	fn next(&mut self) -> Option<Self::Item> {

		if self.state == InfoState::End {
			return None;
		}

		let mut line = String::new();
		let mut h = Self::Item::new();

		loop {
			line.clear();
			let s = match self.stdin.read_line(&mut line) {
				Ok(s)  => s,
				Err(_) => break
			};

			if s == 0 {
				self.state = InfoState::End;
				break;
			}

			match self.state {
				InfoState::Start => {
						if line == SLABINFO_HEADER {
							self.state = InfoState::HeaderFound;
						}
					},
				InfoState::HeaderFound => {
						if line == SLABINFO_HEADER {
							break
						}

						let a = line
							.split_whitespace()
							.collect::<Vec<_>>();

						if a[0] == "#" {
							continue;
						}

						if a.len() != 16 {
							self.state = InfoState::Start;
							break;
						}

						if let Ok(s) = Slabinfo::new(&a) {
							h.insert(s.name.clone(), s);
						}
					},
				_ => panic!("internal error")
			}
		}

		Some(h)
	}
}

fn main() {
	let v = Info::new().collect::<Vec<_>>();

	let keys = &v[0]
		.keys()
		.collect::<Vec<_>>();

	for (current, prev) in v[1..].iter().zip(v.iter()) {
		let mut tmp = keys
			.iter()
			.map(|k| (k, current.get(*k).zip(prev.get(*k))
						.map(|(c,p)| c.minus(p)) // current minus prev
						.flatten()))
			.filter(|(_,o)| *o != None)
			.map(|(k,o)| (k, o.unwrap()))
			.filter(|(_,s)| s.total_size != 0)
			.collect::<Vec<_>>();
		
		tmp.sort_by(|(_,s1), (_,s2)| s2.total_size
		   .partial_cmp(&s1.total_size).unwrap());

		println!("-------------------");
		println!("{:20} {:>13} {:>13} {:>12} {:>12} {:>12} {:>12}",
			"#name",
			"*active_objs",
			"*active_slabs",
			"objsize",
			"objperslab",
			"pagesperslab",
			"*total_size"
		);
		for e in tmp {
			let s = e.1;
			println!("{:20} {:13} {:13} {:>12} {:>12} {:12} {:>12}", 
				e.0,
				s.active_objs,
				s.active_slabs,
				s.objsize,
				s.objperslab,
				s.pagesperslab,
				s.total_size,
			);
		}
	}
}
