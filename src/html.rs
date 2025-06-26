use crate::question::Question;
use crate::question_result::QuestionResult;
use crate::tones::add_tone_marks;
use crate::word::Word;

pub fn get_html(word: &Word, question: &Question, result: QuestionResult) -> String {
	let mut template = String::from(include_str!("../html/template.html"));

	let style = include_str!("../html/style.css");
	template = template.replace("&style&", style);

	let mut body = String::new();
	body.push_str(question_result(result).as_str());
	body.push_str(question_txt(word, question).as_str());
	body.push_str(form(word, question).as_str());
	template = template.replace("&body&", &body);

	return template;
}

fn question_result(result: QuestionResult) -> String {
	let mut txt = String::new();

	match result {
		QuestionResult::None => {}
		QuestionResult::Correct => txt.push_str(div(
			"correct", "That was CORRECT!").as_str()),
		QuestionResult::Incorrect(x) => {
			let message = format!("That was INCORRECT!<br>Expected: {}", x);
			txt.push_str(div("incorrect", &message).as_str());
			txt.push_str(include_str!("../html/veto-button.html"));
		}
	}

	return txt;
}

fn question_txt(word: &Word, question: &Question) -> String {
	let mut result = String::new();

	match question {
		Question::AllRevealed => {
			let pinyin = add_tone_marks(&word.latin, &word.tones);
			result.push_str(div("chinese-text", &word.chinese).as_str());
			result.push_str(div("pinyin-text", &pinyin).as_str());
			result.push_str(div("translation-text", &word.translation).as_str());
			result.push_str(div("clarification-text", &word.clarification).as_str());
		}
		Question::Meaning | Question::Reading =>
			result.push_str(random_font(&word.chinese).as_str()),
		Question::Writing => {
			result.push_str(div("translation-text", &word.translation).as_str());
			result.push_str(div("clarification-text", &word.clarification).as_str());
		}
	}

	return result;
}

fn div(class: &str, content: &str) -> String {
	return format!("<div class=\"{}\">{}</div>", class, content);
}

fn random_font(text: &str) -> String {
	return match rand::random::<bool>() {
		true => format!("<div class=\"chinese-text\" style=\"font-family: serif;\">{}</div>", text),
		false => div("chinese-text", &text)
	}
}

fn form(word: &Word, question: &Question) -> String {
	let mut result = String::new();

	let input = include_str!("../html/normal-input.html");
	match question {
		Question::AllRevealed =>
			result.push_str(include_str!("../html/next-button.html")),
		Question::Meaning => {
			let input = input.replace("&label&", "Translation:");
			result.push_str(input.as_str());
		}
		Question::Reading => {
			result.push_str(tone_inputs(&word.tones).as_str());

			let input = input.replace("&label&", "Pinyin:");
			result.push_str(input.as_str());
		}
		Question::Writing => result.push_str(include_str!("../html/big-input.html"))
	}

	return result;
}

fn tone_inputs(tones: &str) -> String {
	let tone_input = include_str!("../html/tone-input.html");
	let mut result = String::new();
	for i in 1..tones.len() + 1 {
		let replaced = tone_input.replace("&n&", &format!("{:02}", i));
		result.push_str(&replaced);
	}

	return result;
}

pub fn get_error_html(error: String) -> String {
	let mut template = String::from(include_str!("../html/template.html"));

	let style = include_str!("../html/style.css");
	template = template.replace("&style&", style);
	template = template.replace("&body&", div("incorrect", &error).as_str());

	return template;
}
