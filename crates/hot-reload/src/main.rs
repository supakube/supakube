use axum::{extract::Request, response::Html, routing::get, Router};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::Value;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    // Define the address to listen on (0.0.0.0)
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("Listening on {}", addr);
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn handler(req: Request) -> Html<String> {
    let headers = get_headers(&req);
    let jwt = get_jwt(&req);
    let env_vars = get_env_vars();

    let html = format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Hot Reload Info</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
            background-color: #f0f0f0;
        }}
        .container {{
            width: 80%;
            height: 80vh;
            max-width: 800px;
            display: flex;
            flex-direction: column;
        }}
        h1 {{
            text-align: center;
            color: #333;
            margin-bottom: 20px;
        }}
        .tab-container {{
            background: white;
            border-radius: 8px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
            overflow: hidden;
            display: flex;
            flex-direction: column;
            flex-grow: 1;
        }}
        .tab-buttons {{
            display: flex;
            background: #f1f1f1;
        }}
        .tab-button {{
            flex: 1;
            padding: 10px;
            border: none;
            background: none;
            cursor: pointer;
            transition: background-color 0.3s;
        }}
        .tab-button:hover {{
            background-color: #ddd;
        }}
        .tab-button.active {{
            background-color: white;
        }}
        .tab-content {{
            display: none;
            padding: 20px;
            overflow-y: auto;
            flex-grow: 1;
        }}
        .tab-content.active {{
            display: block;
        }}
        pre {{
            white-space: pre-wrap;
            word-wrap: break-word;
            margin: 0;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Hot Reload</h1>
        <div class="tab-container">
            <div class="tab-buttons">
                <button class="tab-button active" onclick="openTab(event, 'headers')">HTTP Headers</button>
                <button class="tab-button" onclick="openTab(event, 'jwt')">JWT Contents</button>
                <button class="tab-button" onclick="openTab(event, 'env')">Environment Variables</button>
            </div>
            <div id="headers" class="tab-content active">
                <pre>{}</pre>
            </div>
            <div id="jwt" class="tab-content">
                <pre>{}</pre>
            </div>
            <div id="env" class="tab-content">
                <pre>{}</pre>
            </div>
        </div>
    </div>

    <script>
        function openTab(evt, tabName) {{
            var i, tabContent, tabButtons;
            tabContent = document.getElementsByClassName("tab-content");
            for (i = 0; i < tabContent.length; i++) {{
                tabContent[i].classList.remove("active");
            }}
            tabButtons = document.getElementsByClassName("tab-button");
            for (i = 0; i < tabButtons.length; i++) {{
                tabButtons[i].classList.remove("active");
            }}
            document.getElementById(tabName).classList.add("active");
            evt.currentTarget.classList.add("active");
        }}
    </script>
</body>
</html>
        "#,
        headers, jwt, env_vars
    );

    Html(html)
}

fn get_headers(req: &Request) -> String {
    req.headers()
        .iter()
        .map(|(name, value)| {
            format!(
                "{}: {}",
                name,
                value.to_str().unwrap_or("Unable to decode value")
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn get_jwt(req: &Request) -> String {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    // This is a dummy secret key. In a real application, you'd use a proper secret.
                    let secret = b"secret";
                    match decode::<Value>(
                        token,
                        &DecodingKey::from_secret(secret),
                        &Validation::default(),
                    ) {
                        Ok(token_data) => {
                            return serde_json::to_string_pretty(&token_data.claims)
                                .unwrap_or_else(|_| "Failed to format JWT".to_string())
                        }
                        Err(_) => return "Invalid JWT".to_string(),
                    }
                }
            }
        }
    }
    "No JWT found".to_string()
}

fn get_env_vars() -> String {
    env::vars()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("\n")
}
