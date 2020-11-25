use rand::{distributions::Alphanumeric, Rng};

pub fn emit_log(context: &str, session: &str, msg: &str) {
    let log_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    println!("{{\"logID\":\"edgeml{}\",\"source\":\"{}\",\"context\":\"{}\",\"session\":\"{}\",\"msg\":\"{}\"}}", std::env::var("FASTLY_HOSTNAME").unwrap_or_default(), log_id, context, session, msg);
}
