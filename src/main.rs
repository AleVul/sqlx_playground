use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[derive(sqlx::Type)]
#[sqlx(transparent)]
struct Email(String);

#[actix_web::main]
async fn main() -> std::result::Result<(), sqlx::Error> {
    dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test")
        .await?;

    let email = Email("foo".to_owned());

    let result = sqlx::query!("INSERT INTO foo (email) VALUES ($1)", email)
        .execute(&pool)
        .await;

    Ok(())
}
