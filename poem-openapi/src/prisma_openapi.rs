// ------------------------------------------------------
// Generated with github.com/thelexoplexx/prisma-generator-poem-openapi
// Do not edit manually.
// It is not formated because it is not meant to be read or seen anways.
// ------------------------------------------------------

use chrono::NaiveDateTime;
use poem::{error::InternalServerError, web::Data, Result};
use poem_openapi::{payload::Json, Object, OpenApi};
use sqlx::PgPool;

#[derive(Object)]
pub struct System {
    id: String,
    label: String,
    order_number: String,
}

#[derive(Object)]
pub struct Serialnumberpart {
    serial: String,
    edv_nummer: String,
    system_id: Option<String>,
    parent_id: Option<String>,
    manufacturing_date: Option<NaiveDateTime>,
    shipping_date: Option<NaiveDateTime>,
    installation_date: Option<NaiveDateTime>,
}

#[derive(Object)]
pub struct Project {
    id: String,
    project_number: i32,
    description: String,
}

#[derive(Object)]
pub struct Order {
    order_number: String,
    project_id: String,
}

#[derive(Object)]
pub struct FinalCustomer {
    id: String,
    sap_id: String,
    name: String,
    alias: Option<Vec<String>>,
}

#[derive(Object)]
pub struct Factory {
    id: String,
    customer_id: String,
    name: String,
}

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/system", method = "get")]
    async fn get_system(&self, pool: Data<&PgPool>) -> Result<Json<Vec<System>>> {
        let get = sqlx::query_as!(System, "SELECT * FROM \"System\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get System");
        Ok(Json(get))
    }

    #[oai(path = "/system", method = "post")]
    async fn post_system(&self, pool: Data<&PgPool>, input: Json<System>) -> Result<Json<String>> {
        match sqlx::query("INSERT INTO \"System\" (id, label, order_number) values ($1, $2, $3)")
            .bind(&input.id)
            .bind(&input.label)
            .bind(&input.order_number)
            .execute(pool.0)
            .await
        {
            Ok(x) => {
                println!("Inserted System");
                dbg!(x);
                Ok(Json("ok".to_string()))
            }
            Err(e) => {
                println!("Failed to insert System: {}", e);
                Err(InternalServerError(e))
            }
        }
    }

    #[oai(path = "/serialnumberpart", method = "get")]
    async fn get_serialnumberpart(
        &self,
        pool: Data<&PgPool>,
    ) -> Result<Json<Vec<Serialnumberpart>>> {
        let get = sqlx::query_as!(Serialnumberpart, "SELECT * FROM \"Serialnumberpart\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get Serialnumberpart");
        Ok(Json(get))
    }

    #[oai(path = "/serialnumberpart", method = "post")]
    async fn post_serialnumberpart(
        &self,
        pool: Data<&PgPool>,
        input: Json<Serialnumberpart>,
    ) -> Result<Json<String>> {
        match sqlx::query("INSERT INTO \"Serialnumberpart\" (serial, part_id, system_id, parent_id, manufacturing_date, shipping_date, installation_date) values ($1, $2, $3, $4, $5, $6, $7)").bind(&input.serial).bind(&input.edv_nummer).bind(&input.system_id).bind(&input.parent_id).bind(&input.manufacturing_date).bind(&input.shipping_date).bind(&input.installation_date).execute(pool.0).await {
  Ok(x) => {
      println!("Inserted Serialnumberpart");
      dbg!(x);
      Ok(Json("ok".to_string()))
    }
    Err(e) => {
      println!("Failed to insert Serialnumberpart: {}", e);
      Err(InternalServerError(e))
    }
  }
    }

    #[oai(path = "/project", method = "get")]
    async fn get_project(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Project>>> {
        let get = sqlx::query_as!(Project, "SELECT * FROM \"Project\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get Project");
        Ok(Json(get))
    }

    #[oai(path = "/project", method = "post")]
    async fn post_project(
        &self,
        pool: Data<&PgPool>,
        input: Json<Project>,
    ) -> Result<Json<String>> {
        match sqlx::query(
            "INSERT INTO \"Project\" (id, project_number, description) values ($1, $2, $3)",
        )
        .bind(&input.id)
        .bind(&input.project_number)
        .bind(&input.description)
        .execute(pool.0)
        .await
        {
            Ok(x) => {
                println!("Inserted Project");
                dbg!(x);
                Ok(Json("ok".to_string()))
            }
            Err(e) => {
                println!("Failed to insert Project: {}", e);
                Err(InternalServerError(e))
            }
        }
    }

    #[oai(path = "/order", method = "get")]
    async fn get_order(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Order>>> {
        let get = sqlx::query_as!(Order, "SELECT * FROM \"Order\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get Order");
        Ok(Json(get))
    }

    #[oai(path = "/order", method = "post")]
    async fn post_order(&self, pool: Data<&PgPool>, input: Json<Order>) -> Result<Json<String>> {
        match sqlx::query("INSERT INTO \"Order\" (order_number, project_id) values ($1, $2)")
            .bind(&input.order_number)
            .bind(&input.project_id)
            .execute(pool.0)
            .await
        {
            Ok(x) => {
                println!("Inserted Order");
                dbg!(x);
                Ok(Json("ok".to_string()))
            }
            Err(e) => {
                println!("Failed to insert Order: {}", e);
                Err(InternalServerError(e))
            }
        }
    }

    #[oai(path = "/finalcustomer", method = "get")]
    async fn get_finalcustomer(&self, pool: Data<&PgPool>) -> Result<Json<Vec<FinalCustomer>>> {
        let get = sqlx::query_as!(FinalCustomer, "SELECT * FROM \"FinalCustomer\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get FinalCustomer");
        Ok(Json(get))
    }

    #[oai(path = "/finalcustomer", method = "post")]
    async fn post_finalcustomer(
        &self,
        pool: Data<&PgPool>,
        input: Json<FinalCustomer>,
    ) -> Result<Json<String>> {
        match sqlx::query(
            "INSERT INTO \"FinalCustomer\" (id, sap_id, name, alias) values ($1, $2, $3, $4)",
        )
        .bind(&input.id)
        .bind(&input.sap_id)
        .bind(&input.name)
        .bind(&input.alias)
        .execute(pool.0)
        .await
        {
            Ok(x) => {
                println!("Inserted FinalCustomer");
                dbg!(x);
                Ok(Json("ok".to_string()))
            }
            Err(e) => {
                println!("Failed to insert FinalCustomer: {}", e);
                Err(InternalServerError(e))
            }
        }
    }

    #[oai(path = "/factory", method = "get")]
    async fn get_factory(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Factory>>> {
        let get = sqlx::query_as!(Factory, "SELECT * FROM \"Factory\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get Factory");
        Ok(Json(get))
    }

    #[oai(path = "/factory", method = "post")]
    async fn post_factory(
        &self,
        pool: Data<&PgPool>,
        input: Json<Factory>,
    ) -> Result<Json<String>> {
        match sqlx::query("INSERT INTO \"Factory\" (id, customer_id, name) values ($1, $2, $3)")
            .bind(&input.id)
            .bind(&input.customer_id)
            .bind(&input.name)
            .execute(pool.0)
            .await
        {
            Ok(x) => {
                println!("Inserted Factory");
                dbg!(x);
                Ok(Json("ok".to_string()))
            }
            Err(e) => {
                println!("Failed to insert Factory: {}", e);
                Err(InternalServerError(e))
            }
        }
    }
}
