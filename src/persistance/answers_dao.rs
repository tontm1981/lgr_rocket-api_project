use async_trait::async_trait;
use rocket::http::ext::IntoCollection;
use sqlx::PgPool;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        todo!() // return an instance of AnswersDaoImpl
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        // Use the `sqlx::types::Uuid::parse_str` method to parse the `question_uuid` field
        // in `Answer` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid)
            .map_err(|e| DBError::Other(Box::new(e)))?;

        // Make a database query to insert a new answer.
        // Here is the SQL query:
        // ```
        // INSERT INTO answers ( question_uuid, content )
        // VALUES ( $1, $2 )
        // RETURNING *
        // ```
        // If executing the query results in an error, check to see if
        // the error code matches `postgres_error_codes::FOREIGN_KEY_VIOLATION`.
        // If so early return the `DBError::InvalidUUID` error. Otherwise, early return
        // the `DBError::Other` error.
        let record = sqlx::query!(
                "INSERT INTO public.answers (question_uuid, content) VALUES ($1, $2) RETURNING *",
                uuid,
                answer.content
            )
            .fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        // Populate the AnswerDetail fields using `record`.
        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: answer.question_uuid.to_string(),
            content: answer.content.to_string(),
            created_at: answer.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        // Use the `sqlx::types::Uuid::parse_str` method to parse `answer_uuid` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid = sqlx::types::Uuid::parse_str(&answer_uuid).map_err(|e| DBError::Other(Box::new(e)))?;

        // TODO: Make a database query to delete an answer given the answer uuid.
        // Here is the SQL query:
        // ```
        // DELETE FROM answers WHERE answer_uuid = $1
        // ```
        // If executing the query results in an error, map that error
        // to a `DBError::Other` error and early return from this function.

        let result = sqlx::query!("DELETE FROM public.answers WHERE answer_uuid = $1", uuid)
            .fetch_one(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        // Use the `sqlx::types::Uuid::parse_str` method to parse `question_uuid` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid).map_err(|e| DBError::Other(Box::new(e)))?;

        // Make a database query to get all answers associated with a question uuid.
        // Here is the SQL query:
        // ```
        // SELECT * FROM answers WHERE question_uuid = $1
        // ```
        // If executing the query results in an error, map that error
        // to a `DBError::Other` error and early return from this function.
        let records = sqlx::query!("SELECT * FROM public.answers WHERE question_uuid = $1", uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        // Iterate over `records` and map each record to a `AnswerDetail` type
        let answers = records
            .iter()
            .map(|r| {
                let question_uuid = uuid.to_string();
                let content = r.content.to_string();
                let answer_uuid = r.answer_uuid.to_string();
                let created_at = r.created_at.to_string();
                AnswerDetail{
                    question_uuid,
                    answer_uuid,
                    content,
                    created_at,
                };
            })
            .into_collection()
            .to_vec();

        Ok(answers)
    }
}