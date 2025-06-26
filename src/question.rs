use rand::prelude::IndexedRandom;

#[derive(Clone)]
pub enum Question {
	AllRevealed,
	Meaning,
	Reading,
	Writing
}
static FOR_REVIEW: [Question; 3] = [Question::Meaning, Question::Reading, Question::Writing];

impl Question {
	pub fn next(&self, reviews: bool) -> Self {
		return match reviews {
			true => FOR_REVIEW.choose(&mut rand::rng()).unwrap().clone(),
			false => match self {
				Self::AllRevealed => Self::Meaning,
				Self::Meaning => Self::Reading,
				Self::Reading => Self::Writing,
				Self::Writing => Self::AllRevealed
			}
		}
	}

	pub fn serialize(&self) -> String {
		return match self {
			Self::AllRevealed => "a".to_string(),
			Self::Meaning => "m".to_string(),
			Self::Reading => "r".to_string(),
			Self::Writing => "w".to_string()
		};
	}

	pub fn deserialize(str: &str) -> Self {
		return match str {
			"a" => Self::AllRevealed,
			"m" => Self::Meaning,
			"r" => Self::Reading,
			"w" => Self::Writing,
			_ => Self::AllRevealed
		};
	}
}
