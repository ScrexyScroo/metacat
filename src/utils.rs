use google_drive3::{hyper, hyper_rustls, oauth2, DriveHub};
use oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use serde_json::Value;
use serde_json_utils::JsonUtils;

static DRIVE_ID: &str = "0AC8Iw2zWuOj0Uk9PVA";

// * Acquired enough skill to atleast make this run in tokio
pub async fn get_gdrive_changes() -> Value {
    let secret = oauth2::read_application_secret("clientsecret.json")
        .await
        .expect("clientsecret.json failed to load from local storage");

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk("tokencache.json")
        .build()
        .await
        .unwrap();

    let hub = DriveHub::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        auth,
    );

    let (_, start_page_token) = hub
        .changes()
        .get_start_page_token()
        .supports_all_drives(true)
        .drive_id(DRIVE_ID)
        .doit()
        .await
        .unwrap();

    let changes = hub
        .changes()
        .list(start_page_token.start_page_token.unwrap().as_str())
        .supports_all_drives(true)
        .include_items_from_all_drives(true)
        .drive_id(DRIVE_ID)
        .doit()
        .await;

    let (_, change) = changes.expect("Some minor issue in change detection");

    let json_str: String =
        serde_json::to_string(&change).expect("Issue converting json object to string");

    let mut clean_json: Value = serde_json::from_str(&json_str)
        .expect("Issue extracting value out of json_str in get_gdrive_changes");

    clean_json.skip_null_and_empty();
    return clean_json;
}

// println!("--------------------------------------------------------");
// println!("{:?}", change);
// println!("--------------------------------------------------------");

// ! I do not know what request to create?
// let req = Channel::default();
// let changes = hub
//     .changes()
//     .watch(req, start_page_token.start_page_token.unwrap().as_str())
//     .supports_all_drives(true)
//     .restrict_to_my_drive(false)
//     .include_removed(false)
//     .include_items_from_all_drives(true)
//     .include_corpus_removals(false)
//     .drive_id(DRIVE_ID)
//     .doit()
//     .await;

// ! Cannot use this as You cannot really run a Tokio Runtime inside another Tokio Runtime
/*
#[allow(dead_code)]
#[tokio::main]
pub async fn get_hub() -> DriveHub<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
    let secret = oauth2::read_application_secret("clientsecret.json")
        .await
        .expect("clientsecret.json failed to load from local storage");

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk("token_cache.json")
        .build()
        .await
        .unwrap();

    let hub = DriveHub::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        auth,
    );

    return hub;
}

#[tokio::main]
pub async fn get_access_token() -> Result<oauth2::AccessToken, Error> {
    let scopes = &["https://www.googleapis.com/auth/drive.file"];

    match auth.token(scopes).await {
        Ok(token) => {
            println!("The token is {:?}", token);
            return Ok(token);
        }
        Err(e) => {
            println!("error: {:?}", e);
            return Err(e);
        }
    }
}*/
/*

#[allow(unused_variables)]
let result = hub
    .files()
    .list()
    .page_size(1)
    .supports_all_drives(true)
    .include_team_drive_items(true)
    .drive_id(DRIVE_ID)
    .corpora("drive")
    .doit()
    .await;

let (_, file_list) = result.expect("issue");

#[derive(Debug)]
pub struct ClientDetails {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub token: String,
    pub refresh_token: String,
}

impl ClientDetails {
    pub fn new() -> Self {
        ClientDetails {
            client_id: String::from("client id"),
            client_secret: String::from("secret"),
            redirect_uri: String::from("redirect uri"),
            token: String::from(""),
            refresh_token: String::from(""),
        }
    }
}
*/
