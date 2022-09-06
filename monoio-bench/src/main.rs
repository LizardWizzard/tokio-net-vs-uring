use common_bench::{ADDRESS, RESPONSE};
use monoio::{io::AsyncWriteRent, net::TcpListener};
use std::io;

fn main() -> io::Result<()> {
    monoio::start::<monoio::IoUringDriver, _>(async {
        let listener = TcpListener::bind(ADDRESS)?;

        loop {
            let (mut stream, _) = listener.accept().await?;

            monoio::spawn(async move {
                let (result, _) = stream.write(RESPONSE).await;

                if let Err(err) = result {
                    eprintln!("Client connection failed: {}", err);
                }
            });
        }
    })
}
