use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use local_ip_address::local_ip;

use crate::check_answer::check_answer;
use crate::file::*;
use crate::html::*;
use crate::question::Question;
use crate::question_result::QuestionResult;
use crate::read_form::read_form;
use crate::utils::*;
use crate::word::LearnState;

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
    let mut server = HttpServer::new(|| App::new()
        .service(get)
        .service(post))
        .bind(("127.0.0.1", port))?;

    match local_ip().and_then(|x| Ok(x.to_string())) {
        Ok(ip) => {
            println!("Listening on {}:{}", ip, port);
            server = server.bind((ip, port))?;
        }
        Err(e) => {
            eprintln!("Failed to get local IP address: {}", e);
            println!("Listening on 127.0.0.1:{}", port);
        }
    }
    server.run().await
}

fn port() -> u16 {
    let default_port = 8080;
    let args: Vec<String> = std::env::args().collect();
    let index = match args.iter().position(|x| x == "-p" || x == "--port") {
        None => return default_port,
        Some(x) => x
    };

    let arg = match args.get(index + 1) {
        None => {
            eprintln!("Missing number argument after port");
            return default_port;
        }
        Some(x) => x
    };

    return match arg.parse::<u16>() {
        Ok(x) =>  x,
        Err(_) => {
            eprintln!("Could not parse {} as port number", arg);
            default_port
        }
    };
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

    if veto && state.reviews {
        set_learn_state(&mut words, &state.previous_word, LearnState::Learnt);
        if let Some(err) = write_file(&state, &words) {
            return get_error_html(err);
        }
        return get_html(&current_word, &state.question_type, QuestionResult::None);
    }

    let correction = match veto {
        true => None,
        false => check_answer(answer, tones, &current_word, &state.question_type)
    };
    let update_ok = update_learnt_state(&mut words, &state, correction.is_none());
    if !update_ok {
        return get_error_html(format!(
            "Failed to update learnt state. Word {} not found in the list", &state.current_word));
    }

    let prev_q = state.question_type.clone();
    let word_count_limit = get_word_limit(&state, &words);
    state.update(correction.is_none(), word_count_limit);

    let next_word = match get_next_word(&state, &mut words) {
        Some(x) => x,
        None => return get_error_html(String::from("Failed to get new randomised word"))
    };
    state.current_word = next_word.chinese.clone();

    let question_result = match (correction, prev_q) {
        (_, Question::AllRevealed) => QuestionResult::None,
        (None, _) => QuestionResult::Correct,
        (Some(x), _) => QuestionResult::Incorrect(x)
    };

    if let Some(err) = write_file(&state, &words) {
        return get_error_html(err);
    }
    return get_html(&next_word, &state.question_type, question_result);
}
