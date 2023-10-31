// ------------------------------------------------------
// Generated with github.com/thelexoplexx/prisma-generator-poem-openapi
// Do not edit manually.
// ------------------------------------------------------

use poem::{web::Data, Result};
use poem_openapi::{payload::Json, Object, OpenApi};
use sqlx::PgPool;

#[derive(Object)]
pub struct Part {
    part_number: String,
}

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/part", method = "get")]
    async fn get_part(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Part>>> {
        let get = sqlx::query_as!(Part, "SELECT * FROM \"Part\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get Part");
        Ok(Json(get))
    }

    #[oai(path = "/part", method = "post")]
    async fn post_part(&self, pool: Data<&PgPool>, part: Json<Part>) -> Result<Json<String>> {
        let id = sqlx::query("INSERT INTO \"Part\" (part_number) values ($1)")
            .bind(&part.part_number)
            .execute(pool.0)
            .await
            .expect("Failed to insert Part");

        dbg!(id);
        // .bind(description.0)
        // .execute(pool.0)
        // .await
        // .map_err(InternalServerError)?
        // .last_insert_rowid();

        Ok(Json("ok".to_string()))
    }
}
