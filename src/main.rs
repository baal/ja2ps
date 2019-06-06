use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod font;
use font::FontMetrics;

mod cidmap;
use cidmap::CIDMapping;
use cidmap::make_cid_mapping_list;

// A4 595x842(pt) 72dpi
const PAGE_WIDTH: u32 = 595;
const PAGE_HEIGHT: u32 = 842;

const MARGIN_TOP: u32 = 60;
const MARGIN_RIGHT: u32 = 40;
const MARGIN_BOTTOM: u32 = 40;
const MARGIN_LEFT: u32 = 40;

const PADDING_TOP: u32 = 20;
const PADDING_RIGHT: u32 = 20;
const PADDING_BOTTOM: u32 = 20;
const PADDING_LEFT: u32 = 20;

static mut CID_MAPPING_LIST: Option<Vec<CIDMapping>> = None;

fn escape_ps_string(s: &str) -> String {

	let cid_mapping_list = unsafe {
		CID_MAPPING_LIST.as_ref().unwrap()
	};

	let mut ret = String::new();
	for ch in s.chars() {
		if ch == '\\' || ch == '(' || ch == ')' {
			ret.push('\\');
			ret.push(ch);
		} else if ch.is_control() {
			ret.push_str("??");
		} else {
			if let Some(cm) = cid_mapping_list.into_iter().find(|x| x.unicodes.iter().any(|&u| u == ch as u32)) {
				if cm.cid <= 0xFFFF {
					ret.push_str(&format!("\\{:03o}", (cm.cid >> 8) & 0xFF));
					ret.push_str(&format!("\\{:03o}", cm.cid & 0xFF));
				} else {
					ret.push_str("??");
				}
			} else {
				ret.push_str("??");
			}
		}
	}
	ret
}

fn ja2ps(in_file_path: Option<String>, out_file_path: Option<String>) -> Result<(), io::Error> {

	let stdin = io::stdin();
	let r: io::BufReader<Box<Read>> = io::BufReader::new(
		match in_file_path {
			Some(path) => Box::new(File::open(path)?),
			None => Box::new(stdin.lock()),
		}
	);

	let stdout = io::stdout();
	let mut w: io::BufWriter<Box<Write>> = io::BufWriter::new(
		match out_file_path {
			Some(path) => Box::new(File::create(path)?),
			None => Box::new(stdout.lock()),
		}
	);

	w.write(b"%!PS\n")?;
	w.write(b"gsave\n")?;

	let font = FontMetrics {
		size: 12,
		internal_leading: 2,
		ascent: 8,
		descent: 4,
		external_leading: 2,
	};

	let content_width = PAGE_WIDTH - MARGIN_LEFT - MARGIN_RIGHT - PADDING_LEFT - PADDING_RIGHT;
	let content_height = PAGE_HEIGHT - MARGIN_TOP - MARGIN_BOTTOM - PADDING_TOP - PADDING_BOTTOM;
	let content_top = MARGIN_BOTTOM + PADDING_BOTTOM + content_height;
	let max_chars = content_width / font.width();
	let max_rows = content_height / font.row_height();

	let x = MARGIN_RIGHT + PADDING_RIGHT;
	let mut y = content_top - font.internal_leading - font.ascent;
	let mut page = 0;
	let mut rows = 0;
	let mut width = 0;
	let mut buf = String::new();
	let mut bufs: Vec<String> = Vec::new();
	let mut newpage = true;

	for line in r.lines() {
		let line = line?;
		buf.clear();
		bufs.clear();
		for ch in line.chars() {
			let cw = if ch.is_ascii() { 1 } else { 2 };
			if width + cw <= max_chars {
				buf.push(ch);
				width += cw;
			} else {
				bufs.push(buf.clone());
				buf.clear();
				buf.push(ch);
				width = cw;
			}
		}
		if !buf.is_empty() {
			bufs.push(buf.clone());
			width = 0;
		}
		for buf in &bufs {
			if newpage {
				if rows != 0 {
					w.write(b"showpage\n")?;
					rows = 0;
					page += 1;
					y = content_top - font.internal_leading - font.ascent;
				}
				let border_top = MARGIN_BOTTOM + PADDING_BOTTOM + content_height + PADDING_TOP;
				let border_right = MARGIN_LEFT + PADDING_LEFT + content_width + PADDING_RIGHT;
				let border_bottom = MARGIN_BOTTOM;
				let border_left = MARGIN_LEFT;
				w.write(b"0.75 setlinewidth\n")?;
				w.write(format!("{x} {y} moveto\n", x = border_left, y = border_top).as_bytes())?;
				w.write(format!("{x} {y} lineto\n", x = border_right, y = border_top).as_bytes())?;
				w.write(format!("{x} {y} lineto\n", x = border_right, y = border_bottom).as_bytes())?;
				w.write(format!("{x} {y} lineto\n", x = border_left, y = border_bottom).as_bytes())?;
				w.write(b"closepath stroke\n")?;
				w.write(b"/Times-Bold findfont 10 scalefont setfont\n")?;
				w.write(format!("{x} {y} moveto (Page: {page}) show\n", x = 500, y = 800, page = page + 1).as_bytes())?;
				w.write(format!("/GothicBBB-Medium-Identity-H findfont {font_size} scalefont setfont\n", font_size = font.size).as_bytes())?;
				newpage = false;
			}
			w.write(format!("{x} {y} moveto ", x = x, y = y).as_bytes())?;
			w.write(b" (")?;
			w.write(escape_ps_string(&buf).as_bytes())?;
			w.write(b") show\n")?;
			rows += 1;
			if rows >= max_rows {
				newpage = true;
			} else {
				y -= font.row_height();
			}
		}
	}

	w.write(b"showpage\n")?;
	w.write(b"grestore\n")?;

	Ok(())
}

fn main() -> Result<(), io::Error> {

	let cid_mapping_list = make_cid_mapping_list()?;
	unsafe {
		CID_MAPPING_LIST = Some(cid_mapping_list);
	}

	let mut args = env::args().skip(1);
	let in_file_path = args.next();
	let out_file_path = args.next();
	ja2ps(in_file_path, out_file_path)?;

	Ok(())
}
