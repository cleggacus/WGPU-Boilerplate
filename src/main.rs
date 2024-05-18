pub mod app;
pub mod renderer;
pub mod ui;

use app::App;

#[tokio::main]
async fn main() {
    App::new()
        .await
        .run();
}
