use futures::{StreamExt, TryStreamExt};
use sqlx::mysql::{
    MySqlArguments, MySqlConnectOptions, MySqlPoolOptions, MySqlQueryResult, MySqlRow,
    MySqlTransactionManager,
};
use sqlx::{database, Column, Connection, Database, Error, Executor, MySql, Pool, Transaction};
use sqlx::{MySqlPool, Row};
use std::any;
use std::collections::HashMap;
use std::process::id;
use std::ptr::null;

use serde::{Deserialize, Serialize};
use sqlx::database::HasArguments;
use sqlx::query::Query;

pub trait Row2Bean<R> {
    fn from_row(row: &R) -> Self;
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct User {
    pub id: Option<i64>,
    pub name: Option<String>,
}

impl Row2Bean<sqlx::mysql::MySqlRow> for User {
    fn from_row(row: &MySqlRow) -> Self {
        let mut r = Self::default();
        let _ok = row.columns().iter().for_each(|x| {
            // id
            if x.name().to_string() == "id".to_string() {
                r.id = Some(row.get("id"));
            } else {
                r.id = None;
            }
            // name
            if x.name().to_string() == "name".to_string() {
                r.name = Some(row.get("name"));
            } else {
                r.name = None;
            }
        });
        r
    }
}

pub async fn cnt() -> anyhow::Result<MySqlPool> {
    let url = "mysql://root:password@127.0.0.1/test";
    // let mut pool: Pool<MySql> = MySqlPool::connect(url).await?;
    let mut pool: MySqlPool = MySqlPool::connect(url).await?;
    // let pool = MySqlPool::connect_with(MySqlConnectOptions::new()).await?;
    Ok(pool)
}

#[tokio::test]
async fn my_1() -> anyhow::Result<()> {
    let conn = self::cnt().await?;
    let mut tx: Transaction<MySql> = conn.begin().await?;

    // println!("-----------{:#?}-----------", pool);
    //-------------------------------------
    let row: (i64,) = sqlx::query_as("SELECT ?")
        .bind(100_i64)
        .fetch_one(&conn)
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT ?")
        .bind(100_i64)
        .fetch_one(&mut tx)
        .await?;
    //
    println!("-----------{:#?}-----------", row);

    Ok(())
}

#[tokio::test]
async fn my_2() -> anyhow::Result<()> {
    let conn = self::cnt().await?;
    // println!("-----------{:#?}-----------", pool);
    //-------------------------------------
    let r: Vec<MySqlRow> = sqlx::query("SELECT * from t").fetch_all(&conn).await?;
    println!("-----------{:#?}-----------", r);

    Ok(())
}

#[tokio::test]
async fn my_query_as_1() -> anyhow::Result<()> {
    let conn = self::cnt().await?;
    let mut r = sqlx::query("SELECT * from t ")
        .map(|row| User::from_row(&row))
        .fetch(&conn);

    let mut l: Vec<User> = Vec::new();
    while let Ok(data) = r.try_next().await {
        if data.is_none() {
            break;
        }
        l.push(data.unwrap().clone());
    }
    //-------------------------------------
    println!("-----------{l:#?}-----------");

    Ok(())
}

#[tokio::test]
async fn my_find_many_1() -> anyhow::Result<()> {
    let conn = self::cnt().await?;
    let l: Vec<MySqlRow> = sqlx::query("SELECT * from t").fetch_all(&conn).await?;

    let r: Vec<User> = l.iter().map(|row| User::from_row(row)).collect();
    println!("-----------{r:#?}-----------");

    Ok(())
}

fn qry<'a>(user: User) -> Query<'a, MySql, MySqlArguments> {
    sqlx::query("insert into t(id,name)values(?,?)")
        .bind(user.id.unwrap())
        .bind(None::<String>)
}

#[tokio::test]
async fn my_insert_1() -> anyhow::Result<()> {
    let conn = self::cnt().await?;

    let user = User {
        id: Some(105),
        name: Some("abdeefh".to_string()),
    };

    //  user::insert_exp()
    // ta_name()
    // fd_name()
    let qry = sqlx::query("insert into t(id,name)values(?,?)")
        .bind(user.id.unwrap())
        .bind(None::<String>);
    let qry = self::qry(user);

    // let a = DbValue::new();

    let r = conn.execute(qry).await;
    match r {
        Ok(v) => {
            //todo verify pk is multiple columns
            let id = v.last_insert_id();
            let z = v.rows_affected();
            println!("-----------last_id: {id} effected: {z}-----------");
        }
        e @ _ => {
            println!("---{e:#?}---");
        }
    }

    // println!("-----------{r:#?}-----------");

    Ok(())
}
