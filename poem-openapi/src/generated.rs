use poem::{web::Data, Result};
use poem_openapi::{
    payload::{Json, PlainText},
    Object, OpenApi,
};
use sqlx::PgPool;

#[derive(Object)]
pub struct Serialnumberpart {
    serial: String,
    part_id: String,
    plant_id: Option<String>,
    // group: Vec<String>, //The idea came from prisma, basically select * from serialnumberpart where serialnumberpart_id = serial
    parent_id: Option<String>,
    manufacturing_date: Option<i32>, // Should be DateTime but for the life of me I couldn't figure out how. Only possible option
    shipping_date: Option<i32>, // would be to create an intermediary Type for DateTime where the DB-DateTime is converted
    installation_date: Option<i32>, // back and forth but that did not seem like a viable option to me.
}

pub struct API;

#[OpenApi]
impl API {
    #[oai(path = "/serialnumberparts", method = "get")]
    async fn list_serialnumber(
        &self,
        pool: Data<&PgPool>,
        description: PlainText<String>,
    ) -> Result<Json<Vec<Serialnumberpart>>> {
        dbg!(description);

        let serialnumberpart = sqlx::query_as!(Serialnumberpart, "select * from serialnumberpart")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get Serialnumberpart");

        Ok(Json(serialnumberpart))
    }
}
