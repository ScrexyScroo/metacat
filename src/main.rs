use poise::futures_util::future::join_all;

mod bot;
mod utils;

#[tokio::main]
async fn main() {
    // ! spawn_blocking doesn't work because of  futures.push(watcher_task);
    // ! |                                               ---- ^^^^^^^^^^^^ expected opaque type, found a different opaque type
    // ! |
    // let bot_task = tokio::task::spawn_blocking(|| bot::bot());
    // let watcher_task = tokio::task::spawn_blocking(|| utils::gdrive());

    // let mut futures = vec![];
    // futures.push(bot_task);
    // futures.push(watcher_task);

    // join_all(futures).await;

    let bot_task = tokio::task::spawn(bot::bot());
    let watcher_task = tokio::task::spawn(utils::gdrive());

    let mut futures = vec![];
    futures.push(bot_task);
    futures.push(watcher_task);

    join_all(futures).await;

    // bot::bot().await;
    // utils::gdrive().await;
}
