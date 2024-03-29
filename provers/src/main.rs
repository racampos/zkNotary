use actix_web::{web, App, HttpServer};
mod discord_prover;
mod simple_prover;
mod twitter_prover;
mod github_prover;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/notarize_simple", web::get().to(simple_prover::notarize))
            .route("/notarize_discord", web::get().to(discord_prover::notarize))
            .route("/notarize_twitter", web::get().to(twitter_prover::notarize))
            .route("/notarize_github", web::get().to(github_prover::notarize))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
