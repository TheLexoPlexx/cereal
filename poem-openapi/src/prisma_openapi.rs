// ------------------------------------------------------
// Generated with github.com/thelexoplexx/prisma-generator-poem-openapi
// Do not edit manually.
// ------------------------------------------------------`

use poem::{web::Data, Result};
use poem_openapi::{
    payload::{Json, PlainText},
    Object, OpenApi,
};
use sqlx::PgPool;

#[derive(Object)]
pub struct Plant {
    id: String,
    label: String,
    order_number: i32,
}

#[derive(Object)]
pub struct Serialnumberpart {
    serial: String,
    part_id: String,
    plant_id: Option<String>,
    parent_id: Option<String>,
    manufacturing_date: Option<i32>,
    shipping_date: Option<i32>,
    installation_date: Option<i32>,
}

#[derive(Object)]
pub struct Part {
    part_number: String,
}

#[derive(Object)]
pub struct Project {
    id: String,
    project_number: i32,
}

#[derive(Object)]
pub struct Order {
    order_number: i32,
    project_id: String,
}

#[derive(Object)]
pub struct Final_Customer {
    id: String,
    sap_id: String,
    name: String,
    alias: Vec<String>,
}

#[derive(Object)]
pub struct Prime_Contractor {
    id: String,
    sap_id: String,
}

#[derive(Object)]
pub struct Factory {
    id: String,
    customer_id: String,
    name: String,
}

pub struct API;

#[OpenApi]
impl API {
    #[oai(path = "/plant", method = "get")]
    async fn get_plant(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Plant>>> {
        let get = sqlx::query_as!(Plant, "SELECT * FROM \"plant\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get plant");
        Ok(Json(get))
    }

    #[oai(path = "/serialnumberpart", method = "get")]
    async fn get_serialnumberpart(
        &self,
        pool: Data<&PgPool>,
    ) -> Result<Json<Vec<Serialnumberpart>>> {
        let get = sqlx::query_as!(Serialnumberpart, "SELECT * FROM \"serialnumberpart\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get serialnumberpart");
        Ok(Json(get))
    }

    #[oai(path = "/part", method = "get")]
    async fn get_part(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Part>>> {
        let get = sqlx::query_as!(Part, "SELECT * FROM \"part\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get part");
        Ok(Json(get))
    }

    #[oai(path = "/project", method = "get")]
    async fn get_project(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Project>>> {
        let get = sqlx::query_as!(Project, "SELECT * FROM \"project\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get project");
        Ok(Json(get))
    }

    #[oai(path = "/order", method = "get")]
    async fn get_order(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Order>>> {
        let get = sqlx::query_as!(Order, "SELECT * FROM \"order\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get order");
        Ok(Json(get))
    }

    #[oai(path = "/final_customer", method = "get")]
    async fn get_final_customer(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Final_Customer>>> {
        let get = sqlx::query_as!(Final_Customer, "SELECT * FROM \"final_customer\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get final_customer");
        Ok(Json(get))
    }

    #[oai(path = "/prime_contractor", method = "get")]
    async fn get_prime_contractor(
        &self,
        pool: Data<&PgPool>,
    ) -> Result<Json<Vec<Prime_Contractor>>> {
        let get = sqlx::query_as!(Prime_Contractor, "SELECT * FROM \"prime_contractor\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get prime_contractor");
        Ok(Json(get))
    }

    #[oai(path = "/factory", method = "get")]
    async fn get_factory(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Factory>>> {
        let get = sqlx::query_as!(Factory, "SELECT * FROM \"factory\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get factory");
        Ok(Json(get))
    }
}
