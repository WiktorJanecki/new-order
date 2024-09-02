use dashboard::DashboardView;
use home::HomeView;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use list::ListView;
use login::LoginView;
use order::OrderView;
use thaw::*;

pub mod components;
mod dashboard;
mod home;
mod list;
mod login;
pub mod model;
mod order;

pub const API_PATH: &str = "http://localhost:3000/api";

#[derive(Copy, Clone)]
struct Context {
    login: RwSignal<bool>,
    privileges: RwSignal<String>,
}

#[component]
fn App() -> impl IntoView {
    let theme = create_rw_signal(Theme::light());
    let login = create_rw_signal(false);
    let privileges = create_rw_signal("".to_owned());

    provide_context(Context { login, privileges });

    view! {
        <Router>
            <ThemeProvider theme>
                <MessageProvider>
                    <Style>"
                        *{
                            box-sizing: border-box;
                        }
                        html,body{
                            margin:0;
                        }
                        .stripe{
                            width:100vw;
                            height:40px;
                            background-color:#0078ff;
                            left:0;
                            top:0px;
                        }
                    "</Style>
                    <div class="stripe"></div>
                    <Routes>
                        <Route path="/"           view=HomeView />
                        <Route path="/login"      view=LoginView />
                        <Route path="/orders/:id" view=OrderView />
                        <Route path="/orders"     view=ListView />
                        <Route path="/dashboard"  view=DashboardView />
                        <Route path="*any" view=||view!{<h1>"Nie znaleziono strony"</h1>}/>
                    </Routes>
                </MessageProvider>
            </ThemeProvider>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    println!("Hello, world!");
    mount_to_body(App);
}
