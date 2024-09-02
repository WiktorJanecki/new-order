use anyhow::{bail, Context as _};
use icondata as i;
use leptos::*;
use leptos_meta::Style;
use leptos_router::*;
use reqwest::StatusCode;
use serde_json::json;
use thaw::*;

use crate::{
    model::{ItemResponseBasic, OrderResponseBasic},
    Context, API_PATH,
};

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

    // fetch and create editable input for receiver
    let receiver = move || {
        if let Some(Some(s)) = res.get() {
            return s.receiver;
        }
        "".to_string()
    };
    let receiver_val = create_rw_signal("".to_string());
    create_effect(move |_| receiver_val.set(receiver()));

    // for additional info
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
            return s.items.iter().map(|item| item.checked).collect();
        }
        vec![]
    };
    let checked_values: RwSignal<Vec<bool>> = create_rw_signal(vec![]);
    // binds api responses to vec of signals
    create_effect(move |_| {
        checked_values.set(checkeds().to_owned());
    });
    let update_check = move |order_id: i32, index: usize| {
        let mut values = checked_values.get();
        if let Some(v) = values.get_mut(index) {
            let new_value: bool = !*v;
            spawn_local(async move {
                fetch_update_safe(order_id, receiver_val, additional_val).await;
                fetch_item_check_safe(order_id, index as i32, new_value).await;
                res.refetch();
            });
        }
    };

    let delete_item = move |order_id: i32, item_id: i32| {
        spawn_local(async move {
            fetch_item_delete_safe(order_id, item_id).await;
            res.refetch();
        });
    };

    let new_item_quantity = create_rw_signal("".to_string());
    let new_item_name = create_rw_signal("".to_string());
    let new_item_value = create_rw_signal(0i32);
    let add_item = move |order_id: i32| {
        spawn_local(async move {
            let val = new_item_value.get_untracked();
            fetch_update_safe(order_id, receiver_val, additional_val).await;
            fetch_new_item_safe(order_id, new_item_quantity, new_item_name, val as i32).await;
            res.refetch();
        });
    };
    let delete_order = move |order_id:i32|{
        spawn_local(async move{
            fetch_order_delete_safe(order_id).await;
            let nav = use_navigate();
            nav("/orders",Default::default());  
        });
    };

    move || {
        match res.get() {
            None | Some(None) => {
                view! {<Space justify=SpaceJustify::Center><Spinner /></Space>}.into_view()
            }
            Some(Some(order)) => {
                let order_b = order.clone();
                view! {
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
                // rows
                {move||{order.items.clone().iter().cloned().enumerate().map(|(index,item)|view!{
                    <div style="margin-top:5px"></div>
                    <div class:striped=move||item.checked>
                        <Space align=SpaceAlign::Center class="inputs">
                            // checkbox
                            <input
                                type="checkbox"
                                class="checkbox"
                                checked=move||{item.checked}
                                on:click=move|_|update_check(order.id,index)
                            ></input>

                            // quantity
                            <div class="thaw-input thaw-input--disabled" style="--thaw-background-color-disabled: #fafafc;">
                                <input disabled value=move||item.quantity.to_string() class="thaw-input__input-el" placeholder="1kg" />
                            </div>

                            // name
                            <div class="thaw-input thaw-input--disabled" style="--thaw-background-color-disabled: #fafafc;">
                                <input disabled value=move||item.name.to_string() class="thaw-input__input-el" placeholder="bejca" />
                            </div>

                            // value
                            <div class="thaw-input thaw-input--disabled" style="--thaw-background-color-disabled: #fafafc;">
                                <input
                                disabled
                                value=move||format!("{:.2} zł",item.value as f32/100f32)
                                class="thaw-input__input-el"
                                placeholder="bejca" />
                            </div>

                            // delete
                            <Button on_click=move|_|delete_item(order.id,item.id) variant=ButtonVariant::Outlined>
                                <Icon width="15px" icon=i::RiDeleteBin5SystemLine/>
                            </Button>
                        </Space>
                    </div>
                }).collect::<Vec<_>>()}}
                //row for add
                <div>
                    <Space align=SpaceAlign::Center class="inputs">
                        // checkbox
                        // <input type="checkbox" class="checkbox"></input>
                        // <div style="width:20px; height:20px;content:'';"></div>
                        <Icon icon=i::AiRightOutlined width="20px" />
                        <Input value=new_item_quantity placeholder="1kg" />
                        <Input value=new_item_name placeholder="lakier" />
                        <div class="thaw-input thaw-input" style="">
                            <input 
                                on:input=move|ev|{
                                    let value = event_target_value(&ev).parse::<f32>().unwrap_or(0.0f32);
                                    let int_value = (value * 100f32) as i32;
                                    new_item_value.set(int_value);
                                }
                                class="thaw-input__input-el"
                                placeholder="100.00zł" />
                        </div>

                        // add
                        <Button on:click=move|_|add_item(order.id) color=ButtonColor::Success>
                            <Icon width="15px" icon=i::RiAddCircleSystemLine/>
                        </Button>
                    </Space>
                </div>
                </Space>
                <br/>

                <Space>
                    "Suma: "
                    {move||format!("{:.2} zł",get_order_sum_value(order_b.items.clone()) as f32 / 100.0f32)}
                </Space>
                <br/>
                <Button on_click=move|_|delete_order(order.id) color=ButtonColor::Error>"Usuń"</Button>
                // divider and back button
                <Divider/>
                <Space justify=SpaceJustify::Center>

                <Button on:click=move|_|{
                    spawn_local(async move{
                        fetch_update_safe(order.id,receiver_val,additional_val).await;
                        let nav = use_navigate();
                        nav("/orders",Default::default());
                    });
                 }>
                     "Wróć"
                 </Button>
                 </Space>


                </Card>
            }
            .into_view()
            }
        }
    }
}

