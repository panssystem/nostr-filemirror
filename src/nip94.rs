use std::arch::x86_64::_CMP_TRUE_UQ;

use nostr_bot::*;
// use super::State;

pub fn get_nip94_handler() -> Handler<State<Vec<String>>> {
    let filter = Filter::new_kind(1063
    );
    let sub = Subscription::new("nip94".to_string(),vec![filter]);
Handler::new(
        sub,
        wrap!(handle_kind94))
}

fn handle_kind94(evt: Event, state: State<Vec<&str>>) -> (){
    for x in evt.tags.iter() {
        match x
            .get(0)
            .expect("tags must have at least two entries")
            .as_str()
        {
            "i" => process_hash(x),
            "magnet" => process_magnet(x),
            _ => println!("ignoring tag {:?}", x),
        }
    }
}
fn process_hash(tag: &Vec<String>) {
    println!("processing hash for {:?}", tag);
}

fn process_magnet(tag: &Vec<String>) {
    println!("processing magnet for {:?}", tag);
}
