use rand::prelude::IndexedRandom;
use crate::file::read_file;
use crate::question::Question;
use crate::state::State;
use crate::word::{LearnState, Word};

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

pub fn get_next_word(state: &State, words: &mut Vec<Word>) -> Option<Word> {
	if !state.reviews {
		return words.iter().find(|word| 
			match word.learn_state {
				LearnState::NotLearnt => state.allow_word_pool_increase,
				LearnState::Forgotten => true,
				_ => false
			}
		).cloned();
	}

	let for_review = words.iter()
		.filter(|word| matches!(word.learn_state, LearnState::ForReview))
		.collect::<Vec<_>>();

	if !for_review.is_empty() {
		return for_review.choose(&mut rand::rng()).cloned().cloned();
	}

	let mut learnt = words.iter_mut()
		.filter(|word| matches!(word.learn_state, LearnState::Learnt))
		.collect::<Vec<_>>();

	learnt.iter_mut().for_each(|x| x.learn_state = LearnState::ForReview);
	return learnt.choose(&mut rand::rng()).map(|x| Word {
		learn_state: x.learn_state.clone(),
		chinese: x.chinese.clone(),
		latin: x.latin.clone(),
		tones: x.tones.clone(),
		translation: x.translation.clone(),
		clarification: x.clarification.clone(),
	});
}

pub fn find_word(chinese: &str, words: &Vec<Word>) -> Option<Word> {
	words.iter().find(|word| word.chinese == chinese).cloned()
}

pub fn update_learnt_state(mut words: &mut Vec<Word>,
	state: &State, answer_correct: bool
) -> bool {
	if !state.reviews && !matches!(state.question_type, Question::Writing) {
		return true;
	}

	let learnt_state = match answer_correct {
		true => LearnState::Learnt,
		false => LearnState::Forgotten
	};
	return set_learn_state(&mut words, &state.current_word, learnt_state);
}

pub fn set_learn_state(words: &mut Vec<Word>, chinese: &str, state: LearnState) -> bool {
	for word in words.iter_mut() {
		if word.chinese == chinese {
			word.learn_state = state;
			return true;
		}
	}
	return false;
}
