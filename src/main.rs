#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

#[macro_use]
extern crate pretty_env_logger;

mod cors;
mod handlers;
mod models;
mod persistance;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use cors::*;
use handlers::*;
use crate::persistance::answers_dao::AnswersDaoImpl;
use crate::persistance::questions_dao::QuestionsDaoImpl;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().expect("Unable to initialize dotenv");

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
        // The manage method allows us to add state to the state managed by this instance of Rocket. Then we can use this state in the handlers.
        .manage(questions_dao) // pass in `questions_dao` as a boxed trait object. hint: you must cast `questions_dao` to a trait object.
        .manage(answers_dao) // pass in `answers_dao` as a boxed trait object. hint: you must cast `answers_dao` to a trait object.
}