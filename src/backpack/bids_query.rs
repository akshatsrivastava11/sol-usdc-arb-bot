use std::{env, vec};
use dotenv::dotenv;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use solana_sdk::feature_set::raise_block_limits_to_50m;
use tokio_tungstenite::connect_async;
use tungstenite::{connect, Message};
use url::Url;
pub async fn getBestBidAndAsk(){
    dotenv().ok();
    println!("In the getBestBidAndAsk function");
    let wsBpUrl="wss://ws.backpack.exchange/";

    let (mut socket,_reponse)=connect_async(wsBpUrl).await.expect("Cannot connect to the backpack exchange's ws");
    println!("Connected to the backpack exchange's websocket:{:?}",_reponse);
    let (mut write,mut read)=socket.split();
    let subscriberMsg=json!({
        "method":"SUBSCRIBE",
        "params":["depth.SOL_USDC"]
    });
    write.send(Message::Text(subscriberMsg.to_string().into())).await.expect("Failed to write the message");
    println!("Subscribed to the SOL_USDC channel");
   while let Some(msg)=read.next().await{

    match msg {
        Ok(Message::Text(txt))=>{
            println!("Message is: {}",txt);
        }
        Ok(other)=>{println!("Other is {:?}",other)}
        Err(e)=>{
            eprintln!("An error occured during the reading onf the message :{:?}",e)
        }
    };

   }

}

