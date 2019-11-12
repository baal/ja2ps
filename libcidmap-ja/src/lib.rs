use std::io::{ BufRead, BufReader };

struct CidChar {
	pub utf32: u32,
	pub cid: u32,
}

struct CidRange {
	pub begin_utf32: u32,
	pub end_utf32: u32,
	pub cid: u32,
}

enum CidMapping {
	Pair(CidChar),
	Range(CidRange),
}

pub struct Cidmap {
	vec: Vec<CidMapping>,
}

impl Cidmap {
	pub fn new() -> Cidmap {
		let mut vec: Vec<CidMapping> = Vec::new();
		let mut cidchar = false;
		let mut cidrange = false;
		let bytes = include_bytes!("UniJIS-UTF32-H");
		for line in BufReader::new(&bytes[..]).lines() {
			//let line = line?;
			let line = match line {
				Ok(l) => l,
				Err(_) => {
					continue;
				},
			};
			if line.contains("begincidchar") {
				cidchar = true;
				continue;
			}
			if line.contains("endcidchar") {
				cidchar = false;
				continue;
			}
			if line.contains("begincidrange") {
				cidrange = true;
				continue;
			}
			if line.contains("endcidrange") {
				cidrange = false;
				continue;
			}
			if cidchar {
				if let Some(m) = parse_cidchar(line.as_str()) {
					vec.push(CidMapping::Pair(m));
				}
			}
			if cidrange {
				if let Some(m) = parse_cidrange(line.as_str()) {
					vec.push(CidMapping::Range(m));
				}
			}
		}
		Cidmap { vec: vec }
	}
	pub fn get_cid(&self, c: char) -> u32 {
		let u = c as u32;
		for m in &self.vec {
			match m {
				CidMapping::Pair(p) => if p.utf32 == u {
					return p.cid;
				},
				CidMapping::Range(r) => if r.begin_utf32 <= u && u <= r.end_utf32 {
					return r.cid + (u - r.begin_utf32);
				},
			}
		}
		return 0
	}
}

fn parse_cidchar(line: &str) -> Option<CidChar> {
	let cols: Vec<&str> = line.split_ascii_whitespace().collect();
	if cols.len() == 2 {
		let col1 = trim_brackets(cols[0]);
		if let Ok(u) = u32::from_str_radix(col1.as_str(), 16) {
			if let Ok(c) = u32::from_str_radix(cols[1], 10) {
				return Some(CidChar { utf32: u, cid: c });
			}
		}
	}
	None
}

fn parse_cidrange(line: &str) -> Option<CidRange> {
	let cols: Vec<&str> = line.split_ascii_whitespace().collect();
	if cols.len() == 3 {
		let col1 = trim_brackets(cols[0]);
		let col2 = trim_brackets(cols[1]);
		if let Ok(u1) = u32::from_str_radix(col1.as_str(), 16) {
			if let Ok(u2) = u32::from_str_radix(col2.as_str(), 16) {
				if let Ok(c) = u32::from_str_radix(cols[2], 10) {
					return Some(CidRange { begin_utf32: u1, end_utf32: u2, cid: c });
				}
			}
		}
	}
	None
}

fn trim_brackets(s: &str) -> String {
	let cs: Vec<char> = s.chars().collect();
	let fc: char = *cs.first().unwrap_or(&'\0');
	let lc: char = *cs.last().unwrap_or(&'\0');
	if fc == '<' && lc == '>' {
		cs[1..cs.len()-1].into_iter().collect()
	} else {
		String::from(s)
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
