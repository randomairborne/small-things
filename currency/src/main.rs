use axum::{extract::Json, routing::get};
use parking_lot::Mutex;
async fn find(
    _: Authorization
)

async fn submit(
    _: Authorization,
    Json(payload): Json<Transaction>
)

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", get(find).post(submit));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct Entry {
    hash: String,
    ledger: std::collections::HashMap<String, Entity>,
    diff: Transaction,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct Transaction {
    sender: Entity,
    receiver: Entity,
    change: u64,
}
	
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct Entity {
    r#type: EntityType,
    id: u128,
    balance: i128,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
enum EntityType {
    Company,
    Individual,
    Government,
}

struct Authorization;

#[async_trait::async_trait]
impl<T: Send> axum::extract::FromRequest<T> for Authorization {
    type Rejection = WebServerError;
    async fn from_request(
        req: &mut axum::extract::RequestParts<T>,
    ) -> Result<Self, Self::Rejection> {
        let headers = req.headers().ok_or(WebServerError::MissingHeaders)?;

        let username = String::from_utf8(
            headers
                .get("username")
                .ok_or(WebServerError::BadRequest)?
                .as_bytes()
                .into(),
        )
        .map_err(|_| WebServerError::BadRequest)?;
        let password = String::from_utf8(
            headers
                .get("password")
                .ok_or(WebServerError::BadRequest)?
                .as_bytes()
                .into(),
        )
        .map_err(|_| WebServerError::BadRequest)?;
        let config = crate::CONFIG.get().ok_or(WebServerError::ConfigNotFound)?;
        let result = sha2::Sha256::digest(password)
            .into_iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>();
        let existing_hash = config.users.get(&username).map(String::as_str);
        tracing::debug!("Attempting to log in user {}", username);
        tracing::trace!(
            "supplied password hash is {}, correct password hash is {}",
            result,
            existing_hash.unwrap_or("(failed to get proper password hash)")
        );

        if existing_hash.map_or(false, |user| user == result) {
            Ok(Self)
        } else {
            Err(WebServerError::IncorrectAuth)
        }
    }
}


pub fn read_bincode(filename: &String) -> Result<Arc<Mutex<Vec<Storage>>>> {
    let target_path = std::fs::Path::new(filename);
    if !target_path.exists() {
        let new_map: Vec<Storage> = Vec::new();
        let encoded: Vec<u8> = bincode::serialize(&new_map).unwrap();
        std::fs::write(target_path, encoded)
            .expect("failed to write to database file: check permissions");
        return Vec::new();
    }

    let f = std::fs::OpenOptions::new()
        .read(true)
        .open(target_path)
        .expect("Failed to open file");

    let deserialized: dashmap::DashMap<String, String> =
        bincode::deserialize_from(f).expect("Failed to deserialize database");
    deserialized
}

pub fn save_bincode<P: AsRef<Path>>(
    filename: P,
    map: &dashmap::DashMap<String, String>,
) -> Result<(), bincode::Error> {
    let mut f = std::fs::OpenOptions::new().write(true).read(false).open(filename)?;
    let encoded: Vec<u8> = bincode::serialize(&map)?;
    f.write_all(&encoded)?;
    Ok(())
}
