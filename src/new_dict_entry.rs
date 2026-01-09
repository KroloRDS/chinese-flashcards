use crate::tones::remove_tone_marks;

pub async fn dict_test(str: &str) -> Option<String> {
	let url = String::from("https://www.mdbg.net/chinese/dictionary?wdrst=1&wdqb=") + str;
	let res = reqwest::get(url).await.ok()?;
	let html = res.text().await.ok()?;

	let index = html.find("<td class=\"resultswrap\">")?;
	let html = &html[index..];

	let index = html.find("</table>")?;
	let html = &html[..index];

	let mut result = String::from("3\t");
	let pinyin = match get_all_pinyin(html) {
		Some(pinyin) => pinyin,
		None => return None
	};

	match get_all_hanzi_and_tones(html) {
		Some((h, t)) => {
			result.push_str(&h);
			result.push_str("\t");
			result.push_str(&pinyin);
			result.push_str("\t");
			result.push_str(&t);
			result.push_str("\t");
		}
		_ => return None
	}

	match get_translation(html) {
		Some(x) => result.push_str(&x),
		None => return None
	}

	return Some(result);
}

fn get_all_hanzi_and_tones(html: &str) -> Option<(String, String)> {
	let html = get_section(html, "hanzi")?;

	let chars = html.split("<wbr />")
		.map(|x| parse_single_hanzi(x))
		.collect::<Vec<_>>();

	let mut hanzi = String::new();
	let mut tones = String::new();

	for char in chars {
		match char {
			Some((t, h)) => {
				hanzi.push(h);
				tones.push(t);
			},
			None => return None,
		}
	}

	return Some((hanzi, tones));
}

fn parse_single_hanzi(html: &str) -> Option<(char, char)> {
	let mut chars = html.chars();
	let tone = chars.nth(16);
	let hanzi = chars.nth(2);
	return match (tone, hanzi) {
		(Some('5'), Some(hanzi)) => Some(('0', hanzi)),
		(Some(tone), Some(hanzi)) => Some((tone, hanzi)),
		_ => None
	};
}

fn get_all_pinyin(html: &str) -> Option<String> {
	let html = get_section(html, "pinyin")?;
	let mut vec = Vec::<String>::new();

	for char in html.split("&#8203;") {
		match parse_single_pinyin(char) {
			Some(x) => vec.push(x),
			None => return None
		}
	}

	return Some(vec.join(" "));
}

fn parse_single_pinyin(html: &str) -> Option<String> {
	if html.len() <= 19 {
		return None;
	}
	let html = &html[19..];

	let index = html.find("<");
	let pinyin = match index {
		Some(index) => &html[..index],
		None => html,
	};

	return Some(remove_tone_marks(pinyin.to_string()));
}

fn get_translation(html: &str) -> Option<String> {
	let find = "<div class=\"defs\">";
	let index = html.find(find)?;
	let html = &html[index + find.len()..];

	let index = html.find("</div>")?;
	let html = &html[..index];

	let mut vec = html
		.split("<strong>/</strong>")
		.map(|x| x.trim())
		.collect::<Vec<_>>();

	if vec.len() == 0 {
		return None;
	}

	let mut translations = vec.remove(0).to_lowercase();
	translations.push_str("\t");
	translations.push_str(&vec.join("; "));

	return Some(translations);
}

fn get_section(html: &str, section: &str) -> Option<String> {
	let mut class = String::from("class=\"");
	class.push_str(section);
	let index = html.find(&class)?;
	let html = &html[index..];

	let index = html.find("</div>")?;
	let html = &html[..index];

	let lindex = html.find("<span")?;
	let rindex = html.rfind("</span")?;
	let result = &html[lindex..rindex];
	return Some(result.to_string());
}
