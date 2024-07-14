mod login;

#[tokio::main]
async fn main() {
    let client = login::login().await;
}
