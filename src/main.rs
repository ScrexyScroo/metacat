use google_drive3::api::ChangeList;
use poise::futures_util::future::join_all;

mod bot;
mod utils;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ChangeList>(10);

    let bot_task = tokio::task::spawn(bot::bot());
    // let watcher_task = tokio::task::spawn(utils::gdrive());

    let watcher_task = tokio::task::spawn(async move {
        // ! Some spaghetti infinite loop - hope google doesn't ban
        // * minor skill issue on my end
        loop {
            let change = utils::get_gdrive_changes().await;
            // println!("{:?}", change);
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
    // println!("{:?}", rx.recv().await); // only prints once

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
