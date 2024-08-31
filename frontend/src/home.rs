use anyhow::Result;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use reqwest::StatusCode;
use thaw::*;
use wasm_bindgen::JsValue;

use crate::{Context, API_PATH};

#[component]
pub fn HomeView() -> impl IntoView {
    let messages = use_message();
    let ctx = expect_context::<Context>();
    create_effect(move |_| {
        spawn_local(check_login_safe(ctx.login, messages));
    });
    let logout = |_| {
        let nav = use_navigate();
        nav("/login", Default::default());
    };
    let list = |_| {
        let nav = use_navigate();
        nav("/orders", Default::default());
    };
    let window_size = window()
        .inner_width()
        .unwrap_or(JsValue::from_f64(400.0))
        .as_f64()
        .unwrap_or(400.0);
    let mobile = window_size < 1000f64;
    view! {
        <Style>"
            * {
                box-sizing: border-box;
            }
            body,html{
                margin:0;
                height:100%;
            }
            .card{
               max-width:500px;
               margin-right:10px;
            }
            .btn{
                height:270px;
                width:270px;
            }
            .fullheight {
                height:calc(100% - 40px);
            }
            .stripe{
                width:100vw;
                height:40px;
                background-color:#0078ff;
                left:0;
                top:0px;
            }
        "</Style>
        <div class="stripe"></div>
        <Space class="fullheight" justify=SpaceJustify::Center align=SpaceAlign::Center>
                {if mobile{view!{
                    // on mobile
                    <Space vertical=true>
                        <Button class="btn" block=true>"Nowe Zamówienie"</Button>
                        "" // adds 10px margin
                        ""
                        ""
                        <Button on_click=list class="btn" block=true>"Lista Zamówień"</Button>
                        <Divider/>
                        <Button on_click=logout block=true>"Wyloguj"</Button>
                    </Space>
                }.into_view()}


                else{view!{
                    // on desktop
                    <Space>
                        <Button class="btn" block=true>"Nowe Zamówienie"</Button>
                        <Button on_click=list class="btn" block=true>"Lista Zamówień"</Button>
                        <Button class="btn" on_click=logout block=true size=ButtonSize::Large>"Wyloguj"</Button>
                    </Space>
                }.into_view()}}
        </Space>
    }
}

async fn check_login_safe(login_signal: RwSignal<bool>, messages: MessageInjection) {
    if let Err(e) = check_login(login_signal).await {
        messages.create(e.to_string(), MessageVariant::Error, Default::default());
    }
}

async fn check_login(login_signal: RwSignal<bool>) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/token", API_PATH))
        .fetch_credentials_include()
        .send()
        .await?;
    if res.status() == StatusCode::OK {
        login_signal.set(true);
        Ok(())
    } else {
        login_signal.set(false);
        let navigate = leptos_router::use_navigate();
        navigate("/login", Default::default());
        Ok(())
    }
}
