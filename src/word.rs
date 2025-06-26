#[derive(Clone)]
pub struct Word {
	pub correct_guesses: u16,
	pub chinese: String,
	pub latin: String,
	pub tones: String,
	pub translation: String,
	pub clarification: String
}

impl Word {
	pub fn serialize(&self) -> String {
		return format!("{}\t{}\t{}\t{}\t{}\t{}",
			self.correct_guesses,
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
			correct_guesses: Self::get_field_n(&fields, 0)
				.parse::<u16>().unwrap_or(0)
		};
	}

	fn get_field_n(fields: &Vec<&str>, n: usize) -> String {
		return match fields.get(n) {
			Some(field) => field.to_string(),
			None => String::new()
		};
	}
}
