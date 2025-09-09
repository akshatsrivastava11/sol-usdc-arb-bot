use std::sync::Arc;
mod yellowstone;
use crate::yellowstone::connect;
use tokio::sync::Mutex;
use yellowstone::*;
use yellowstone_grpc_proto::prelude::TokenBalance;

pub struct DEXstuct {
    token_in: u64,
    token_out: u64,
}

pub struct CEXstruct {
    best_bid: u64,
    best_ask: u64,
}

#[tokio::main]
async fn main() {
    //Three threads
    // one for grpc streaming
    // one for streaming api
    // one for main maths

    println!("A SOL/USDC ARB BOT");
    let dex_struct = Arc::new(Mutex::new(DEXstuct {
        token_in: 064,
        token_out: 064,
    }));
    let cex_struct = Arc::new(CEXstruct {
        best_ask: 064,
        best_bid: 064,
    });

    let thread1 = tokio::spawn(async move { connect().await }).await;
}
