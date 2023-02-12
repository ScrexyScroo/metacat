use google_drive3::{hyper, hyper_rustls, oauth2, DriveHub};
use oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use serde_derive::Deserialize;
use serde_derive::Serialize;

static DRIVE_ID: &str = "0AC8Iw2zWuOj0Uk9PVA";

// * Icons for discord embeds
pub static MKV_ICON: &str = "https://static.vecteezy.com/system/resources/previews/009/130/185/non_2x/mkv-logo-mkv-letter-mkv-letter-logo-design-initials-mkv-logo-linked-with-circle-and-uppercase-monogram-logo-mkv-typography-for-technology-business-and-real-estate-brand-vector.jpg";
pub static FLAC_ICON: &str =
    "https://upload.wikimedia.org/wikipedia/commons/f/f3/FLAC_logo_transparent.png";
pub static CAT_ICON: &str =
    "http://img2.chinadaily.com.cn/images/201909/10/5d76f03ea310cf3e979b79c5.jpeg";
pub static MP4_ICON: &str = "https://logowik.com/content/uploads/images/962_mp4video.jpg";
pub static OPUS_ICON: &str =
    "https://www.pngitem.com/pimgs/m/200-2008269_opus-codec-icon-hd-png-download.png";

// * Acquired enough skill to atleast make this run in tokio
pub async fn get_gdrive_changes() -> Option<Root> {
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
        .expect("Issue getting start page token");

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

    let clean_root = serde_json::from_str::<Root>(json_str.as_str()).unwrap();

    match clean_root.changes.is_empty() {
        true => return None,
        false => {
            println!("{:#?}", clean_root);
            return Some(clean_root);
        }
    }
}

impl Change {
    pub fn get_file_name(&self) -> Option<&str> {
        match self.file.is_some() {
            true => Some(
                self.file
                    .as_ref()
                    .expect("Failed to get file name")
                    .name
                    .as_str(),
            ),
            false => None,
        }
    }

    pub fn get_mime_type(&self) -> &str {
        self.file
            .as_ref()
            .expect("Failed to get mime type")
            .mime_type
            .as_str()
    }

    pub fn get_file_id(&self) -> &str {
        self.file
            .as_ref()
            .expect("Failed to get file id")
            .id
            .as_str()
    }
}

// * Made using https://transform.tools/json-to-rust-serde
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub changes: Vec<Change>,
    pub kind: String,
    pub new_start_page_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub change_type: String,
    pub drive: Option<Drive>,
    pub drive_id: Option<String>,
    pub kind: String,
    pub removed: bool,
    pub team_drive: Option<TeamDrive>,
    pub team_drive_id: Option<String>,
    pub time: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub file: Option<File>,
    pub file_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drive {
    pub id: String,
    pub kind: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamDrive {
    pub id: String,
    pub kind: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub drive_id: String,
    pub id: String,
    pub kind: String,
    pub mime_type: String,
    pub name: String,
    pub team_drive_id: String,
}

// let mut clean_json: Value = serde_json::from_str(&json_str)
//     .expect("Issue extracting value out of json_str in get_gdrive_changes");

// clean_json.skip_null_and_empty();

// println!(
//     "Serialized to rust type: {:?}",
//     serde_json::from_str::<Root>(json_str.as_str()).unwrap()
// );

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
