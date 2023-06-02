use std::path::Path;

use tokio::io::AsyncBufReadExt;

use crate::Auth;

pub async fn update_auth<P: AsRef<Path>>(path: P) -> tokio::io::Result<()> {
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());

    let mut simple_saml_auth_token = String::new();
    println!("Please enter SimpleSAMLAuthToken:");
    stdin.read_line(&mut simple_saml_auth_token).await?;

    let mut simple_saml_session_id = String::new();
    println!("Please enter SimpleSAMLSessionID:");
    stdin.read_line(&mut simple_saml_session_id).await?;

    let mut sse_pair = (String::new(), String::new());
    println!("Please enter the name of the SSE pair:");
    stdin.read_line(&mut sse_pair.0).await?;
    println!("Please enter the value of the SSE pair:");
    stdin.read_line(&mut sse_pair.1).await?;

    tokio::fs::write(path.as_ref().join("auth.ron"), ron::to_string(&Auth {
        simple_saml_auth_token: simple_saml_auth_token.trim().to_string(),
        simple_saml_session_id: simple_saml_session_id.trim().to_string(),
        sse_pair: (sse_pair.0.trim().to_string(), sse_pair.1.trim().to_string()),
    }).unwrap()).await?;

    Ok(())
}