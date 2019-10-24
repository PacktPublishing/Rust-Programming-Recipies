#![feature(async_await)]

extern crate d5_futures;

use tokio::net::TcpListener;
use tokio::prelude::*;

use futures::compat::{AsyncRead01CompatExt,AsyncWrite01CompatExt};
use futures::stream::StreamExt;
use futures::future::{FutureExt,TryFutureExt};
use futures::io::AsyncWriteExt;

fn main()->Result<(),failure::Error>{
    //new channel here 
    let listener = TcpListener::bind(&"127.0.0.1:8092".parse()?)?;
    let s_future = listener.incoming()
        .map_err(|e| eprintln!("Accept failure {:?}",e))
        .for_each(|sock| {
            let f03=async move {
                let (s_in,s_out) = sock.split();
                let mut read_stream = d5_futures::ReadStream::new(s_in.compat());

                let mut s_out = s_out.compat();
                while let Some(s) = read_stream.next().await {
                    if s.len() == 0 {
                        return Ok(())
                    }
                    println!("s == {}",s);
                    let s = format!("You said \"{}\"\n",s.trim_end());
                    if let Err(e) = s_out.write_all(s.as_bytes()).await{
                        return Err(e);
                    }

                };
                Ok(())

            };
            tokio::spawn(f03.map_err(|e|eprintln!("Error : {}",e)).boxed().compat());
            Ok(())
        });

    //second future here
    //



    //join futures and run

    tokio::run(s_future);


    Ok(())
}

