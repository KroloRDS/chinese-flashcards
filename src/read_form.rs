pub fn read_form(data: &str) -> (String, String, bool) {
	let data = data.trim().to_lowercase();
	let lines = data.split('&').collect::<Vec<_>>();
	return (get_answer(&lines), get_tones(&lines), get_veto(&lines));
}

fn get_answer(lines: &Vec<&str>) -> String {
	let name = "answer=";
	let line = lines.iter().find(|line| line.starts_with(name));
	return match line {
		None => String::new(),
		Some(x) => x[name.len()..].to_string()
	};
}

fn get_veto(lines: &Vec<&str>) -> bool {
	lines.iter().any(|line| line.starts_with("veto"))
}

fn get_tones(lines: &Vec<&str>) -> String {
	let mut tones = lines.iter().filter(|line| line.starts_with("tone"))
		.map(|line| line[4..].to_string())
		.filter(|line| matches_format(line))
		.collect::<Vec<_>>();
	tones.sort();

	let mut result = String::new();
	let mut iter = tones.iter();
	let mut next = iter.next();

	while let Some(input) = next {
		if input.starts_with(format!("{:02}", result.len() + 1).as_str()) {
			result.push_str(&input[3..]);
			next = iter.next();
		}
		else {
			result.push_str("0");
		}
	}

	return result;
}

fn matches_format(str: &str) -> bool {
	let mut chars = str.chars();
	if chars.next().is_none_or(|c| !c.is_digit(10)) {
		return false;
	};
	if chars.next().is_none_or(|c| !c.is_digit(10)) {
		return false;
	};
	if chars.next().is_none_or(|c| c != '=') {
		return false;
	};
	if chars.next().is_none_or(|c| c < '0' || c > '4') {
		return false;
	};

	return chars.next().is_none();
}
