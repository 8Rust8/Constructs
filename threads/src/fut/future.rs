#![allow(unused)]

pub mod fut_test {

    use async_std::io::ReadExt;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
        process,
    };

    pub async fn fut() -> Result<(), Box<dyn std::error::Error>> {
        // create a async/fut block
        let listener = TcpListener::bind("127.0.0.1:8080").await?;

        loop {
            let (mut stream, address) = listener.accept().await?;

            tokio::spawn(async move {
                let mut buffer = [0; 20];
                loop {
                    // read
                    let n = match stream.read(&mut buffer).await {
                        Ok(n) => {
                            if n == 0 {
                                return;
                            } else {
                                n
                            }
                        }
                        Err(e) => {
                            eprintln!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };
                    // weite
                    dbg!(buffer);

                    //stream.write_all(&mut buffer[0..n]).await;
                }
            });
        }
        // make a db call from inside it
        // create two treads and
        // call the async/fut block 30 times
        //todo!();
    }
}
