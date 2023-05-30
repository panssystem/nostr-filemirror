use nostr_bot::*;

mod nip94;


#[tokio::main]
async fn main() {
    init_logger();

    let relays = vec![
        "wss://nostr-pub.wellorder.net",
        "wss://relay.damus.io",
        "wss://relay.nostr.info",
    ];

    let keypair = keypair_from_secret(
        // Your secret goes here
        "0000000000000000000000000000000000000000000000000000000000000001",
    );
    
    let nip94Handler = nip94::get_nip94_handler();

    let state: State<Vec<String>> = wrap_state(vec![]);

    Bot::new(keypair, relays, state)
    .subscription_handler(nip94Handler)
    .run()
    .await
}