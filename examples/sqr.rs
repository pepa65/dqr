use std::path::PathBuf;
use std::time::Instant;

use sqr::*;

#[derive(Debug, Clone)]
struct ResultInfo {
	file_count: usize,
	id_count: usize,
	decode_count: usize,
	load_time: u128,
	identify_time: u128,
	total_time: u128,
}

#[derive(Clone)]
struct Opts {
	verbose: bool,
	cell_dump: bool,
}

fn print_result(name: &str, info: &mut ResultInfo) {
	print!("-------------------------------------------------------------------------------");
	print!(
		"{}: {} files, {} codes, {} decoded, ({} failures)",
		name,
		info.file_count,
		info.id_count,
		info.decode_count,
		info.id_count - info.decode_count,
	);
	if info.id_count != 0 {
		print!(", {}% success rate", (info.decode_count * 100 + info.id_count / 2) / info.id_count,);
	}
	println!();
	println!("Total time: load:{} identify:{} total:{}", info.load_time, info.identify_time, info.total_time,);
	if info.file_count != 0 {
		println!(
			"Average time: load:{} identify:{} total:{}",
			info.load_time.wrapping_div(info.file_count as u128),
			info.identify_time.wrapping_div(info.file_count as u128),
			info.total_time.wrapping_div(info.file_count as u128),
		);
	}
}

fn add_result(sum: &mut ResultInfo, inf: &mut ResultInfo) {
	sum.file_count += inf.file_count;
	sum.id_count += inf.id_count;
	sum.decode_count += inf.decode_count;
	sum.load_time = sum.load_time.wrapping_add(inf.load_time);
	sum.identify_time = sum.identify_time.wrapping_add(inf.identify_time);
	sum.total_time = sum.total_time.wrapping_add(inf.total_time);
}

fn scan_file(decoder: &mut Quirc, opts: &Opts, path: &str, info: &mut ResultInfo) -> i32 {
	let path = PathBuf::from(path);
	let start = Instant::now();
	let total_start = start;

	let img = match image::open(&path) {
		Ok(im) => im.into_luma8(),
		Err(e) => {
			println!("# {}: {e}, no image", path.display());
			return 42;
		},
	};

	info.load_time = start.elapsed().as_millis();

	let start = Instant::now();

	let res: Vec<_> = decoder.identify(img.width() as usize, img.height() as usize, &img).collect::<Result<_, _>>().unwrap();

	info.identify_time = start.elapsed().as_millis();
	info.id_count = decoder.count();

	for code in &res {
		if code.decode().is_ok() {
			info.decode_count += 1
		}
	}

	info.total_time = total_start.elapsed().as_millis();
	if opts.verbose {
		//        println!("  {:<30}  {:<5} {:<5} {:<5} {:<5} {:<5}",
		println!(
			"= {} (found: {}  decoded: {})",
			path.file_name().unwrap().to_string_lossy(),
			//            info.load_time,
			//            info.identify_time,
			//            info.total_time,
			info.id_count,
			info.decode_count,
		);
	};
	for code in &res {
		if opts.cell_dump {
			dump_cells(code);
			println!();
		};
		match code.decode() {
			Ok(data) => {
				if opts.verbose {
					println!(
						"- Version: {}  ECC: {:?}  Mask: {}  Type: {:?}  Len: {}  ECI: {:?}  Data:",
						data.version,
						data.ecc_level,
						data.mask,
						data.data_type.unwrap(),
						data.payload.len(),
						data.eci
					);
				};
				println!("{}", std::str::from_utf8(&data.payload).unwrap());
			}
			Err(err) => {
				println!("# ERROR: {err}");
			}
		};
	}

	info.file_count = 1;
	1
}

fn run_tests(opts: &Opts, args: &[String]) -> i32 {
	let mut sum = ResultInfo { file_count: 0, id_count: 0, decode_count: 0, load_time: 0, identify_time: 0, total_time: 0 };
	let mut count: i32 = 0;
	let mut decoder = Quirc::new();
	for path in args {
		let mut info: ResultInfo = ResultInfo { file_count: 0, id_count: 0, decode_count: 0, load_time: 0, identify_time: 0, total_time: 0 };
		if scan_file(&mut decoder, opts, path, &mut info) > 0 {
			add_result(&mut sum, &mut info);
			count += 1
		}
	}
	if count > 1 && opts.verbose {
		print_result("\nTOTAL", &mut sum);
	}

	0
}

fn dump_cells(code: &Code) {
	let code = *code;
	print!("- {} cells, corners:", code.size);
	for u in 0..4 {
		print!(" ({},{})", code.corners[u].x, code.corners[u].y);
	}
	println!();
	for _ in 0..code.size + 2 {
		print!("██");
	}
	println!();
	for v in 0..code.size {
		print!("██");
		for u in 0..code.size {
			let p = v * code.size + u;
			if (code.cell_bitmap[(p >> 3) as usize] & (1 << (p & 7))) != 0 {
				print!("  ");
			} else {
				print!("██");
			}
		}
		println!("██");
	}
	for _ in 0..code.size + 2 {
		print!("██");
	}
}

fn help() {
	println!("sqr {}\nUsage:  sqr [-h|--help] | [-v|--verbose] [-d|--dump] <image>...", version());
	println!("    -h/--help       Show this help text");
	println!("    -v/--verbose    Show processing information");
	println!("    -d/--dump       Dump each identified QR code to the terminal");
	std::process::exit(0);
}

fn main() {
	let mut opts = Opts { verbose: false, cell_dump: false };
	let args: Vec<String> = std::env::args()
		.skip(1)
		.filter(|e| {
			if e == "-v" || e == "--verbose" {
				opts.verbose = true;
				false
			} else {
				true
			}
		})
		.filter(|e| {
			if e == "-d" || e == "--dump" {
				opts.cell_dump = true;
				false
			} else {
				true
			}
		})
		.filter(|e| {
			if e == "-h" || e == "--help" {
				help();
				false
			} else {
				true
			}
		})
		.collect();
	if opts.verbose {
		println!("sqr {} with Quircs library", version());
	};
	let res = run_tests(&opts, &args);
	std::process::exit(res);
}
