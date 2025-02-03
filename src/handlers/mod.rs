mod handlers_inner;

use rocket::{serde::json::Json, State};
use crate::{
    models::*,
    persistance::{
        answers_dao::AnswersDao,
        questions_dao::QuestionsDao
    },
};
use handlers_inner::*;

#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalServerError(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(message) => Self::BadRequest(message),
            HandlerError::InternalError(s) => Self::InternalServerError(s),
        }
    }
}

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
#[allow(unused_variables)]
pub async fn create_question(
    question: Json<Question>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<QuestionDetail>, APIError> {
    match handlers_inner::create_question(question.0, questions_dao.inner()).await {
        Ok(details) => Ok(Json(details)),
        Err(err) => Err(err.into()),
    }
}

#[get("/questions")]
pub async fn read_questions(questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>) -> Result<Json<Vec<QuestionDetail>>, APIError> {
    /**
     *  I know it's not recommended to leave comments in code, but it's just an important note.
     *  There's another way, without using `match`. We can do as following code, but in `map_err`'s
     *  closure, we must use Into::<T>::into(err), or we'll face some strange casting error messages.
     */
    let vec = handlers_inner::read_questions(questions_dao.inner())
        .await
        .map_err(|err| Into::<APIError>::into(err))?;
    Ok(Json(vec))
}

#[delete("/question", data = "<question_uuid>")]
#[allow(unused_variables)]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<(), APIError> {
    let uuid = question_uuid.0;
    match handlers_inner::delete_question(uuid, questions_dao.inner()).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
#[allow(unused_variables)]
pub async fn create_answer(
    answer: Json<Answer>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>,
) -> Result<Json<AnswerDetail>, APIError> {
    let detail = handlers_inner::create_answer(answer.0, answers_dao.inner())
        .await
        .map_err(|err| Into::<APIError>::into(err))?;
    Ok(Json(detail))
}

#[get("/answers", data = "<question_id>")]
#[allow(unused_variables)]
pub async fn read_answers(
    question_id: Json<QuestionId>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>,
) -> Result<Json<Vec<AnswerDetail>>, APIError> {
    let vec = handlers_inner::read_answers(question_id.0, answers_dao.inner())
        .await;
    match vec {
        Ok(answers) => Ok(Json(answers)),
        Err(err) => Err(err.into()),
    }
}

#[delete("/answer", data="<answer_id>")]
#[allow(unused_variables)]
pub async fn delete_answer(
    answer_id: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>,
) -> Result<(), APIError> {
    handlers_inner::delete_answer(answer_id.0, answers_dao.inner())
        .await
        .map_err(|err| Into::<APIError>::into(err))?;
    Ok(())
}