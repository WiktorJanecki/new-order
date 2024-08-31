use anyhow::{bail, Result};
use chrono::{Datelike, Days};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use reqwest::StatusCode;
use thaw::*;

use crate::{
    model::{ItemResponseBasic, OrderResponseBasic},
    API_PATH,
};

#[component]
pub fn ListView() -> impl IntoView {
    let params = create_rw_signal(String::from(""));
    let date_picker_start = create_rw_signal(Some(chrono::Local::now().date_naive()));
    let date_picker_end = create_rw_signal(
        chrono::Local::now()
            .date_naive()
            .checked_add_days(Days::new(1)),
    );
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
    let show_filters = create_rw_signal(false);
    let show_today = move |_| {
        let today = chrono::Local::now().naive_local().date().to_string();
        params.set(format!("?date_start={}", today));
        res.refetch();
    };
    let show_this_month = move |_| {
        let today = chrono::Local::now().naive_local().date();
        let this_month = chrono::NaiveDate::from_ymd_opt(today.year(), today.month(), 1)
            .expect("day 1 so can't panic");
        params.set(format!("?date_start={}", this_month));
        res.refetch();
    };
    let show_all = move |_| {
        params.set("".to_owned());
        res.refetch();
    };
    let filter = move |_| {
        params.set(format!(
            "?date_start={}&date_end={}",
            date_picker_start.get_untracked().unwrap(),
            date_picker_end.get_untracked().unwrap()
        ));
        res.refetch();
        show_filters.set(false);
    };
    let back = move |_| {
        let nav = use_navigate();
        nav("/", Default::default());
    };

    view! {
        <Style>"
            * {
                box-sizing: border-box;
            }
            body,html{
                margin:0;
            }
            .stripe{
                width:100vw;
                height:40px;
                background-color:#0078ff;
                left:0;
                top:0px;
            }
            .inner > div{
                padding: 12px 0px !important;
            }
            .padding{
                padding: 0px 28px;
            }
            .modal{
                width:90vw;
                max-width:500px;
            }
        "
        </Style>
        <Modal title="Filtry" show=show_filters class="modal" width="90vw">
            <Space vertical=true>
                <Space>
                {"Od (włącznie)"}
                <DatePicker value=date_picker_start/>
                </Space>

                <Space>
                {"Do (włącznie)"}
                <DatePicker value=date_picker_end/>
                </Space>
                <Button on_click=filter block=true>"Filtruj"</Button>
            </Space>
        </Modal>
        <div class="stripe"></div>
        <div style="padding:0 30px;">
        <h1>"Zamówienia: "</h1>
        <Space>
        <ButtonGroup>
        <Button variant=ButtonVariant::Outlined on_click=show_today>"Dzisiaj"</Button>
        <Button variant=ButtonVariant::Outlined on_click=show_this_month>{||this_month_in_polish()}</Button>
        <Button variant=ButtonVariant::Outlined on_click=show_all>"Wszystko"</Button>
        <Button variant=ButtonVariant::Outlined on_click=move |_| show_filters.set(true)>"Inne"</Button>
        </ButtonGroup>
        </Space>
        <br/>
        <br/>
        <Collapse>

            {
               move|| match res.get() {
                    None => view!{<Spinner/>}.into_view(),
                    Some(s) => { s.iter().cloned().map(|order: OrderResponseBasic|{view!{
                        <CollapseItem key={order.id.to_string()}  title={order.receiver.to_owned()}>
                            <Card class="inner">
                                <div class="padding">
                                    <br/>
                                    <Space justify=SpaceJustify::SpaceBetween>
                                        {order.time_created.format("%H:%M").to_string()}
                                        {order.time_created.format("%d.%m.%Y").to_string()}
                                    </Space>
                                    <Divider />
                                    <Space justify=SpaceJustify::SpaceBetween>{"Dla: ".to_owned()}{order.receiver}</Space>
                                    <Divider />
                                    <Space justify=SpaceJustify::Center>"Zawartość"</Space>
                                    <br/>
                                </div>
                                <Table>
                                    <thead>
                                        <th>"ilość"</th>
                                        <th>"Nazwa"</th>
                                        <th>"Wartość"</th>
                                    </thead>
                                   {order.items.iter().cloned().map(|item: ItemResponseBasic|view!{
                                           <tr>
                                           <td>{item.quantity}</td>
                                           <td>
                                           {item.name}
                                           </td>
                                           <td>{format!("{}.{:02} zł",item.value/100,item.value%100)}</td>
                                           </tr>
                                   }).collect::<Vec<_>>()}
                               </Table>
                               <br/>
                               <div class="padding">
                               {"Dodatkowe informacje: ".to_owned() + order.additional_info.unwrap_or("".to_owned()).as_ref()}
                               </div>

                            </Card>
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

fn this_month_in_polish() -> &'static str {
    let month = chrono::Local::now().naive_local().date().month0() + 1; // 1-12
    match month {
        1 => "Styczeń",
        2 => "Luty",
        3 => "Marzec",
        4 => "Kwiecień",
        5 => "Maj",
        6 => "Czerwiec",
        7 => "Lipiec",
        8 => "Sierpień",
        9 => "Wrzesień",
        10 => "Październik",
        11 => "Listopad",
        12 => "Grudzień",
        _ => "Miesiąc",
    }
}
