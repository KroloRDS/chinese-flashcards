#[derive(Clone)]
pub struct Word {
	pub learn_state: LearnState,
	pub chinese: String,
	pub latin: String,
	pub tones: String,
	pub translation: String,
	pub clarification: String
}

#[derive(Clone)]
pub enum LearnState {
	NotLearnt,
	Learnt,
	ForReview,
	Forgotten
}

impl LearnState {
	pub fn serialize(&self) -> String {
		match self {
			LearnState::NotLearnt => "0",
			LearnState::Learnt => "2",
			LearnState::ForReview => "1",
			LearnState::Forgotten => "3"
		}.to_string()
	}

	pub fn deserialize(str: String) -> LearnState {
		match str.trim() {
			"1" => LearnState::ForReview,
			"2" => LearnState::Learnt,
			"3" => LearnState::Forgotten,
			_ => LearnState::NotLearnt
		}
	}
}

impl Word {
	pub fn serialize(&self) -> String {
		return format!("{}\t{}\t{}\t{}\t{}\t{}",
			self.learn_state.serialize(),
			self.chinese,
			self.latin,
			self.tones,
			self.translation,
			self.clarification);
	}

	pub fn deserialize(str: &str) -> Self {
		let fields: Vec<&str> = str.split('\t').collect();
		return Self {
			chinese: Self::get_field_n(&fields, 1),
			latin: Self::get_field_n(&fields, 2),
			tones: Self::get_field_n(&fields, 3),
			translation: Self::get_field_n(&fields, 4),
			clarification: Self::get_field_n(&fields, 5),
			learn_state: LearnState::deserialize(
				Self::get_field_n(&fields, 0))
		};
	}

	fn get_field_n(fields: &Vec<&str>, n: usize) -> String {
		return match fields.get(n) {
			Some(field) => field.trim().to_string(),
			None => String::new()
		};
	}
}
