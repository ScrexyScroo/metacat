use google_drive3::api::ChangeList;
use poise::futures_util::future::join_all;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

mod bot;
mod utils;

// * keeping this at 1 gives instant feedback. But can change the value.
static POLL_INTERVAL: u64 = 1;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<ChangeList>(10);

    let bot_task = tokio::task::spawn(bot::bot());

    let watcher_task = tokio::task::spawn(async move {
        // ! Some spaghetti infinite loop - hope google doesn't ban
        // * minor skill issue on my end
        loop {
            let interval = Duration::from_secs(POLL_INTERVAL); // seconds
            let mut next_time = Instant::now() + interval;

            sleep(next_time - Instant::now());
            next_time += interval;

            let change = utils::get_gdrive_changes().await;
            tx.send(change)
                .await
                .expect("Transmission of data out of gdrive thread gone wrong");
        }
    });

    let sync_task = tokio::task::spawn(async move {
        // ! Some spaghetti infinite loop again......
        loop {
            println!("{:?}", rx.recv().await);
        }
    });

    let mut futures = vec![];

    futures.push(bot_task);
    futures.push(watcher_task);
    futures.push(sync_task);

    join_all(futures).await;
}

// avoid vec allocation
// let mut futures = vec![];
// futures.push(bot_task);
// futures.push(watcher_task);

// join_all(futures).await;

// ! Apparently join can only join 2 task?
// join(bot_task, watcher_task, sync_task)
//     .await
//     .0 // I guess we don't care about the other result lol?
//     .expect("Issue in joining tokio tasks in main");

// ! spawn_blocking doesn't work because of  futures.push(watcher_task);
// ! |                                               ---- ^^^^^^^^^^^^ expected opaque type, found a different opaque type
// ! |

// let bot_task = tokio::task::spawn_blocking(|| bot::bot());
// let watcher_task = tokio::task::spawn_blocking(|| utils::gdrive());

// let mut futures = vec![];
// futures.push(bot_task);
// futures.push(watcher_task);

// join_all(futures).await;
// bot::bot().await;
// utils::gdrive().await;
