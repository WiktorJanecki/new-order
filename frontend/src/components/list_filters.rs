use chrono::Datelike;
use leptos::*;
use leptos_meta::*;
use thaw::*;

use crate::model::OrderResponseBasic;

#[component]
pub fn ListFilter(
    params: RwSignal<String>,
    res: Resource<RwSignal<String>, Vec<OrderResponseBasic>>,
) -> impl IntoView {
    let date_picker_start = create_rw_signal(Some(chrono::Local::now().date_naive()));
    let date_picker_end = create_rw_signal(
        chrono::Local::now()
            .date_naive()
            .checked_add_days(chrono::Days::new(1)),
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
    view! {
        <Style>"
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
        <ButtonGroup>
        <Button variant=ButtonVariant::Outlined on_click=show_today>"Dzisiaj"</Button>
        <Button variant=ButtonVariant::Outlined on_click=show_this_month>{this_month_in_polish}</Button>
        <Button variant=ButtonVariant::Outlined on_click=show_all>"Wszystko"</Button>
        <Button variant=ButtonVariant::Outlined on_click=move |_| show_filters.set(true)>"Inne"</Button>
        </ButtonGroup>
    }
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
