pub fn add_tone_marks(latin: &str, tones: &str) -> String {
	let tones = tones.chars().collect::<Vec<char>>();
	let mut result = String::new();

	for (i, str) in latin.split(' ').enumerate() {
		let tone = tones.get(i).cloned().unwrap_or('0');
		result.push_str(&add_tone_mark(str, tone));
	}
	return result;
}

fn add_tone_mark(s: &str, tone: char) -> String {
	let mut index = s.len() - 1;
	let mut result = String::new();
	let mut ending = String::new();

	if s.ends_with("ng") {
		index -= 2;
		ending = "ng".to_string();
	}
	else if s.ends_with("ao") || s.ends_with("ou") || s.ends_with('n') {
		index -= 1;
		if let Some(c) = s.chars().last() {
			ending = c.to_string();
		}
	}

	let c = match s.chars().nth(index) {
		None => return s.to_string(),
		Some(c) => add_tone_to_char(c, tone)
	};

	result.push_str(&s[..index]);
	result.push(c);
	result.push_str(&ending);
	return result;
}

fn add_tone_to_char(char: char, tone: char) -> char {
	match char {
		'a' => match tone {
			'1' => 'ā',
			'2' => 'á',
			'3' => 'ǎ',
			'4' => 'à',
			_ => 'a'
		}
		'e' => match tone {
			'1' => 'ē',
			'2' => 'é',
			'3' => 'ě',
			'4' => 'è',
			_ => 'e'
		},
		'i' => match tone {
			'1' => 'ī',
			'2' => 'í',
			'3' => 'ǐ',
			'4' => 'ì',
			_ => 'i'
		},
		'o' => match tone {
			'1' => 'ō',
			'2' => 'ó',
			'3' => 'ǒ',
			'4' => 'ò',
			_ => 'o'
		},
		'u' => match tone {
			'1' => 'ū',
			'2' => 'ú',
			'3' => 'ǔ',
			'4' => 'ù',
			_ => 'u'
		},
		'ü' => match tone {
			'1' => 'ǖ',
			'2' => 'ǘ',
			'3' => 'ǚ',
			'4' => 'ǜ',
			_ => 'ü'
		},
		c => c
	}
}
