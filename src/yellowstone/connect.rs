use std::collections::HashMap;

use futures_util::StreamExt;
use tracing_subscriber::field::MakeExt;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient, Interceptor};
use yellowstone_grpc_proto::geyser::{
    SubscribeRequest, SubscribeRequestFilterAccounts, SubscribeRequestFilterAccountsFilter,
    SubscribeRequestFilterAccountsFilterMemcmp, subscribe_request_filter_accounts_filter::Filter,
    subscribe_request_filter_accounts_filter_memcmp, subscribe_update::UpdateOneof,
};

pub async fn connect() {
    println!("Connect triggered");
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
            println!("Message is {:?}", message);
        }
    };
}
