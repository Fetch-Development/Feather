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
use std::collections::BTreeMap;
use encoding::all::WINDOWS_1251;
use encoding::{Encoding, DecoderTrap};
use std::io::prelude::*;
use std::io::BufReader;
use progress_bar::progress_bar::ProgressBar;
use progress_bar::color::{Color, Style};
use std::iter::FromIterator;
extern crate colorful;
use colorful::Color as TermColor;
use colorful::Colorful;

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
	let mut colors_collection = [blacks, whites, cyans, blues, reds, yellows, greens, oranges, violets, pinks, turquoises, vinouses, lightgreens, browns,
	lilacs, hazels, bluishes, cherryishes, sparkies];
	const ROOTS: [&str; 20] = ["черн", "бел", "голуб", "син", "красн", "желт", "зелен", "оранжев", "фиолетов", "розов", "бирюзов", "бордов", "салатов",
	"коричнев", "лилов", "сер", "кар", "сиз", "вишнев", "огненн"];
	const TAILS: [&str; 20] = ["ый", "ый", "ой", "ий", "ый", "ый", "ый", "ый", "ый", "ый", "ый", "ый", "ый", "ый", "ый", "ый", "ий", "ый", "ый", "ый"];
	const COMMON: [&str; 201] = ["–", "-", "меня", "мне", "был", "было","были", "была", "ты", "чтобы", "я","он", "она", "они", "оно", "его", "ее", "её",
	 "наш", "их", "мой", "и", "но","не", "на", "в", "под", "над", "из", "от", "что", "это", "все" , "как", "с", "у", "к", "а", "за", "мы", "нас", "по",
	 "когда", "о", "так", "бы", "же", "для", "или", "если", "еще", "со", "с", "очень", "до", "то", "ни", "ему", "чем", "только", "уже", "даже", "во",
	 "этого","тебя","сказал","тебе","после","я,","где","них","этом","этот","него","себя","время","и,","вы","через","надо","нам", "несколько","меня",
	 "мог","которые","им","вместе","без","будет","при","того","раз","ним","один","может","там","быть","всех","мной","больше","здесь","ли","тоже","просто",
	 "свою","тем","тут","ничего","который","этой","затем","об","потому","ней","можно","ответил","стал","том","пока","эти","тот","есть","ко","теперь",
	 "знал","всего","почти","своей","кто","рядом","ей","между","перед","знаю","слишком","моей","нет","себе","вот","нее","свои","хотя","хорошо","два",
	 "такой","много","тогда","всем","этим","более","мои","мое","никогда","хотел","почему","что-то","эту","словно","свой","могу","ними","своих","нами",
	 "также","которых","нем","сейчас","поэтому","будто","лишь","сказала","сейчас","опять","сам","спросил","тому","немного","спросила","куда","однако",
	 "ну", "да", "нет", "вдруг","ведь","уж","вам","вас","совсем","впрочем","тотчас","про","хоть","ж","чтоб","точно","кажется", "какой", "такое", "другой",
	 "которое"];

	let matches = App::new("feather")
	.version("2.1")
	.about("Creates images, demonstrating dominant colors out of e-books.")
	.author("Aydar N.")
		.arg(Arg::with_name("INPUT")
		.help("Sets path to the e-book file")
		.required(true)
		.index(1))
	.arg(Arg::with_name("analyze")
		.short("a")
		.long("analyze")
		.help("Analyzing the text of an e-book"))
	.arg(Arg::with_name("image")
		.short("i")
		.long("image")
		.help("Create a picture, which represent dominant colors mentioned in an e-book"))
	.get_matches();

	
	let val = matches.value_of("INPUT").unwrap();
	println!("{} {}...", "Opening ".color(TermColor::Cyan).bold(), val);
	let file = File::open(val).expect("file not found");
	println!("{}words...", "Counting ".color(TermColor::Cyan).bold());
	let reader = BufReader::new(&file);
	let words_temp = reader.split(b' ').map(|l| l.unwrap());
	let mut words_encoded = Vec::new();
	let mut sentences = 0;
	let mut length_offset = 0;
	let mut longest = "".to_string();
	let mut buf = "".to_string();
	let mut longest_len = 0;
	let mut up_to_append = false;
	for word in words_temp{
		let encoded = WINDOWS_1251.decode(&word, DecoderTrap::Strict).unwrap();
		if encoded.contains(".") || encoded.contains("!") || encoded.contains("?") { if up_to_append{write!(longest, "{}", encoded).unwrap(); longest_len += 1; up_to_append = false} sentences += 1; length_offset = 0; buf = "".to_string()}
		else{length_offset += 1; write!(buf, "{} ", encoded).unwrap(); if length_offset > longest_len { longest = buf.clone(); longest_len += 1; up_to_append = true }}
		words_encoded.push(encoded.to_lowercase().replace(".", "").replace(",", "").replace(":", "").replace("\u{a0}–", "").replace("\u{a0}", " "));
	}
	let words_count = words_encoded.len();
	if words_count < 10000 { panic!("Not enough words to analyze"); }
	let words_in_block = words_count / 64;
	if matches.is_present("image") {
		println!("{}, thus {} words will be analyzed in a single block.", words_count, words_in_block);
		println!("Analyzing blocks of text for color matches...");
		let mut progress_bar = ProgressBar::new(64);
		progress_bar.set_action("Analyzing", Color::Cyan, Style::Bold);
		for i in 1..65 {
			progress_bar.inc();
			for j in 0..ROOTS.len() - 1{
				let mut occurrences = 0;
				for k in i * words_in_block - words_in_block .. i * words_in_block{
					if words_encoded[k].contains(ROOTS[j]){
						if check_declination(ROOTS[j], &words_encoded[k], TAILS[j]) {occurrences = occurrences + 1}
					}
				}
				colors_collection[j][i - 1] = occurrences;
			}
		}
		println!();
		println!("{}orphaned color blocks...", "Eliminating ".color(TermColor::Cyan).bold());

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
		for i in 0..64{
			if orphaned[i] == true{
				orphaned_count = orphaned_count + 1;
			}
		}
		println!("{}", "Done".color(TermColor::Green).bold());
		println!("{}dominant colors for each block...", "Calculating ".color(TermColor::Cyan).bold());
		let mut maxindexes = [0; 64];
		let mut saturations: [u8; 64]=[0; 64];
		for block in 0..64{
			if !orphaned[block]{
				let mut maxval = 0;
				for j in 0..ROOTS.len() - 1{
					if colors_collection[j][block] > maxval {
						maxval = colors_collection[j][block];
						maxindexes[block] = j;
						if maxval >= 3{
							saturations[block] = 3;
						}else{
							saturations[block] = maxval as u8;
						}
						
					}
				}
				//println!("In block {} it's {}, with saturation {}", block, ROOTS[maxindexes[block]], saturations[block]);
			}
		}
		println!("{}", "Done".color(TermColor::Green).bold());
		println!("{}color values for each block...", "Generating ".color(TermColor::Cyan).bold());
		let mut rblocks: [u8; 64] = [50; 64];
		let mut gblocks: [u8; 64] = [50; 64];
		let mut bblocks: [u8; 64] = [50; 64]; 
		for block in 0..64{
			if !orphaned[block]{
				match maxindexes[block] {
					0 => { rblocks[block] = 0; gblocks[block] = 0; bblocks[block] = 0 },
					1 => { rblocks[block] = 255 / 3 * saturations[block]; gblocks[block] = 255 / 3 * saturations[block]; bblocks[block] = 255 / 3 * saturations[block] },
					2 => { rblocks[block] = 66 / 3 * saturations[block]; gblocks[block] = 170 / 3 * saturations[block]; bblocks[block] = 255 / 3 * saturations[block] },
					3 => { rblocks[block] = 0; gblocks[block] = 0; bblocks[block] = 255 / 3 * saturations[block] },
					4 => { rblocks[block] = 255 / 3 * saturations[block]; gblocks[block] = 0; bblocks[block] = 0 },
					5 => { rblocks[block] = 255 / 3 * saturations[block]; gblocks[block] = 255 / 3 * saturations[block]; bblocks[block] = 0 },
					6 => { rblocks[block] = 0; gblocks[block] = 255 / 3 * saturations[block]; bblocks[block] = 0 },
					7 => { rblocks[block] = 255 / 3 * saturations[block]; gblocks[block] = 165 / 3 * saturations[block]; bblocks[block] = 0 },
					8 => { rblocks[block] = 139 / 3 * saturations[block]; gblocks[block] = 0; bblocks[block] = 255 / 3 * saturations[block] },
					9 => { rblocks[block] = 255 / 3 * saturations[block]; gblocks[block] = 192 / 3 * saturations[block]; bblocks[block] = 203 / 3 * saturations[block] },
					10 => { rblocks[block] = 48 / 3 * saturations[block]; gblocks[block] = 213 / 3 * saturations[block]; bblocks[block] = 200 / 3 * saturations[block] },
					11 => { rblocks[block] = 155 / 3 * saturations[block]; gblocks[block] = 45 / 3 * saturations[block]; bblocks[block] = 48 / 3 * saturations[block] },
					12 => { rblocks[block] = 153 / 3 * saturations[block]; gblocks[block] = 255 / 3 * saturations[block]; bblocks[block] = 153 / 3 * saturations[block] },
					13 | 16 => { rblocks[block] = 150 / 3 * saturations[block]; gblocks[block] = 75 / 3 * saturations[block]; bblocks[block] = 0 },
					15 => { rblocks[block] = 100 / 3 * saturations[block]; gblocks[block] = 100 / 3 * saturations[block]; bblocks[block] = 100 / 3 * saturations[block] },
					14 => { rblocks[block] = 219 / 3 * saturations[block]; gblocks[block] = 112/ 3 * saturations[block]; bblocks[block] = 147 / 3 * saturations[block] },
					17 => { rblocks[block] = 121 / 3 * saturations[block]; gblocks[block] = 160 / 3 * saturations[block]; bblocks[block] = 193 / 3 * saturations[block] },
					18 => { rblocks[block] = 145 / 3 * saturations[block]; gblocks[block] = 30 / 3 * saturations[block]; bblocks[block] = 66 / 3 * saturations[block] },
					19 => { rblocks[block] = 226 / 3 * saturations[block]; gblocks[block] = 88 / 3 * saturations[block]; bblocks[block] = 34 / 3 * saturations[block] },
					_ => (),
				}
			}
		}

		println!("{}", "Done".color(TermColor::Green).bold());
		println!("{}image...", "Generating ".color(TermColor::Cyan).bold());
		blocks::set_blocks(rblocks, gblocks, bblocks);
		println!("{}", "Done".color(TermColor::Green).bold());
	} else if matches.is_present("analyze"){
		println!("{} in total", words_count);
		println!("Total sentences: {}, the longest seems to be this:", sentences);
		println!("{:?}", longest);
		println!("Looking for most common used words...");
		let mut progress_bar = ProgressBar::new(words_encoded.len());
		progress_bar.set_action("Analyzing", Color::Cyan, Style::Bold);
		let mut counts = BTreeMap::new();
		for word in words_encoded{
			progress_bar.inc();
			*counts.entry(word).or_insert(0) += 1;
		}
		println!();
		for i in 0..COMMON.len() {
			counts.remove(COMMON[i]);
		}
		let mut v = Vec::from_iter(counts.clone());
		println!("Top-15 most used words:");
		v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

		let mut offset = 0;
		for i in 0..15 + offset{
			if v[i].0.len() > 2 { println!("{}. {:?} ({})",i + 1 - offset, v[i].0, v[i].1)}
			else {offset += 1}
			//print!("{:?},", v[i].0);
		}
	}
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
