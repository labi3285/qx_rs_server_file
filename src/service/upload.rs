use std::fs;
use std::path::Path;

use axum::body::Bytes;
use axum::extract::Multipart;

use qx_rs_server::util::uuid;

use qx_rs_server::err::{Error, Result};
use qx_rs_server::env;


pub async fn upload_to_path(mut multipart: Multipart) -> Result<String>
{
    let app_public_url = env::str("APP.PUBLIC_URL")?;
    let upload_path = env::str("UPLOAD_FILE.PATH")?;
    let public_path = env::str("UPLOAD_FILE.PUBLIC_PATH")?;
    let public_url: String = format!("{}{}", app_public_url, public_path);
    let mut domain: Option<String> = None;
    let mut file_bytes: Option<Bytes> = None;
    let mut ext: Option<String> = None;
    while let Some(field) = multipart.next_field().await.map_err(|err| {
        tracing::error!("{}", err);
        Error::Request(format!("upload_to_path failed:{:?}", err))
    })? {
        let key = field.name().unwrap().to_string();
        if key == "domain" {
            let value = field.text().await.map_err(|err| {
                tracing::error!("{}", err);
                Error::Request(format!("upload_to_path reading domain failed:{:?}", err))
            })?;
            domain = Some(value);
        } else if key == "file" {
            let name = &field.file_name();
            if let Some(v) = name {
                let comps = v.split(".").filter(|s| !s.is_empty()).map(|e| e.to_string()).collect::<Vec<String>>();
                if comps.len() > 1 {
                    let s = &comps[comps.len() -1];
                    ext = Some(s.clone())
                }
            }
            let bytes = field.bytes().await.map_err(|err| {
                tracing::error!("{}", err);
                Error::Request(format!("upload_to_path parsing file failed:{:?}", err))
            })?;
            file_bytes = Some(bytes);
        }
    }

    if let Some(file) = file_bytes {
        let mut upload_path = Path::new(&upload_path).to_path_buf();
        if let Some(v) = &domain {
            upload_path = upload_path.join(&v);
        }
        fs::create_dir_all(&upload_path).map_err(|err| {
            tracing::error!("{}", err);
            Error::Request(format!("upload_to_path create_dir_all failed:{:?}", err))
        })?;
        let mut file_name = format!("{}", uuid::v4());
        if let Some(ext) = ext {
            file_name = format!("{}.{}", file_name, ext);
        }
        let file_path = &upload_path.join(&file_name);
        fs::write(&file_path, file).map_err(|err| {
            tracing::error!("{}", err);
            Error::Request(format!("upload_to_path write failed:{:?}", err))
        })?;
        if let Some(v) = &domain {
            Ok(format!("{}/{}/{}", public_url, v, file_name))
        } else {
            Ok(format!("{}/{}", public_url, file_name))
        }
    } else {
        tracing::error!("upload_to_path file_bytes empty");
        Err(Error::Request(format!("upload_to_path file_bytes empty")))
    }
}


