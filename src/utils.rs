use rand::prelude::IndexedRandom;
use crate::file::read_file;
use crate::question::Question;
use crate::state::State;
use crate::word::Word;

pub fn get_state() -> Result<(Word, Vec<Word>, State), String> {
	let (state, words) = match read_file() {
		Ok(x) => x,
		Err(e) => return Err(e)
	};

	return match find_word(&state.current_word, &words) {
		Some(word) => Ok((word, words, state)),
		None =>  Err(format!("Invalid state. Word {} not found in the list", state.current_word))
	}
}

pub fn get_next_word(state: &State, words: &Vec<Word>) -> Option<Word> {
	if !state.reviews {
		return words.iter().find(|word| word.correct_guesses == 0).cloned()
	}

	let mut min = u16::MAX;
	let mut min_words: Vec<Word> = Vec::new();

	for word in words.iter() {
		if word.correct_guesses == 0 {
			continue;
		}
		else if word.correct_guesses < min {
			min = word.correct_guesses;
			min_words = vec![word.clone()];
		}
		else if word.correct_guesses == min {
			min_words.push(word.clone());
		}
	}

	return min_words.choose(&mut rand::rng()).cloned();
}

pub fn find_word(chinese: &str, words: &Vec<Word>) -> Option<Word> {
	words.iter().find(|word| word.chinese == chinese).cloned()
}

pub fn update_counter(mut words: &mut Vec<Word>, state: &State,
	correct_guesses: u16, answer_correct: bool
) -> bool {
	if !state.reviews && !matches!(state.question_type, Question::Writing) {
		return true;
	}

	let counter = match answer_correct {
		true => correct_guesses + 1,
		false => 0
	};
	return set_guess_counter(&mut words, &state.current_word, counter);
}

pub fn set_guess_counter(words: &mut Vec<Word>, chinese: &str, value: u16) -> bool {
	for word in words.iter_mut() {
		if word.chinese == chinese {
			word.correct_guesses = value;
			return true;
		}
	}
	return false;
}

pub fn get_word_limit(state: &State, words: &Vec<Word>) -> u8 {
	if !state.reviews {
		return 3;
	}
	
	let learnt_words = words.iter().filter(|word| word.correct_guesses > 0).count();
	return (learnt_words as f64).sqrt().ceil() as u8;
}
