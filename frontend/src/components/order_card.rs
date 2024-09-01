use leptos::*;
use leptos_meta::*;
use thaw::*;

use crate::model::{ItemResponseBasic, OrderResponseBasic};

#[component]
pub fn OrderCard(order: OrderResponseBasic) -> impl IntoView {
    view! {
        <Style>"
            .inner > div{
                padding: 12px 0px !important;
            }
            .checked{
                  background: repeating-linear-gradient(
                    180deg,
                    black 0%,
                    black 100%
                  );
                  background-size: 100% 1px;
                  background-position: center;
                  background-repeat: no-repeat;
            }
            .checked_text > .thaw-collapse-item__header{
                text-decoration:line-through;
            }
        "
        </Style>
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
                   <tr class={if item.checked {"checked"} else {""}}>
                       <td>{item.quantity}</td>
                       <td>{item.name}</td>
                       <td>{format!("{}.{:02} zł",item.value/100,item.value%100)}</td>
                   </tr>
               }).collect::<Vec<_>>()}
           </Table>
           <br/>
           <div class="padding">
               {"Dodatkowe informacje: ".to_owned() + order.additional_info.unwrap_or("".to_owned()).as_ref()}
           </div>

        </Card>
    }
}
