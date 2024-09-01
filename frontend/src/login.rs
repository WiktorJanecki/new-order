use anyhow::{bail, Result};
use leptos::*;
use leptos_meta::*;
use leptos_router::use_navigate;
use reqwest::StatusCode;
use serde_json::json;
use thaw::*;

use crate::API_PATH;

#[component]
pub fn LoginView() -> impl IntoView {
    let login = create_rw_signal("".to_owned());
    let password = create_rw_signal("".to_owned());
    let messages = use_message();

    let callback_login = move |_| {
        spawn_local(login_request_safe(login, password, messages));
    };

    view! {
        <Space class="fullheight" align=SpaceAlign::Center
         justify=SpaceJustify::Center>
            <Style>"
                *{
                    box-sizing: border-box;
                }
                body,html{
                    margin:0;
                    height:100%;
                }
                .fullheight {
                    height:calc(100% - 160px );
                }
                .card{
                    margin-right:10px;
                }
            "
            </Style>
            <Card class="card" title="Logowanie">
                <Space vertical=true>
                    <Input placeholder="Login" value=login/>
                    <Input placeholder="Hasło" variant=InputVariant::Password value=password/>
                    "" // adds 5px margin
                    <Button on_click=callback_login block=true size=ButtonSize::Medium variant=ButtonVariant::Primary>"Zaloguj"</Button>
                </Space>
            </Card>
        </Space>
    }
}

pub async fn login_request_safe(
    login: RwSignal<String>,
    password: RwSignal<String>,
    messages: MessageInjection,
) {
    if let Err(e) = login_request(login, password).await {
        messages.create(e.to_string(), MessageVariant::Error, Default::default());
    }
}

pub async fn login_request(login: RwSignal<String>, password: RwSignal<String>) -> Result<()> {
    let payload = json!({
        "login": login.get_untracked(),
        "password": password.get_untracked()
    });
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/login", API_PATH))
        .fetch_credentials_include()
        .json(&payload)
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        let e = res.text().await?;
        bail!(e);
    }
    let nav = use_navigate();
    nav("/", Default::default());
    Ok(())
}
