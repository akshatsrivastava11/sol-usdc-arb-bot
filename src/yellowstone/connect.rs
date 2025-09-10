use std::collections::HashMap;

use anyhow::Result;
use futures::{lock::Mutex, task::ArcWake};
use futures_util::StreamExt;
use tracing_subscriber::field::MakeExt;
use tungstenite::http::Error;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient, Interceptor};
use yellowstone_grpc_proto::geyser::{
    SubscribeRequest, SubscribeRequestFilterAccounts, SubscribeRequestFilterAccountsFilter,
    SubscribeRequestFilterAccountsFilterMemcmp, subscribe_request_filter_accounts_filter::Filter,
    subscribe_request_filter_accounts_filter_memcmp, subscribe_update::UpdateOneof,
};
use std::sync::Arc;
use crate::{raydium_clmm::PoolState, raydium_math::{sqrt_price_math::get_next_sqrt_price_from_input, swap_math::compute_swap_step}, DEXstuct};

pub async fn connect(){
    println!("Connect triggered");
      let dex_struct = Arc::new(Mutex::new(DEXstuct {
        token_in: 064,
        token_out: 064,
    }));
    let tls_config = ClientTlsConfig::new().with_native_roots();
    if let Ok(mut client) =
        GeyserGrpcClient::build_from_shared("https://solana-yellowstone-grpc.publicnode.com:443")
            .unwrap()
            .keep_alive_while_idle(true)
            .tls_config(tls_config)
            .unwrap()
            .connect()
            .await
    {
        let mut accounts: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();

        let filter = SubscribeRequestFilterAccounts {
            owner: vec![],                                                             // TODO
            account: vec!["3ucNos4NbumPLZNWztqGHNFFgkHeRMBQAVemeeomsUxv".to_string()], // TODO
            ..Default::default()
        };

        accounts.insert("client".to_string(), filter);
        let request = SubscribeRequest {
            accounts: accounts,
            ..Default::default()
        };
        let (_tx, mut stream) = client.subscribe_with_request(Some(request)).await.unwrap();

        while let Some(message) = stream.next().await {
            // println!("Message is {:?}", message);
            match message{
                Ok(r)=>{
                    if let Some(UpdateOneof::Account(r))=r.update_oneof{
                        // println!("Update one off account is {:?}",r);
                        if let Some(account)=r.account{
                            // println!("inside block account is {:?} ",account);
                            let pool:PoolState=bincode::deserialize(&account.data[8..]).unwrap();
                            // println!("The pool is {:?}",pool);
                            let sqrt_price_x64=pool.sqrt_price_x64;
                            let liquidity=pool.liquidity;
                            let amount_in:u64=1*1_000_000_000;
                            let zero_for_one=false;
                            let is_base_input=false;
                            let sqrt_price=get_next_sqrt_price_from_input(sqrt_price_x64, liquidity, amount_in, zero_for_one);
                            // println!("The square root price is {:?}",sqrt_price);
                            let swap=compute_swap_step(sqrt_price, sqrt_price_x64, liquidity, amount_in, 0, is_base_input, zero_for_one);
                            let mut dex_struct=dex_struct.lock().await;
                            let swap=swap.unwrap();
                            dex_struct.token_in=swap.amount_in;
                            dex_struct.token_out=swap.amount_out;
                        
                            println!("The token in is {}",dex_struct.token_in);
                            println!("The token out is {}",dex_struct.token_out);

                        };
                    }
                }
                Err(e)=>{
                    eprintln!("An error occured :{}",e);
                    
                }
            }
        }
    };
}
