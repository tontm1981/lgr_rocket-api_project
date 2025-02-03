#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

extern crate pretty_env_logger;

mod cors;
mod handlers;
mod models;
mod persistance;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use cors::*;
use handlers::*;
use crate::persistance::answers_dao::{AnswersDao, AnswersDaoImpl};
use crate::persistance::questions_dao::{QuestionsDao, QuestionsDaoImpl};

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://lgr:123456@localhost:5431/lgr")
        .await
        .expect("Unable to connect to database");

    let questions_dao = QuestionsDaoImpl::new(pool.clone());
    let answers_dao = AnswersDaoImpl::new(pool);

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
        .manage(Box::new(questions_dao) as Box<dyn QuestionsDao + Send + Sync>)
        .manage(Box::new(answers_dao) as Box<dyn AnswersDao + Send + Sync>)
}