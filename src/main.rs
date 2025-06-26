use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

use crate::check_answer::check_answer;
use crate::file::*;
use crate::html::*;
use crate::question_result::QuestionResult;
use crate::read_form::read_form;
use crate::utils::*;

mod question;
mod word;
mod check_answer;
mod state;
mod file;
mod html;
mod utils;
mod question_result;
mod tones;
mod read_form;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = port();
    println!("Listening on port {}", port);
    HttpServer::new(|| App::new()
        .service(get)
        .service(post))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}

fn port() -> u16 {
    let args: Vec<String> = std::env::args().collect();
    return args.iter()
        .enumerate()
        .find_map(|(i, arg)| {
            if arg == "-p" || arg == "--port" {
                args.get(i + 1).and_then(|p| p.parse::<u16>().ok())
            } else {
                None
            }
        })
        .unwrap_or(8080);
}

#[get("/")]
async fn get() -> impl Responder {
    let html = match get_state() {
        Ok((word, _, state)) => get_html(&word, &state.question_type, QuestionResult::None),
        Err(e) => get_error_html(e)
    };
    HttpResponse::Ok().body(html)
}

#[post("/")]
async fn post(req_body: String) -> impl Responder {
    let (answer, tones, veto) = read_form(&req_body);
    let html = get_html_from_answer(&answer, &tones, veto);
    HttpResponse::Ok().body(html)
}

fn get_html_from_answer(answer: &str, tones: &str, veto: bool) -> String {
    let (current_word, mut words, mut state) = match get_state() {
        Ok(x) => x,
        Err(e) => return get_error_html(e)
    };

    if veto {
        let guess_count = state.previous_correct_guesses + 1;
        set_guess_counter(&mut words, &state.previous_word, guess_count);
        if let Some(err) = write_file(&state, &words) {
            return get_error_html(err);
        }
        return get_html(&current_word, &state.question_type, QuestionResult::None);
    }

    let correction = check_answer(answer, tones, &current_word, &state.question_type);
    let update_ok = update_counter(&mut words, &state, current_word.correct_guesses,
        correction.is_none());
    if !update_ok {
        return get_error_html(format!(
            "Failed to update guess counter. Word {} not found in the list", &state.current_word));
    }

    let word_count_limit = get_word_limit(&state, &words);
    state.update(correction.is_none(), current_word.correct_guesses, word_count_limit);

    let next_word = match get_next_word(&state, &words) {
        Some(x) => x,
        None => return get_error_html(String::from("Failed to get new randomised word"))
    };
    state.current_word = next_word.chinese.clone();

    let question_result = match correction {
        None => QuestionResult::Correct,
        Some(x) => QuestionResult::Incorrect(x)
    };

    if let Some(err) = write_file(&state, &words) {
        return get_error_html(err);
    }
    return get_html(&next_word, &state.question_type, question_result);
}
