use percent_encoding::percent_decode;
use crate::question::Question;
use crate::tones::add_tone_marks;
use crate::word::Word;

pub fn check_answer(answer: &str, tones: &str, word: &Word, question: &Question) -> Option<String> {
	let answer = percent_decode(answer.as_bytes()).decode_utf8_lossy()
		.to_string().to_lowercase();

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

fn check_reading(answer: &str, tones: &str, word: &Word) -> bool {
	let latin = word.latin.replace('ü', "v").replace(' ', "");
	return answer.replace('+', "") == latin.trim() && tones == word.tones.trim();
}

fn check_meaning(answer: &str, word: &Word) -> bool {
	let answer = answer.replace('+', " ");
	let meanings = word.translation
		.split([',', ';'])
		.chain(word.clarification.split([',', ';']))
		.map(|s| s.trim().to_lowercase().to_string())
		.collect::<Vec<String>>();

	return meanings.contains(&answer);
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
