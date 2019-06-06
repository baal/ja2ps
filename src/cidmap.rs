use std::io::{BufRead, BufReader, Error};

pub struct CIDMapping {
	pub cid: u32,
	pub unicodes: Vec<u32>,
}

pub fn make_cid_mapping_list() -> Result<Vec<CIDMapping>, Error> {
	let mut cid_mapping_list: Vec<CIDMapping> = Vec::new();
	let bytes = include_bytes!("cid2code.txt");
	for line in BufReader::new(&bytes[..]).lines() {
		let line = line?;
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
	}
	Ok(cid_mapping_list)
}