async fn fetch_item_delete(order_id: i32, item_id: i32) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .delete(format!("{}/orders/{order_id}/items/{item_id}", API_PATH))
        .fetch_credentials_include()
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        let e = res.text().await?;
        bail!(e.to_string());
    }
    Ok(())
}

async fn fetch_item_delete_safe(order_id: i32, item_id: i32) {
    let _ = fetch_item_delete(order_id, item_id).await;
}

async fn fetch_new_item_safe(
    order_id: i32,
    quan: RwSignal<String>,
    name: RwSignal<String>,
    val: i32,
) {
    if let Err(e) = fetch_new_item(order_id, quan, name, val).await {
        // use_message().create(
        //     e.to_string(),
        //     thaw::MessageVariant::Error,
        //     Default::default(),
        // );
    }
}

async fn fetch_new_item(
    order_id: i32,
    quan: RwSignal<String>,
    name: RwSignal<String>,
    val: i32,
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/orders/{order_id}/items", API_PATH))
        .json(&json!({
            "quantity": quan.get_untracked(),
            "name": name.get_untracked(),
            "value": val,
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
            n("/", Default::default());
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

async fn fetch_item_check_safe(order_id: i32, item_id: i32, value: bool) {
    if let Err(e) = fetch_item_check(order_id, item_id, value).await {
        // use_message().create(
        //     e.to_string(),
        //     thaw::MessageVariant::Error,
        //     Default::default(),
        // );
        let n = use_navigate();
        n("/", Default::default());
    }
}
async fn fetch_update_safe(
    order_id: i32,
    receiver: RwSignal<String>,
    additional_info: RwSignal<String>,
) {
    if let Err(_) = fetch_update(order_id, receiver, additional_info).await {}
}

// item_index in order
async fn fetch_item_check(order_id: i32, item_index: i32, value: bool) -> anyhow::Result<()> {
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
    let item_id = json
        .items
        .get(item_index as usize)
        .context("failed to index item")?
        .id;
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
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
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
    Ok(())
}

fn get_number_from_string(s: String) -> i32 {
    let numb = s
        .chars()
        .filter(|e| e.is_ascii_digit())
        .collect::<String>()
        .parse();
    numb.unwrap_or(0)
}
fn get_order_sum_value(items: Vec<ItemResponseBasic>) -> i32 {
    items
        .iter()
        .map(|item| get_number_from_string(item.quantity.to_string()) * item.value)
        .sum()
}

async fn fetch_order_delete_safe(order_id: i32){
    let _ = fetch_order_delete(order_id).await;
}
async fn fetch_order_delete(order_id: i32) -> anyhow::Result<()>{
    let client = reqwest::Client::new();
    let res = client.delete(format!("{}/orders/{order_id}",API_PATH)).
        fetch_credentials_include()
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        let e = res.text().await?;
        bail!(e.to_string());
    }
    Ok(())    
}
