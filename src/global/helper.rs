use tokio::{
    io::{self, AsyncReadExt},
    sync::watch::Sender,
    task::{self, JoinHandle},
};

pub async fn quit_task_handler(shutdown_tx: Sender<bool>) -> JoinHandle<()> {
    task::spawn(async move {
        let mut stdin = io::stdin();
        let mut input = [0u8; 2];
        loop {
            if let Ok(n) = stdin.read_exact(&mut input).await {
                if n == 2 && input[0] == b'q' && input[1] == b'\n' {
                    let _ = shutdown_tx.send(true);
                    break;
                }
            }
        }
    })
}
