/***************************
COPYRIGHT FETCH DEVELOPMENT,
2020

LESTERRRY AUTHORSHIP
***************************/

extern crate encoding;
extern crate image;
use clap::App;
use clap::Arg;
use std::fs::File;
use std::fmt::Write;
use encoding::all::WINDOWS_1251;
use encoding::{Encoding, DecoderTrap};
use std::io::prelude::*;
use std::io::BufReader;
use progress_bar::progress_bar::ProgressBar;
use progress_bar::color::{Color, Style};

mod blocks;

fn main(){
	let blacks: [i32;64] = [0;64];
	let whites: [i32;64] = [0;64];
	let cyans: [i32;64] = [0;64];
	let blues: [i32;64] = [0;64];
	let reds: [i32;64] = [0;64];
	let yellows: [i32;64] = [0;64];
	let greens: [i32;64] = [0;64];
	let oranges: [i32;64] = [0;64];
	let violets: [i32;64] = [0;64];
	let pinks: [i32;64] = [0;64];
	let turquoises: [i32;64] = [0;64];
	let vinouses: [i32;64] = [0;64];
	let lightgreens: [i32;64] = [0;64];
	let browns: [i32;64] = [0;64];
	let lilacs: [i32;64] = [0;64];
	let hazels: [i32;64] = [0;64];
	let bluishes: [i32;64] = [0;64];
	let cherryishes: [i32;64] = [0;64];
	let sparkies: [i32;64] = [0;64];
	let mut colors_collection = [blacks, whites, cyans, blues, reds, yellows, greens, oranges, violets, pinks, turquoises, vinouses, lightgreens, browns, lilacs, hazels, bluishes, cherryishes, sparkies];
	const ROOTS: [&str; 20] = ["черн", "бел", "голуб", "син", "красн", "желт", "зелен", "оранжев", "фиолетов", "розов", "бирюзов", "бордов", "салатов", "коричнев", "лилов", "сер", "кар", "сиз", "вишнев", "огненн"];
	const TAILS: [&str; 20] = ["ый", "ый", "ой", "ий", "ый", "ый", "ый", "ый", "ый", "ый", "ый", "ый", "ый", "ый", "ый", "ый", "ий", "ый", "ый", "ый"];

	let matches = App::new("feather")
	.version("1.0")
	.about("Creates images, demonstrating dominant colors out of e-books.")
	.author("Aydar N.")
		.arg(Arg::with_name("INPUT")
		.help("Sets path to the e-book file")
		.required(true)
		.index(1))
	.get_matches();

	let val = matches.value_of("INPUT").unwrap();
	println!("Opening {}...", val);
	let file = File::open(val).expect("file not found");
	println!("Counting words...");
	let reader = BufReader::new(&file);
	let words_temp = reader.split(b' ').map(|l| l.unwrap());
	let mut words_encoded = Vec::new();
	for word in words_temp{
		let encoded = WINDOWS_1251.decode(&word, DecoderTrap::Strict).unwrap();
		words_encoded.push(encoded);
	}
	let words_count = words_encoded.len();
	if words_count < 10000 { panic!("Not enough words to create a visualization"); }
	let words_in_block = words_count / 64;
	println!("{}, thus {} words will be analyzed in a single block.", words_count, words_in_block);
	println!("Analyzing blocks of text for color matches...");
	let mut progress_bar = ProgressBar::new(64);
	progress_bar.set_action("Analyzing", Color::Cyan, Style::Bold);
	progress_bar.inc();
	for i in 1..64 {
		progress_bar.inc();
		for j in 0..ROOTS.len() - 1{
			let mut occurrences = 0;
			for k in i * words_in_block - words_in_block .. i * words_in_block{
				if words_encoded[k].contains(ROOTS[j]){
					if check_declination(ROOTS[j], &words_encoded[k], TAILS[j]) {occurrences = occurrences + 1}
				}
			}
			colors_collection[j][i] = occurrences;
		}
	}
	println!();
	println!("Eliminating orphaned color blocks...");

	let mut orphaned : [bool; 64] = [true; 64];
	for block in 0..64{
		for j in 0..ROOTS.len() - 1{
			if colors_collection[j][block] != 0 {
				orphaned[block] = false;
				break;
			}
		}
	}

	let mut orphaned_count = 0;
	for i in 0..63{
		if orphaned[i] == true{
			orphaned_count = orphaned_count + 1;
		}
	}

	println!("{} eliminated", orphaned_count);
	println!("Calculating dominant colors for each block...");
	let mut maxindexes = [0; 64];
	for block in 0..64{
		if !orphaned[block]{
			let mut maxval = 0;
			for j in 0..ROOTS.len() - 1{
				if colors_collection[j][block] > maxval {maxval = colors_collection[j][block]; maxindexes[block] = j;}
			}
		}
	}
	println!("Done");
	println!("Generating color values for each block...");
	let mut rblocks: [u8; 64] = [50; 64];
	let mut gblocks: [u8; 64] = [50; 64];
	let mut bblocks: [u8; 64] = [50; 64]; 
	for block in 0..64{
		if !orphaned[block]{
			match maxindexes[block] {
				0 => { rblocks[block] = 0; gblocks[block] = 0; bblocks[block] = 0 },
				1 => { rblocks[block] = 255; gblocks[block] = 255; bblocks[block] = 255 },
				2 => { rblocks[block] = 66; gblocks[block] = 170; bblocks[block] = 255 },
				3 => { rblocks[block] = 0; gblocks[block] = 0; bblocks[block] = 255 },
				4 => { rblocks[block] = 255; gblocks[block] = 0; bblocks[block] = 0 },
				5 => { rblocks[block] = 255; gblocks[block] = 255; bblocks[block] = 0 },
				6 => { rblocks[block] = 0; gblocks[block] = 255; bblocks[block] = 0 },
				7 => { rblocks[block] = 255; gblocks[block] = 165; bblocks[block] = 0 },
				8 => { rblocks[block] = 139; gblocks[block] = 0; bblocks[block] = 255 },
				9 => { rblocks[block] = 255; gblocks[block] = 192; bblocks[block] = 203 },
				10 => { rblocks[block] = 48; gblocks[block] = 213; bblocks[block] = 200 },
				11 => { rblocks[block] = 155; gblocks[block] = 45; bblocks[block] = 48 },
				12 => { rblocks[block] = 153; gblocks[block] = 255; bblocks[block] = 153 },
				13 | 16 => { rblocks[block] = 150; gblocks[block] = 75; bblocks[block] = 0 },
				15 => { rblocks[block] = 100; gblocks[block] = 100; bblocks[block] = 100 },
				14 => { rblocks[block] = 219; gblocks[block] = 112; bblocks[block] = 147 },
				17 => { rblocks[block] = 121; gblocks[block] = 160; bblocks[block] = 193 },
				18 => { rblocks[block] = 145; gblocks[block] = 30; bblocks[block] = 66 },
				19 => { rblocks[block] = 226; gblocks[block] = 88; bblocks[block] = 34 },
				_ => (),
			}
		}
	}

	println!("Done");
	println!("Generating image...");
	blocks::set_blocks(rblocks, gblocks, bblocks);
	println!("Done");
}
fn check_declination(root:&str, marker:&str, tail:&str) -> bool{
	//Checking singular
	if tail == "ий"{
		let mut nominative = root.to_string();
		write!(&mut nominative, "ий").unwrap();
		if nominative == marker { return true; }
		let mut genitive= root.to_string();
		write!(&mut genitive, "его").unwrap();
		if genitive == marker { return true; }
		let mut dative = root.to_string();
		write!(&mut dative, "ему").unwrap();
		if dative == marker { return true; }
		let mut ablative = root.to_string();
		write!(&mut ablative, "им").unwrap();
		if ablative == marker { return true; }
		let mut accusative = root.to_string();
		write!(&mut accusative, "юю").unwrap();
		if accusative == marker { return true; }
		let mut prepositional = root.to_string();
		write!(&mut prepositional, "ем").unwrap();
		if prepositional == marker { return true; }
		let mut pet_nominative = root.to_string();
		write!(&mut pet_nominative, "енький").unwrap();
		if pet_nominative == marker { return true; }
		let mut pet_genitive= root.to_string();
		write!(&mut pet_genitive, "енького").unwrap();
		if pet_nominative == marker { return true;}
		let mut pet_dative = root.to_string();
		write!(&mut pet_dative, "енькому").unwrap();
		if pet_dative == marker { return true;}
		let mut pet_ablative = root.to_string();
		write!(&mut pet_ablative, "еньким").unwrap();
		if pet_ablative == marker { return true;}
		let mut pet_accusative = root.to_string();
		write!(&mut pet_accusative, "енькую").unwrap();
		if pet_accusative == marker { return true; }
		let mut pet_prepositional = root.to_string();
		write!(&mut pet_prepositional, "еньком").unwrap();
		if pet_prepositional == marker { return true;}
	}else{
		if tail == "ый"{
			let mut nominative = root.to_string();
			write!(&mut nominative, "ый").unwrap();
			if nominative == marker { return true; }
		}
		else if tail == "ой"{
			let mut nominative = root.to_string();
			write!(&mut nominative, "ой").unwrap();
			if nominative == marker { return true; }
		}else{
			panic!("Unknown tail passed");
		}
		let mut genitive= root.to_string();
		write!(&mut genitive, "ого").unwrap();
		if genitive == marker { return true; }
		let mut dative = root.to_string();
		write!(&mut dative, "ому").unwrap();
		if dative == marker { return true; }
		let mut ablative = root.to_string();
		write!(&mut ablative, "ым").unwrap();
		if ablative == marker { return true; }
		let mut accusative = root.to_string();
		write!(&mut accusative, "ую").unwrap();
		if accusative == marker { return true; }
		let mut prepositional = root.to_string();
		write!(&mut prepositional, "ом").unwrap();
		if prepositional == marker { return true; }
		let mut pet_nominative = root.to_string();
		write!(&mut pet_nominative, "енький").unwrap();
		if pet_nominative == marker { return true; }
		let mut pet_genitive= root.to_string();
		write!(&mut pet_genitive, "енького").unwrap();
		if pet_nominative == marker { return true;}
		let mut pet_dative = root.to_string();
		write!(&mut pet_dative, "енькому").unwrap();
		if pet_dative == marker { return true;}
		let mut pet_ablative = root.to_string();
		write!(&mut pet_ablative, "еньким").unwrap();
		if pet_ablative == marker { return true;}
		let mut pet_accusative = root.to_string();
		write!(&mut pet_accusative, "енькую").unwrap();
		if pet_accusative == marker { return true; }
		let mut pet_prepositional = root.to_string();
		write!(&mut pet_prepositional, "еньком").unwrap();
		if pet_prepositional == marker { return true;}
	}

	//Checking plular
	if tail == "ий"{
		let mut nominative = root.to_string();
		write!(&mut nominative, "ие").unwrap();
		if nominative == marker { return true; }
		let mut genitive= root.to_string();
		write!(&mut genitive, "их").unwrap();
		if genitive == marker { return true; }
		let mut dative = root.to_string();
		write!(&mut dative, "им").unwrap();
		if dative == marker { return true; }
		let mut ablative = root.to_string();
		write!(&mut ablative, "ими").unwrap();
		if ablative == marker { return true; }
		
	}else{
		let mut nominative = root.to_string();
		write!(&mut nominative, "ые").unwrap();
		if nominative == marker { return true; }
		let mut genitive= root.to_string();
		write!(&mut genitive, "ых").unwrap();
		if genitive == marker { return true; }
		let mut dative = root.to_string();
		write!(&mut dative, "ым").unwrap();
		if dative == marker { return true; }
		let mut ablative = root.to_string();
		write!(&mut ablative, "ыми").unwrap();
		if ablative == marker { return true; }
	}
	let mut pet_nominative = root.to_string();
	write!(&mut pet_nominative, "енькие").unwrap();
	if pet_nominative == marker { return true; }
	let mut pet_genitive= root.to_string();
	write!(&mut pet_genitive, "еньких").unwrap();
	if pet_nominative == marker { return true;}
	let mut pet_dative = root.to_string();
	write!(&mut pet_dative, "еньким").unwrap();
	if pet_dative == marker { return true;}
	let mut pet_ablative = root.to_string();
	write!(&mut pet_ablative, "енькими").unwrap();
	if pet_ablative == marker { return true;}

	return false;
}
