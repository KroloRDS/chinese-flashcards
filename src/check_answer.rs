use percent_encoding::percent_decode;
use crate::question::Question;
use crate::tones::add_tone_marks;
use crate::word::Word;

pub fn check_answer(answer: &str, tones: &str, word: &Word, question: &Question) -> Option<String> {
	let answer = format_answer(answer);
	let correct = match question {
		Question::AllRevealed => return None,
		Question::Meaning => check_meaning(&answer, word),
		Question::Reading => check_reading(&answer, tones, word),
		Question::Writing => answer == word.chinese
	};

	return match correct && !answer.is_empty() {
		true => None,
		false => Some(get_correct_answer(word, question))
	}
}

fn format_answer(answer: &str) -> String {
	let decoded = percent_decode(answer.as_bytes());
	let utf8 = decoded.decode_utf8_lossy();
	let lowercase = utf8.replace('+', " ").to_lowercase();
	return lowercase.trim().to_string();
}

fn check_reading(answer: &str, tones: &str, word: &Word) -> bool {
	let correct_tones = word.tones.trim();
	let mut tones = tones.trim().to_string();
	while tones.len() < correct_tones.len() {
		tones.push('0');
	}

	let latin = word.latin.replace('ü', "v").replace(' ', "");
	return answer.replace(' ', "") == latin.trim() && tones == correct_tones;
}

fn check_meaning(answer: &str, word: &Word) -> bool {
	let meanings = word.translation
		.split([',', ';'])
		.chain(word.clarification.split([',', ';']))
		.map(|s| s.trim().to_lowercase())
		.collect::<Vec<_>>();

	return meanings.contains(&answer.to_string());
}

fn get_correct_answer(word: &Word, question: &Question) -> String {
	match question {
		Question::AllRevealed => String::new(),
		Question::Meaning => combine_meanings(word),
		Question::Reading => add_tone_marks(&word.latin, &word.tones),
		Question::Writing => word.chinese.clone()
	}
}

fn combine_meanings(word: &Word) -> String {
	let mut result = word.translation.clone();
	if !result.is_empty() && !word.clarification.is_empty() {
		result.push_str("; ");
	}
	result.push_str(&word.clarification);
	return result;
}
