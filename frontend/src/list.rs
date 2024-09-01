use crate::components::{list_filters::ListFilter, order_card::OrderCard};
use anyhow::{bail, Result};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use reqwest::StatusCode;
use thaw::*;

use crate::{model::OrderResponseBasic, API_PATH};

#[component]
pub fn ListView() -> impl IntoView {
    let params = create_rw_signal(String::from(""));
    let res = create_resource(
        move || params,
        |params| async move {
            match fetch_orders(params.get_untracked()).await {
                Err(e) => {
                    use_message().create(e.to_string(), MessageVariant::Error, Default::default());
                    vec![]
                }
                Ok(s) => s,
            }
        },
    );
    let back = move |_| {
        let nav = use_navigate();
        nav("/", Default::default());
    };

    view! {
        <Style>"
            .padding{
                padding: 0px 28px;
            }
        "
        </Style>
        <div style="padding:0 30px;">
        <h1>"Zamówienia: "</h1>
        <Space>
            <ListFilter params=params res=res />
        </Space>
        <br/>
        <br/>
        <Collapse>
        {
            move || match res.get() {
                None => view!{<Space justify=SpaceJustify::Center><Spinner/></Space>}.into_view(),
                Some(s) => { s.iter().cloned().map(|order: OrderResponseBasic|{view!{
                    <CollapseItem class={if is_order_checked(&order) {"checked_text"} else {""}} key={order.id.to_string()}  title={order.receiver.to_owned()}>
                        <OrderCard order=order />
                    </CollapseItem>
                }}).collect::<Vec<_>>().into_view()},

            }
        }
        </Collapse>
        <Divider/>
        <Space justify=SpaceJustify::Center>
        <Button on_click=back>"Wróć"</Button>
        </Space>
        </div>
    }
}

async fn fetch_orders(params: String) -> Result<Vec<OrderResponseBasic>> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/orders{params}", API_PATH))
        .fetch_credentials_include()
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        let err = res.text().await?;
        bail!(err);
    }
    let vec: Vec<OrderResponseBasic> = res.json().await?;
    Ok(vec)
}

fn is_order_checked(order: &OrderResponseBasic) -> bool {
    // if all items inside are checked
    if order.items.is_empty() {
        return false;
    }
    order.items.iter().all(|x| x.checked)
}
