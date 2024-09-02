use anyhow::{bail, Context as _};
use leptos::*;
use leptos_meta::Style;
use leptos_router::*;
use reqwest::StatusCode;
use serde_json::json;
use thaw::*;

use crate::{model::{ItemResponseBasic, OrderResponseBasic}, Context, API_PATH};

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

    let quantities= move || {
        if let Some(Some(s)) = res.get() {
            let s = s.items.iter().map(|item| item.quantity.to_string()).collect();
            return s;
        }
        vec![]
    };
    let quantities_val = create_rw_signal(vec![]);
    create_effect(move |_| {quantities_val.set(quantities().to_owned());});

    let names= move || {
        if let Some(Some(s)) = res.get() {
            let s = s.items.iter().map(|item| item.name.to_string()).collect();
            return s;
        }
        vec![]
    };
    let names_val= create_rw_signal(vec![]);
    create_effect(move |_| {names_val.set(names().to_owned());});

    let values= move || {
        if let Some(Some(s)) = res.get() {
            let s = s.items.iter().map(|item| item.value).collect();
            return s;
        }
        vec![]
    };
    let values_val= create_rw_signal(vec![]);
    create_effect(move |_| {values_val.set(values().to_owned());});

    move || {
        match res.get() {
        None | Some(None) => {
            view! {<Space justify=SpaceJustify::Center><Spinner /></Space>}.into_view()
        }
        Some(Some(order)) => view! {
            <Style>"
                .checkbox{
                    margin-bottom:5px;
                    border-color: #e5e8eb;
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
                                             fetch_update_safe(order.id,receiver_val,additional_val,quantities_val,names_val,values_val).await;
                                             fetch_item_check_safe(order_id,index as i32,new_value).await;
                                             res.refetch();
                                         }
                                     );
                                }
                            }></input>
                            <div class="thaw-input" style="--thaw-placeholder-color: #c2c2c2;"><input on:input=move|ev|{
                                let mut vec = quantities_val.get();
                                let d = vec.get_mut(index);
                                if let Some(val) = d{
                                    *val=event_target_value(&ev);
                                }
                                quantities_val.set(vec);
                            }
                            value=move||quantities_val.get().get(index).unwrap_or(&"".to_string()).to_string() class="thaw-input__input-el" placeholder="1kg" /></div>
                            <div class="thaw-input" style="--thaw-placeholder-color: #c2c2c2;"><input on:input=move|ev|{
                                let mut vec = names_val.get();
                                let d = vec.get_mut(index);
                                if let Some(val) = d{
                                    *val=event_target_value(&ev);
                                }

                                names_val.set(vec);
                            } value=move||names_val.get().get(index).unwrap_or(&"".to_string()).to_string() class="thaw-input__input-el" placeholder="bejca" /></div>
                            <div class="thaw-input" style="--thaw-placeholder-color: #c2c2c2;"><input on:input=move|ev|{
                                let mut vec = values_val.get();
                                let d = vec.get_mut(index);
                                if let Some(val) = d{
                                    let float_value:f32 = event_target_value(&ev).parse().unwrap_or(0f32); 
                                    let times100 = float_value * 100.0f32;
                                    *val= times100 as i32;
                                }
                                values_val.set(vec);
                            } value=move||{
                                let int_value = *values_val.get().get(index).unwrap_or(&0);
                                let float_value = int_value as f32;
                                let div100 = float_value / 100.0f32;
                                format!("{:.2}",div100)
                            }  class="thaw-input__input-el" placeholder="100zł" /></div>
                    </Space>
                </div>
            }).collect::<Vec<_>>()}
            </Space>
            <br/>
            <Button on:click=move|_|{
                spawn_local(async move{
                    fetch_update_safe(order.id,receiver_val,additional_val,quantities_val,names_val,values_val).await;
                    fetch_new_item_safe(order.id,res).await;
                });
            
            }
         block=true variant=ButtonVariant::Outlined>"Dodaj Nowy"</Button>
            <Divider/>
            <Button block=true on:click=move|_|{
                spawn_local(async move{
                    fetch_update_safe(order.id,receiver_val,additional_val,quantities_val,names_val,values_val).await;
                    let nav = use_navigate();
                    nav("/orders",Default::default());
                });
             }>"Zapisz"</Button>


            </Card>
        }
        .into_view(),
    }
    }
}


async fn fetch_new_item_safe(order_id: i32, res: Resource<(),Option<OrderResponseBasic>>){
    if let Err(e) = fetch_new_item(order_id).await{
        // use_message().create(
        //     e.to_string(),
        //     thaw::MessageVariant::Error,
        //     Default::default(),
        // );
        let n = use_navigate();
        n("/",Default::default());
    }
    res.refetch();
}

async fn fetch_new_item(order_id: i32) -> anyhow::Result<()>{
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/orders/{order_id}/items", API_PATH))
        .json(&json!({
            "quantity": "".to_owned(),
            "name": "".to_owned(),
            "value": 0,
        }))
        .fetch_credentials_include()
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        let e = res.text().await?;
        bail!(e.to_string());
    }
    Ok(())
}

  

async fn fetch_order_safe(id: i32) -> Option<OrderResponseBasic> {
    match fetch_order(id).await {
        Ok(s) => Some(s),
        Err(e) => {
            // use_message().create(
            //     e.to_string(),
            //     thaw::MessageVariant::Error,
            //     Default::default(),
            // );
            let n = use_navigate();
            n("/",Default::default());
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
        // use_message().create(
        //     e.to_string(),
        //     thaw::MessageVariant::Error,
        //     Default::default(),
        // );
        let n = use_navigate();
        n("/",Default::default());
    }
}
async fn fetch_update_safe(
    order_id: i32,
    receiver: RwSignal<String>,
    additional_info: RwSignal<String>,
    quantities: RwSignal<Vec<String>>,
    names: RwSignal<Vec<String>>,
    values: RwSignal<Vec<i32>>,
){
    if let Err(e) =  fetch_update(order_id,receiver,additional_info,quantities,names,values).await{
        // use_message().create(
        //     e.to_string(),
        //     thaw::MessageVariant::Error,
        //     Default::default(),
        // );
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

async fn fetch_update(
    order_id: i32,
    receiver: RwSignal<String>,
    additional_info: RwSignal<String>,
    quantities: RwSignal<Vec<String>>,
    names: RwSignal<Vec<String>>,
    values: RwSignal<Vec<i32>>,

)->anyhow::Result<()>{
    let client = reqwest::Client::new();
    // request for items ids
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
    let items = json.items;

    // request for updating order
    let res = client
        .patch(format!("{}/orders/{order_id}", API_PATH))
        .fetch_credentials_include()
        .json(&json!({
            "receiver": receiver.get_untracked(),
            "additional_info": additional_info.get_untracked(),
        }))
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        let e = res.text().await?;
        bail!(e.to_string());
    }

    // request for updating all items
    for (index, item) in items.iter().enumerate(){
        let item_id = item.id;
        let quantity = quantities.get_untracked().get(index).with_context(||"internal arrays error - fetch update item")?.to_string();
        let name= names.get_untracked().get(index).with_context(||"internal arrays error - fetch update item")?.to_string();
        let value= values.get_untracked().get(index).with_context(||"internal arrays error - fetch update item")?.to_owned();
        let res = client.patch(format!("{}/orders/{order_id}/items/{item_id}",API_PATH)).fetch_credentials_include().json(&json!({
            "quantity":quantity,
            "name":name,
            "value":value,
        })).send().await?;
        if res.status() != StatusCode::OK {
            let e = res.text().await?;
            bail!(e.to_string());
        }
    }
    Ok(())
}

