use crate::question::Question;

pub struct State {
	pub previous_correct_guesses: u16,
	pub previous_word: String,
	pub current_word: String,
	pub question_type: Question,
	pub reviews: bool,
	pub word_number: u8
}

impl State {
	pub fn update(&mut self, answer_correct: bool, previous_correct_guesses: u16, word_limit: u8) {
		self.previous_correct_guesses = previous_correct_guesses;
		self.previous_word = self.current_word.clone();
		if self.should_inc_word_count(answer_correct) {
			self.word_number += 1;
		}
		if self.word_number >= word_limit {
			self.word_number = 0;
			self.reviews = !self.reviews;
		}
		if self.reviews || answer_correct {
			self.question_type = self.question_type.next(self.reviews);
		}
	}

	fn should_inc_word_count(&self, answer_correct: bool) -> bool {
		if self.reviews {
			return true;
		}

		return match self.question_type {
			Question::Writing => answer_correct,
			_ => false
		}
	}

	pub fn serialize(&self) -> String {
		return format!("{}\t{}\t{}\t{}\t{}\t{}",
			self.previous_correct_guesses,
			self.previous_word,
			self.current_word,
			self.question_type.serialize(),
			self.reviews,
			self.word_number);
	}

	pub fn deserialize(str: &str) -> Self {
		let fields: Vec<&str> = str.split('\t').collect();
		return Self {
			previous_correct_guesses: Self::get_field_n(&fields, 0)
				.parse::<u16>().unwrap_or(0),
			previous_word: Self::get_field_n(&fields, 1),
			current_word: Self::get_field_n(&fields, 2),
			question_type: Question::deserialize(&Self::get_field_n(&fields, 3)),
			reviews: Self::get_field_n(&fields, 4)
				.parse::<bool>().unwrap_or(true),
			word_number: Self::get_field_n(&fields, 5)
				.parse::<u8>().unwrap_or(0),
		};
	}

	fn get_field_n(fields: &Vec<&str>, n: usize) -> String {
		return match fields.get(n) {
			Some(field) => field.to_string(),
			None => String::new()
		};
	}
}
