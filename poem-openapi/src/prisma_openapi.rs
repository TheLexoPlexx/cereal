// ------------------------------------------------------
// Generated with github.com/thelexoplexx/prisma-generator-poem-openapi
// Do not edit manually.
// ------------------------------------------------------

use chrono::{DateTime, FixedOffset};
use poem::{web::Data, Result};
use poem_openapi::{payload::Json, Object, OpenApi};
use sqlx::PgPool;

#[derive(Object)]
pub struct System {
    id: String,
    label: String,
    order_number: i32,
}

#[derive(Object)]
pub struct Serialnumberpart {
    serial: String,
    edv_nummer: String,
    system_id: Option<String>,
    parent_id: Option<String>,
    manufacturing_date: Option<DateTime<FixedOffset>>,
    shipping_date: Option<DateTime<FixedOffset>>,
    installation_date: Option<DateTime<FixedOffset>>,
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
pub struct FinalCustomer {
    id: String,
    sap_id: String,
    name: String,
    alias: Option<Vec<String>>,
}

#[derive(Object)]
pub struct PrimeContractor {
    id: String,
    sap_id: String,
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

    #[oai(path = "/project", method = "get")]
    async fn get_project(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Project>>> {
        let get = sqlx::query_as!(Project, "SELECT * FROM \"Project\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get Project");
        Ok(Json(get))
    }

    #[oai(path = "/order", method = "get")]
    async fn get_order(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Order>>> {
        let get = sqlx::query_as!(Order, "SELECT * FROM \"Order\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get Order");
        Ok(Json(get))
    }

    #[oai(path = "/finalcustomer", method = "get")]
    async fn get_finalcustomer(&self, pool: Data<&PgPool>) -> Result<Json<Vec<FinalCustomer>>> {
        let get = sqlx::query_as!(FinalCustomer, "SELECT * FROM \"FinalCustomer\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get FinalCustomer");
        Ok(Json(get))
    }

    #[oai(path = "/primecontractor", method = "get")]
    async fn get_primecontractor(&self, pool: Data<&PgPool>) -> Result<Json<Vec<PrimeContractor>>> {
        let get = sqlx::query_as!(PrimeContractor, "SELECT * FROM \"PrimeContractor\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get PrimeContractor");
        Ok(Json(get))
    }

    #[oai(path = "/factory", method = "get")]
    async fn get_factory(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Factory>>> {
        let get = sqlx::query_as!(Factory, "SELECT * FROM \"Factory\"")
            .fetch_all(pool.0)
            .await
            .expect("Failed to get Factory");
        Ok(Json(get))
    }
}
