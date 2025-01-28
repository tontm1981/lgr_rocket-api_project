use rocket::{serde::json::Json};

use crate::models::*;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
#[allow(unused_variables)]
pub async fn create_question(
    question: Json<Question>,
) -> Json<QuestionDetail> {
    Json(QuestionDetail {
        question_uuid: "question_id".to_string(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_string(),
    })
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    let questions = vec![
        QuestionDetail {
            question_uuid: "question_id".to_string(),
            title: "title".to_owned(),
            description: "description".to_string(),
            created_at: "created_at".to_string(),
        },
        QuestionDetail {
            question_uuid: "uuid2".to_string(),
            title: "title2".to_string(),
            description: "description 2".to_owned(),
            created_at: "created_at".to_string(),
        }
    ];
    Json(questions)
}

#[delete("/question", data = "<question_uuid>")]
#[allow(unused_variables)]
pub async fn delete_question(
    question_uuid: Json<QuestionId>
) {
}

// ---- CRUD for Answers ----

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//
//       hint: this function should look very similar to the create_question function above
#[post("/answer", data = "<answer>")]
#[allow(unused_variables)]
pub async fn create_answer( answer: Json<Answer> ) -> Json<AnswerDetail> {
    Json(AnswerDetail {
        question_uuid: "question_uuid".to_string(),
        answer_uuid: "answer_uuid".to_string(),
        content: "content".to_owned(),
        created_at: "created_at".to_string(),
    })
}

// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//
//       hint: this function should look very similar to the read_questions function above
#[get("/answers/<question_id>")]
#[allow(unused_variables)]
pub async fn read_answers( question_id: String) -> Json<Vec<AnswerDetail>> {
    let answers = vec![
        AnswerDetail {
            question_uuid: "question_uuid".to_owned(),
            answer_uuid: "answer_uuid".to_owned(),
            content: "content".to_owned(),
            created_at: "created_at".to_string(),
        },
    ];
    Json(answers)
}

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//
//       hint: this function should look very similar to the delete_question function above
#[delete("/answer", data="<answer_id>")]
#[allow(unused_variables)]
pub async fn delete_answer( answer_id: String ) {
    todo!()
}