use router::init_router;

mod login;
mod router;
mod stamp;

// 環境変数のチェック
macro_rules! check_env {
    ($name:literal) => {
        if std::env::var($name).is_err() {
            eprintln!("{} is not set", $name);
            std::process::exit(1);
        }
    };
}

#[tokio::main]
async fn main() {
    check_env!("CLIENT_ID");
    check_env!("EMAIL");
    check_env!("PASSWORD");
    check_env!("POSTGRES_URL");

    let router = init_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
