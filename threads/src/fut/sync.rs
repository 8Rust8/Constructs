pub mod future_plays {

    const NUM: usize = 1000;

    pub fn sync_fut() {
        // what ever we try this code will  send first and then recieve because we are using sync version of std::net Sockets
        // sleep version of std is also sync hence it won't release the thread
        let mut buffer = [0; 256];
        let mut count = 0;

        let receiver = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
        let sender = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();

        let sender_fut = async {
            for _ in 0..NUM {
                let _ = sender.send_to(b"Hello world!!", receiver.local_addr().unwrap());
                println!("Sending");
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        };

        let receiver_fut = async {
            for _ in 0..NUM {
                let _ = receiver.recv_from(&mut buffer).unwrap();
                count += 1;
                println!("receiving :: {count}");
            }
        };

        // the future crate has inbuild executor which can be used to block and run async threads inside main

        // it will execute them one by one - first sender fut and then revieving
        // futures::executor::block_on(async {
        //     sender_fut.await;
        //     receiver_fut.await;
        // });
        // to make then run cocurrently we can use the join macro
        futures::executor::block_on(async {
            // the join macro will let then run cocurrently :) , but the problem is std::net protocall its sync
            futures::join!(sender_fut, receiver_fut);
        });
    }

    pub fn async_fut() {
        // we need to change async_std to get async udpsocket amd we need to make sure they are captured before loop hence using executor to get them
        // we need async version of timer too then only send-recieve will work together
        let mut buffer = [0; 256];
        let mut send_count = 0;
        let mut recv_count = 0;

        let receiver = futures::executor::block_on(async {
            async_std::net::UdpSocket::bind("127.0.0.1:0")
                .await
                .unwrap()
        });

        let sender = futures::executor::block_on(async {
            async_std::net::UdpSocket::bind("127.0.0.1:0")
                .await
                .unwrap()
        });

        let sender_fut = async {
            for _ in 0..NUM {
                let _ = sender
                    .send_to(b"Hello world!!", receiver.local_addr().unwrap())
                    .await;
                send_count += 1;
                println!("sending :: {send_count}");
                // we need async sleep
                // we need this so that this sender_fut will be waiting and then the thread will go to receiver_fut
                // we need to await it also so that the thread moves on
                futures_timer::Delay::new(std::time::Duration::from_millis(10)).await;
                // std::thread::sleep(std::time::Duration::from_millis(1));
            }
        };

        let receiver_fut = async {
            for _ in 0..NUM {
                let _ = receiver.recv_from(&mut buffer).await.unwrap();
                recv_count += 1;
                println!("receiving :: {recv_count}");
            }
        };

        futures::executor::block_on(async {
            futures::join!(sender_fut, receiver_fut);
        });
    }
}
