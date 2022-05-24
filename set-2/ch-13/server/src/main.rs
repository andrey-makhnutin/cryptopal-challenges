use form_urlencoded;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use warp::Filter;

#[tokio::main]
async fn main() {
    let state = init_server_state();
    let api = make_routes(state);
    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}

fn make_routes(
    state: State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    routes::create_account(state.clone())
        .or(routes::check_role(state))
        .or(routes::index())
}

mod routes {
    use super::handlers;
    use super::State;
    use warp::Filter;

    pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path::end().and(warp::filters::fs::file("index.html"))
    }

    pub fn create_account(
        state: State,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("account")
            .and(warp::post())
            .and(with_state(state))
            .and(warp::body::form())
            .map(handlers::create_account)
    }

    pub fn check_role(
        state: State,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("role")
            .and(warp::get())
            .and(with_state(state))
            .and(warp::cookie("account"))
            .map(handlers::check_role)
    }

    fn with_state(
        state: State,
    ) -> impl Filter<Extract = (State,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || state.clone())
    }
}

mod handlers {
    use super::deserialize_account;
    use super::serialize_account;
    use super::Account;
    use super::AccountRole;
    use super::State;
    use my_cryptopals_lib::aes;
    use my_cryptopals_lib::bytes_to_hex;
    use my_cryptopals_lib::parse_hex_string;
    use std::collections::HashMap;
    use warp::http::Response;
    use warp::http::Result;

    pub fn create_account(
        state: State,
        form: HashMap<String, String>,
    ) -> Result<Response<&'static str>> {
        println!("Received a form {:?}", form);
        let email = form.get("email").unwrap().clone();
        let mut state = state.lock().unwrap();
        state.max_uid += 1;
        let uid = state.max_uid;
        let account = Account {
            email,
            uid,
            role: AccountRole::USER,
        };
        let serialized_account = serialize_account(&account);
        let encrypted_account = aes::encrypt_128_ecb(&serialized_account, &state.encrypt_key);
        let account_hex = bytes_to_hex(&encrypted_account);
        Response::builder()
            .header("Set-Cookie", format!("account={}", account_hex))
            .header("Location", "/role")
            .status(warp::http::StatusCode::SEE_OTHER)
            .body("Welcome\n")
    }

    pub fn check_role(state: State, cookie: String) -> Result<Response<String>> {
        println!("Received cookie {:?}", cookie);
        let account = match parse_account_cookie(state, cookie) {
            Err(err) => {
                return Response::builder()
                    .status(warp::http::StatusCode::BAD_REQUEST)
                    .body(err);
            }
            Ok(acc) => acc,
        };
        Response::builder().body(format!(
            "Hello {}, your role is {}",
            account.email,
            account.role.to_str()
        ))
    }

    fn parse_account_cookie(state: State, cookie: String) -> std::result::Result<Account, String> {
        let state = state.lock().unwrap();
        let encrypted = parse_hex_string(&cookie)?;
        let decrypted = aes::decrypt_128_ecb(&encrypted, &state.encrypt_key)
            .map_err(|err| format!("AES decrypt error: {:?}", err))?;
        println!(
            "Decrypted cookie as {}",
            my_cryptopals_lib::bytes_to_str_or_hex(&decrypted)
        );
        deserialize_account(&decrypted)
    }
}

pub struct ServerState {
    pub max_uid: u32,
    pub encrypt_key: [u8; 16],
}
type State = Arc<Mutex<ServerState>>;

impl ServerState {
    pub fn new() -> Self {
        Self {
            max_uid: 0,
            encrypt_key: rand::random(),
        }
    }
}

fn init_server_state() -> State {
    Arc::new(Mutex::new(ServerState::new()))
}

pub fn serialize_account(account: &Account) -> Vec<u8> {
    form_urlencoded::Serializer::new(String::new())
        .append_pair("email", &account.email)
        .append_pair("uid", &account.uid.to_string())
        .append_pair("role", account.role.to_str())
        .finish()
        .as_bytes()
        .to_vec()
}

pub fn deserialize_account(bytes: &[u8]) -> std::result::Result<Account, String> {
    let mut kv_map: HashMap<String, String> = HashMap::new();
    for (k, v) in form_urlencoded::parse(bytes).into_owned() {
        kv_map.insert(k, v);
    }
    let email = kv_map.get("email").ok_or(String::from("no email"))?;
    let uid = kv_map
        .get("uid")
        .ok_or(String::from("no uid"))
        .and_then(|num| {
            num.parse::<u32>()
                .map_err(|e| format!("Failed to parse uid: {}", e))
        })?;
    let role = kv_map
        .get("role")
        .ok_or(String::from("no role"))
        .and_then(|s| AccountRole::from_str(s))?;

    Ok(Account {
        email: email.clone(),
        uid,
        role,
    })
}

pub enum AccountRole {
    USER,
    ADMIN,
}

impl AccountRole {
    pub fn to_str(self: &Self) -> &'static str {
        match self {
            Self::USER => "user",
            Self::ADMIN => "admin",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "user" => Ok(Self::USER),
            "admin" => Ok(Self::ADMIN),
            other => Err(format!("unknown role {}", other)),
        }
    }
}

pub struct Account {
    pub email: String,
    pub uid: u32,
    pub role: AccountRole,
}
