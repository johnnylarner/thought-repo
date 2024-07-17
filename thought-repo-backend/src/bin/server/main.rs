use thought_backend::config::Config;
use thought_backend::domain::blog::service::Service;
use thought_backend::inbound::http::{HttpServer, HttpServerConfig};
use thought_backend::outbound::dev_to_publisher::DevToPublisher;
use thought_backend::outbound::email_client::EmailClient;
use thought_backend::outbound::sqlite::Sqlite;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    // A minimal tracing middleware for request logging.
    tracing_subscriber::fmt::init();

    let sqlite = Sqlite::new(&config.database_url).await?;

    let publisher = DevToPublisher;
    let email_client = EmailClient::new();
    let blog_service = Service::new(sqlite, publisher, email_client);

    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    let http_server = HttpServer::new(blog_service, server_config).await?;
    http_server.run().await
}
