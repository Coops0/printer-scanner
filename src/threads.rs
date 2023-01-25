use indicatif::ProgressBar;
use tokio::{fs::File, io::AsyncWriteExt, sync::mpsc::UnboundedReceiver};

pub enum ProgressBarMessage {
    Increment,
    Message(String),
    Close,
}

pub async fn progress_bar_thread(amount: u64, mut rec: UnboundedReceiver<ProgressBarMessage>) {
    let pb = ProgressBar::new(amount);
    loop {
        match rec.recv().await {
            Some(m) => match m {
                ProgressBarMessage::Increment => pb.inc(1),
                ProgressBarMessage::Message(m) => pb.println(m),
                ProgressBarMessage::Close => {
                    pb.finish_with_message("prematurely done scanning");
                    return;
                }
            },
            None => return,
        }

        if amount <= pb.position() {
            pb.finish_with_message("finished sending requests");
            return;
        }
    }
}

pub enum AppendMessage {
    Amendment(String),
    Close,
}

pub async fn append_thread(mut rec: UnboundedReceiver<AppendMessage>) {
    let _ = tokio::fs::remove_file("./devices.txt").await;

    let mut f = File::create("./devices.txt").await.unwrap();

    loop {
        match rec.recv().await {
            Some(m) => match m {
                AppendMessage::Amendment(a) => f.write_all(a.as_bytes()).await.unwrap(),
                AppendMessage::Close => return,
            },
            None => return,
        }
    }
}
