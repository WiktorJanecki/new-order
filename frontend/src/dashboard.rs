use leptos::*;
use leptos_router::*;
use thaw::MessageVariant;
use thaw::*;

use crate::components::list_filters::ListFilter;
use crate::list::fetch_orders;
use crate::model::OrderResponseBasic;
#[component]
pub fn DashboardView() -> impl IntoView {
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
    let back = |_| {
        let nav = use_navigate();
        nav("/", Default::default());
    };
    view! {
        <Card title="Podsumowanie">
            <Space vertical=true>
                <ListFilter params=params res=res />
                <br/>
                <Space align=SpaceAlign::Center>
                <Text>"Przychód z wyfiltorwanych elementów: "</Text>
                {move||match res.get(){
                    None => view!{<Space justify=SpaceJustify::Center><Spinner /></Space>}.into_view(),
                    Some(s) => view!{{
                        let r = count_income(s);
                        let c = r/100;
                        let rem = r%100;
                        format!("{c}.{rem:02} zł")
                    }}.into_view(),
                }}
                </Space>
                <br/>
                <Space>
                    <Button>"Pobierz excela"</Button>
                    <Button>"Pobierz kopie zapasową"</Button>
                </Space>
                <Divider/>
                <Space justify=SpaceJustify::Center>
                    <Button on_click=back>"Wróć"</Button>
                </Space>
            </Space>
        </Card>
    }
}

fn count_income(vec: Vec<OrderResponseBasic>) -> i32 {
    let mut income = 0;
    for order in vec {
        for item in order.items {
            income += item.value;
        }
    }
    income
}
