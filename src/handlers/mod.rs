mod handlers_inner;

use rocket::{serde::json::Json, State};

use crate::models::*;
use crate::persistance::answers_dao::AnswersDao;
use crate::persistance::questions_dao::QuestionsDao;
// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
#[allow(unused_variables)]
pub async fn create_question(
    question: Json<Question>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Json<QuestionDetail> {
    let detail = questions_dao
        .create_question(question.0)
        .await
        .expect("Unable to create question");
    Json(detail)
}

#[get("/questions")]
pub async fn read_questions(questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>) -> Json<Vec<QuestionDetail>> {
    let vec = questions_dao
        .get_questions()
        .await
        .expect("Unable to retrieve questions");
    Json(vec)
}

#[delete("/question", data = "<question_uuid>")]
#[allow(unused_variables)]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) {
    let uuid = question_uuid.0;
    questions_dao
        .delete_question(uuid.question_uuid)
        .await
        .expect("Unable to delete question");
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
#[allow(unused_variables)]
pub async fn create_answer(
    answer: Json<Answer>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>,
) -> Json<AnswerDetail> {
    let detail = answers_dao
        .create_answer(answer.0)
        .await
        .expect("Unable to create answer");
    Json(detail)
}

#[get("/answers", data = "<question_id>")]
#[allow(unused_variables)]
pub async fn read_answers(
    question_id: Json<QuestionId>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>,
) -> Json<Vec<AnswerDetail>> {
    let vec = answers_dao
        .get_answers(question_id.0.question_uuid)
        .await
        .expect("Unable to retrieve answers");
    Json(vec)
}

#[delete("/answer", data="<answer_id>")]
#[allow(unused_variables)]
pub async fn delete_answer(
    answer_id: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>,
) {
    answers_dao
        .delete_answer(answer_id.0.answer_uuid)
        .await
        .expect("Unable to delete answer");
}