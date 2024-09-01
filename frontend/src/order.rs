use anyhow::bail;
use leptos::*;
use leptos_router::*;
use reqwest::StatusCode;
use thaw::*;

use crate::{model::OrderResponseBasic, API_PATH};

#[derive(PartialEq, Params)]
struct OrderParams {
    id: Option<i32>,
}

#[component]
pub fn OrderView() -> impl IntoView {
    let params = use_params::<OrderParams>();
    let id =
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default());
    let res = create_resource(|| {}, move |_| fetch_order_safe(id().unwrap_or(0)));
    move || match res.get() {
        None | Some(None) => {
            view! {<Space justify=SpaceJustify::Center><Spinner /></Space>}.into_view()
        }
        Some(Some(order)) => view! {
            <h1>{order.receiver}</h1>
        }
        .into_view(),
    }
}

async fn fetch_order_safe(id: i32) -> Option<OrderResponseBasic> {
    match fetch_order(id).await {
        Ok(s) => Some(s),
        Err(e) => {
            use_message().create(
                e.to_string(),
                thaw::MessageVariant::Error,
                Default::default(),
            );
            None
        }
    }
}

async fn fetch_order(id: i32) -> anyhow::Result<OrderResponseBasic> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/orders/{id}", API_PATH))
        .fetch_credentials_include()
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        let e = res.text().await?;
        bail!(e.to_string());
    }
    let json = res.json().await?;
    Ok(json)
}
