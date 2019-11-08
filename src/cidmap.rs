use std::io::{BufRead, BufReader, Error};

pub struct CIDChar {
	pub utf32: u32,
	pub cid: u32,
}

pub struct CIDRange {
	pub begin_utf32: u32,
	pub end_utf32: u32,
	pub cid: u32,
}

pub enum CIDMapping {
	Pair(CIDChar),
	Range(CIDRange),
}

pub fn make_cid_mapping_list() -> Result<Vec<CIDMapping>, Error> {
	let mut cid_mapping_list: Vec<CIDMapping> = Vec::new();
	let mut cidchar = false;
	let mut cidrange = false;
	let bytes = include_bytes!("UniJIS-UTF32-H");
	for line in BufReader::new(&bytes[..]).lines() {
		let line = line?;
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
				cid_mapping_list.push(CIDMapping::Pair(m));
			}
		}
		if cidrange {
			if let Some(m) = parse_cidrange(line.as_str()) {
				cid_mapping_list.push(CIDMapping::Range(m));
			}
		}
		// --
/*
		let cols: Vec<&str> = line.split('\t').collect();
		if cols.len() != 32 {
			continue;
		}
		let cid = u32::from_str_radix(cols[0], 10).unwrap_or(0);
		if cols[20].len() == 0 {
			continue;
		}
		if cols[20].len() == 1 {
			if let Some(ch) = cols[20].chars().next() {
				if ch == '*' {
					continue;
				}
			}
		}
		let ucs1: Vec<&str> = cols[20].split(',').collect();
		let ucs2: Vec<u32> = ucs1.into_iter().filter_map(|s|
			u32::from_str_radix(s, 16).ok()
		).collect();
		let cid_mapping = CIDMapping {
			cid: cid,
			unicodes: ucs2,
		};
		cid_mapping_list.push(cid_mapping);
*/
	}
	Ok(cid_mapping_list)
}

fn parse_cidchar(line: &str) -> Option<CIDChar> {
	let cols: Vec<&str> = line.split_ascii_whitespace().collect();
	if cols.len() == 2 {
		let col1 = trim_brackets(cols[0]);
		if let Ok(u) = u32::from_str_radix(col1.as_str(), 16) {
			if let Ok(c) = u32::from_str_radix(cols[1], 10) {
				return Some(CIDChar { utf32: u, cid: c });
			}
		}
	}
	None
}

fn parse_cidrange(line: &str) -> Option<CIDRange> {
	let cols: Vec<&str> = line.split_ascii_whitespace().collect();
	if cols.len() == 3 {
		let col1 = trim_brackets(cols[0]);
		let col2 = trim_brackets(cols[1]);
		if let Ok(u1) = u32::from_str_radix(col1.as_str(), 16) {
			if let Ok(u2) = u32::from_str_radix(col2.as_str(), 16) {
				if let Ok(c) = u32::from_str_radix(cols[2], 10) {
					return Some(CIDRange { begin_utf32: u1, end_utf32: u2, cid: c });
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
