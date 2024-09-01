use anyhow::{bail, Context};
use leptos::*;
use leptos_meta::Style;
use leptos_router::*;
use reqwest::StatusCode;
use serde_json::json;
use thaw::*;

use crate::{model::{ItemResponseBasic, OrderResponseBasic}, API_PATH};

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

    // form
    let receiver = move || {
        if let Some(Some(s)) = res.get() {
            return s.receiver;
        }
        "".to_string()
    };
    let receiver_val = create_rw_signal("".to_string());
    create_effect(move |_| receiver_val.set(receiver()));

    let additional = move || {
        if let Some(Some(s)) = res.get() {
            return s.additional_info.unwrap_or("".to_string());
        }
        "".to_string()
    };
    let additional_val = create_rw_signal("".to_string());
    create_effect(move |_| additional_val.set(additional()));

    // vec of checkbox bools which reacts to api responses
    let checkeds = move || {
        if let Some(Some(s)) = res.get() {
            let s = s.items.iter().map(|item| item.checked).collect();
            return s;
        }
        vec![]
    };
    let checked_values: RwSignal<Vec<bool>> = create_rw_signal(vec![]);
    // binds api responses to vec of signals
    create_effect(move |_| {
        checked_values.set(checkeds().to_owned());
    });


    move || {
        match res.get() {
        None | Some(None) => {
            view! {<Space justify=SpaceJustify::Center><Spinner /></Space>}.into_view()
        }
        Some(Some(order)) => view! {
            <Style>"
                .checkbox{
                    margin-bottom:5px;
                }
                .striped{
                    position:relative;
                }
                .striped::after{
                      position: absolute;
                      left: 0;
                      top: 50%;
                      height: 1px;
                      background: BLACK;
                      content: '';
                      width: 100%;
                      display: block;
                }
            "</Style>
            <Card title={"Zamówienie nr. ".to_owned()+&order.id.to_string()}>
            <Space vertical=true>
                <Space align=SpaceAlign::Center justify=SpaceJustify::SpaceBetween>
                <Text>"Odbiorca: "</Text> <Input value=receiver_val/>
                </Space>
                <Space align=SpaceAlign::Center  justify=SpaceJustify::SpaceBetween>
                <Text>"Dopis: "</Text> <Input value=additional_val/>
                </Space>
            </Space>
            <br/>
            <p>"Przedmioty: "</p>
            <Space vertical=true>
            {order.items.iter().enumerate().map(|(index,_item)|view!{
                <div style="margin-top:5px"></div>
                <div class:striped=move||{
                    *checked_values.get().get(index).unwrap_or(&false)
                }>
                    <Space align=SpaceAlign::Center>
                            <input type="checkbox" class="checkbox" checked=move||*checked_values.get().get(index).unwrap_or(&false) on:input=move|_|{
                                let mut values = checked_values.get();
                                if let Some(v) = values.get_mut(index){
                                     let new_value: bool = !*v;       
                                     spawn_local(
                                         async move{
                                             let order_id = order.id;
                                             fetch_item_check_safe(order_id,index as i32,new_value).await;
                                             res.refetch();
                                         }
                                     );
                                }
                            }></input>
                            <Input placeholder="1kg" />
                            <Input placeholder="bejca" />
                            <Input placeholder="100zł"/>
                    </Space>
                </div>
            }).collect::<Vec<_>>()}
            </Space>
            <br/>
            <Button block=true variant=ButtonVariant::Outlined>"Dodaj Nowy"</Button>


            </Card>
        }
        .into_view(),
    }
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


async fn fetch_item_check_safe(order_id: i32, item_id: i32, value: bool){
    if let Err(e) =  fetch_item_check(order_id,item_id,value).await{
        use_message().create(
            e.to_string(),
            thaw::MessageVariant::Error,
            Default::default(),
        );
    }
}

// item_index in order
async fn fetch_item_check(order_id: i32, item_index: i32, value:bool) -> anyhow::Result<()>{
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/orders/{order_id}", API_PATH))
        .fetch_credentials_include()
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        let e = res.text().await?;
        bail!(e.to_string());
    }
    let json: OrderResponseBasic = res.json().await?;
    let item_id = json.items.get(item_index as usize).context("failed to index item")?.id;
    let res = client
        .patch(format!("{}/orders/{order_id}/items/{item_id}", API_PATH))
        .fetch_credentials_include()
        .json(&json!({
            "checked": value
        }))
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        let e = res.text().await?;
        bail!(e.to_string());
    }
    Ok(())
}

