use std::default;

use anyhow::Result;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use thaw::*;
use wasm_bindgen::JsValue;

use crate::{Context, API_PATH};

#[component]
pub fn HomeView() -> impl IntoView {
    let messages = use_message();
    let ctx = expect_context::<Context>();
    create_effect(move |_| {
        spawn_local(check_login_safe(ctx, messages));
    });
    let logout = |_| {
        let nav = use_navigate();
        nav("/login", Default::default());
    };
    let list = |_| {
        let nav = use_navigate();
        nav("/orders", Default::default());
    };
    let dashboard = |_| {
        let nav = use_navigate();
        nav("/dashboard", Default::default());
    };
    let new_order = |_| {
        spawn_local(async move {
            if let Ok(id) = fetch_create().await {
                let nav = use_navigate();
                nav(&format!("/orders/{}", id), Default::default())
            }
        });
    };
    let window_size = window()
        .inner_width()
        .unwrap_or(JsValue::from_f64(400.0))
        .as_f64()
        .unwrap_or(400.0);
    let mobile = window_size < 1000f64;
    let full = move || ctx.privileges.get() == "Full";
    view! {
        <Style>"
            html,body{
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
                min-height:calc(100% - 70px);
            }
        "</Style>
        <div style="content:''; height:30px;"></div>
        <Space class="fullheight" justify=SpaceJustify::Center align=SpaceAlign::Center>
                {if mobile{view!{
                    // on mobile
                    <Space vertical=true>
                        <Button on_click=new_order class="btn" block=true>"Nowe Zamówienie"</Button>
                        "" // adds 10px margin
                        ""
                        ""
                        <Button on_click=list class="btn" block=true>"Lista Zamówień"</Button>
                        "" // adds 10px margin
                        ""
                        ""
                        {move|| if  full() {view!{<Button on_click=dashboard class="btn" block=true>"Podsumowanie"</Button>}.into_view()}else {view!{<div style="display:none"></div>}.into_view()}}
                        <Divider/>
                        <Button on_click=logout block=true>"Wyloguj"</Button>
                    </Space>
                }.into_view()}


                else{view!{
                    // on desktop
                        {move|| if  full() {view!{
                            <Space>
                                <Button on_click=new_order class="btn" block=true>"Nowe Zamówienie"</Button>
                                <Button on_click=list class="btn" block=true>"Lista Zamówień"</Button>
                                <Button on_click=dashboard class="btn" block=true>"Podsumowanie"</Button>
                                <Button class="btn" on_click=logout block=true size=ButtonSize::Large>"Wyloguj"</Button>
                            </Space>
                        }.into_view()}else {view!{
                            <Space>
                                <Button on_click=new_order class="btn" block=true>"Nowe Zamówienie"</Button>
                                <Button on_click=list class="btn" block=true>"Lista Zamówień"</Button>
                                <Button class="btn" on_click=logout block=true size=ButtonSize::Large>"Wyloguj"</Button>
                            </Space>
                        }}.into_view()}
                }.into_view()}}
        </Space>
    }
}

async fn check_login_safe(ctx: Context, messages: MessageInjection) {
    if let Err(e) = check_login(ctx.login, ctx.privileges).await {
        messages.create(e.to_string(), MessageVariant::Error, Default::default());
    }
}

async fn check_login(login_signal: RwSignal<bool>, privileges: RwSignal<String>) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/token", API_PATH))
        .fetch_credentials_include()
        .send()
        .await?;
    if res.status() == StatusCode::OK {
        login_signal.set(true);
        #[derive(Deserialize)]
        struct Output {
            privileges: String,
        }
        let prive: Output = res.json().await?;
        privileges.set(prive.privileges);
        Ok(())
    } else {
        login_signal.set(false);
        let navigate = leptos_router::use_navigate();
        navigate("/login", Default::default());
        Ok(())
    }
}

async fn fetch_create() -> Result<i32> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/orders", API_PATH))
        .json(&json!({
            "receiver": "".to_owned(),
            "additional_info": None::<()>,
        }))
        .fetch_credentials_include()
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        let e = res.text().await?;
        anyhow::bail!(e.to_string());
    }
    #[derive(Deserialize)]
    struct Output {
        id: i32,
    }
    let json: Output = res.json().await?;
    Ok(json.id)
}
